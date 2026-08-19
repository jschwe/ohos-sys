mod dir_conf;
mod doc_links;
mod enum_prefix;
mod header_conf;
mod opaque_types;

use crate::dir_conf::get_module_bindings_config;
use crate::header_conf::get_bindings_config;
use anyhow::{anyhow, bail, Context};
use bindgen::callbacks::EnumVariantValue;
use bindgen::{CodeGenAttributes, EnumVariation, Formatter};
use log::{debug, error, info, warn};
use std::fs;
use std::num::ParseIntError;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;
use thiserror::Error;

/// Parse the api version
///
/// Manually parse the API version to avoid pulling in any heavy dependencies.
fn parse_api_version(sdk_native_dir: &Path) -> anyhow::Result<u32> {
    let metadata = std::fs::read_to_string(sdk_native_dir.join("oh-uni-package.json"))
        .context("Failed to read oh-uni-package.json")?;
    let api_version = metadata
        .lines()
        .find(|line| line.contains("apiVersion"))
        .context("No apiVersion in oh-uni-package.json")?
        // Expected: `  "apiVersion": "12",`
        .split_once(":")
        .context("Invalid json?")?
        .1
        .trim()
        .trim_end_matches(",")
        .trim_matches('"')
        .parse::<u32>()
        .context("apiVersion is not an integer")?;
    Ok(api_version)
}

#[derive(Copy, Clone, Debug, PartialEq, Ord, PartialOrd, Eq)]
enum OpenHarmonyApiLevel {
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Eleven = 11,
    Twelve = 12,
    Thirteen = 13,
    Fourteen = 14,
    Fifteen = 15,
    Sixteen = 16,
    Seventeen = 17,
    Eighteen = 18,
    Nineteen = 19,
    Twenty = 20,
    TwentyOne = 21,
    TwentyTwo = 22,
    TwentyThree = 23,
}

#[derive(Error, Debug)]
enum ApiLevelParseError {
    #[error("Could not parse API level from integer: {0:?}")]
    ParseIntError(#[from] ParseIntError),
    #[error("Unknown API level {0}! Perhaps we need an update")]
    UnknownApiVersion(u32),
}

impl TryFrom<&str> for OpenHarmonyApiLevel {
    type Error = ApiLevelParseError;

    fn try_from(api_level: &str) -> Result<Self, ApiLevelParseError> {
        let num: u32 = api_level.parse()?;
        let level = match num {
            8 => OpenHarmonyApiLevel::Eight,
            9 => OpenHarmonyApiLevel::Nine,
            10 => OpenHarmonyApiLevel::Ten,
            11 => OpenHarmonyApiLevel::Eleven,
            12 => OpenHarmonyApiLevel::Twelve,
            13 => OpenHarmonyApiLevel::Thirteen,
            14 => OpenHarmonyApiLevel::Fourteen,
            15 => OpenHarmonyApiLevel::Fifteen,
            16 => OpenHarmonyApiLevel::Sixteen,
            17 => OpenHarmonyApiLevel::Seventeen,
            18 => OpenHarmonyApiLevel::Eighteen,
            19 => OpenHarmonyApiLevel::Nineteen,
            20 => OpenHarmonyApiLevel::Twenty,
            21 => OpenHarmonyApiLevel::TwentyOne,
            22 => OpenHarmonyApiLevel::TwentyTwo,
            23 => OpenHarmonyApiLevel::TwentyThree,
            other => {
                return Err(ApiLevelParseError::UnknownApiVersion(other));
            }
        };
        Ok(level)
    }
}

#[derive(Error, Debug)]
enum ParseDeprecatedError {
    #[error("Could not parse API level: {0:?}")]
    ApiLevelParseError(#[from] ApiLevelParseError),
    #[error("Failed to find @deprecated in line: {0}")]
    InvalidLine(String),
}

#[derive(Debug)]
struct DoxygenCommentCb;

#[derive(Debug, Default, PartialEq, Eq)]
struct DeprecatedInfo {
    since: Option<OpenHarmonyApiLevel>,
    note: Option<String>,
}

/// Split a trimmed string into its first whitespace-delimited token and the
/// remaining tail (trimmed, `None` if empty).
fn split_first_token(s: &str) -> (&str, Option<&str>) {
    match s.split_once(char::is_whitespace) {
        Some((head, tail)) => {
            let tail = tail.trim();
            (head, (!tail.is_empty()).then_some(tail))
        }
        None => (s, None),
    }
}

/// Parse the `@deprecated` (or post-transform `**Deprecated**`) line, returning
/// the `since` API level and any free-form trailing note on the same line.
fn parse_deprecated_line(
    line: &str,
) -> Result<(Option<OpenHarmonyApiLevel>, Option<String>), ParseDeprecatedError> {
    let trimmed = line.trim();

    if trimmed == "@deprecated" {
        return Ok((None, None));
    }

    // Post-transform: `**Deprecated** since N [trailing note]`
    if let Some((_, rhs)) = trimmed.split_once("**Deprecated** since") {
        let (level_str, trailing) = split_first_token(rhs.trim());
        let level = OpenHarmonyApiLevel::try_from(level_str)?;
        return Ok((Some(level), trailing.map(str::to_string)));
    }

    // Post-transform: `**Deprecated** <free-form>` (no since)
    if let Some((_, rhs)) = trimmed.split_once("**Deprecated**") {
        let trailing = rhs.trim();
        return Ok((None, (!trailing.is_empty()).then(|| trailing.to_string())));
    }

    // Raw doxygen form: `@deprecated...`
    let (_, rhs) = trimmed
        .split_once("@deprecated")
        .ok_or_else(|| ParseDeprecatedError::InvalidLine(line.to_string()))?;

    // Variant 1: `@deprecated(since = "XX")`
    // Note: Regex parsing might be more readable, but we want to avoid pulling in more dependencies.
    if let Some(api_level_str) = rhs
        .split_once("(since = \"")
        .or_else(|| rhs.split_once("(since=\""))
        .map(|(_, rhs)| {
            rhs.split_once("\"")
                .expect("String end delimiter not found")
                .0
        })
    {
        return Ok((Some(OpenHarmonyApiLevel::try_from(api_level_str)?), None));
    }

    // Variant 2: `@deprecated since N [trailing note]`
    if let Some((_, after_since)) = rhs.split_once("since ") {
        let (level_str, trailing) = split_first_token(after_since.trim());
        let level = OpenHarmonyApiLevel::try_from(level_str)?;
        return Ok((Some(level), trailing.map(str::to_string)));
    }

    // Variant 3: `@deprecated <free-form>` (e.g. ICU style "@deprecated ICU 68 …")
    let trailing = rhs.trim();
    if !trailing.is_empty() {
        return Ok((None, Some(trailing.to_string())));
    }

    Err(ParseDeprecatedError::InvalidLine(line.to_string()))
}

/// Walk lines starting at `start_idx + 1`, joining continuation lines into one
/// string. Stops at the next `@`-tag, the next post-transform paragraph break
/// (blank line) or the next post-transform bold marker (`**Foo:**`).
fn collect_continuation(lines: &[&str], start_idx: usize) -> String {
    let mut parts: Vec<&str> = Vec::new();
    for line in &lines[start_idx + 1..] {
        let t = line.trim();
        if t.is_empty() {
            break;
        }
        let stripped = t.trim_start_matches(|c: char| c == '*' || c.is_whitespace());
        if stripped.starts_with('@') || stripped.starts_with("**") {
            break;
        }
        parts.push(stripped);
    }
    parts.join(" ")
}

/// Strip `{@link Foo}` doxygen links down to plain `Foo`, since the resulting
/// note is rendered verbatim by rustc inside `#[deprecated(note = "…")]`.
fn strip_doxygen_links(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut rest = raw;
    while let Some(idx) = rest.find("{@") {
        out.push_str(&rest[..idx]);
        rest = &rest[idx + 2..];
        let Some(close) = rest.find('}') else {
            // Unterminated `{@…`: keep the rest verbatim.
            out.push_str("{@");
            out.push_str(rest);
            return out;
        };
        let inner = &rest[..close];
        // `inner` looks like `link Foo` or `linkplain Foo` — drop the tag word.
        let body = match inner.split_once(char::is_whitespace) {
            Some((_tag, body)) => body.trim(),
            None => inner.trim(),
        };
        out.push_str(body);
        rest = &rest[close + 1..];
    }
    out.push_str(rest);
    out
}

/// Clean up a free-form deprecation note for use inside `#[deprecated(note = "…")]`:
/// strip doxygen `{@link …}` markers, collapse whitespace runs, trim trailing
/// punctuation, escape characters that would break the Rust string literal, and
/// cap length so generated attributes stay readable.
fn normalize_note(raw: &str) -> String {
    let s = strip_doxygen_links(raw);

    let mut collapsed = String::with_capacity(s.len());
    let mut prev_ws = false;
    for c in s.chars() {
        if c.is_whitespace() {
            if !prev_ws && !collapsed.is_empty() {
                collapsed.push(' ');
            }
            prev_ws = true;
        } else {
            collapsed.push(c);
            prev_ws = false;
        }
    }
    let trimmed = collapsed
        .trim()
        .trim_end_matches(|c: char| c == '.' || c == ',' || c == ';');

    let mut escaped = String::with_capacity(trimmed.len());
    for c in trimmed.chars() {
        if c == '\\' || c == '"' {
            escaped.push('\\');
        }
        escaped.push(c);
    }

    const MAX_CHARS: usize = 200;
    if escaped.chars().count() <= MAX_CHARS {
        return escaped;
    }
    // Walk to the last char boundary <= MAX_CHARS, then back up to a space.
    let mut byte_end = escaped.len();
    for (i, _) in escaped
        .char_indices()
        .enumerate()
        .take_while(|(i, _)| *i < MAX_CHARS)
    {
        byte_end = i;
    }
    // `byte_end` is now the byte index of the (MAX_CHARS - 1)th char — advance
    // past it to include that char.
    byte_end = escaped[byte_end..]
        .char_indices()
        .nth(1)
        .map(|(off, _)| byte_end + off)
        .unwrap_or(escaped.len());
    let prefix = &escaped[..byte_end];
    let truncated = prefix.rsplit_once(' ').map(|(p, _)| p).unwrap_or(prefix);
    let mut capped = truncated
        .trim_end_matches(|c: char| c == '.' || c == ',' || c == ';')
        .to_string();
    capped.push('…');
    capped
}

/// Extract `since` and `note` from a doxygen comment block.
///
/// Returns `None` if the comment has no `@deprecated` (or post-transform
/// `**Deprecated**`) marker. The comment may arrive in raw doxygen form with
/// `@deprecated` / `@useinstead` tags, or already post-`doxygen_rs::transform`
/// form with `**Deprecated** since` / `**Use instead:**` markers. Both shapes
/// are handled.
fn parse_deprecated_info(comment: &str) -> Result<Option<DeprecatedInfo>, ParseDeprecatedError> {
    let lines: Vec<&str> = comment.lines().collect();
    let Some(dep_idx) = lines
        .iter()
        .position(|line| line.contains("@deprecated") || line.contains("**Deprecated**"))
    else {
        return Ok(None);
    };

    let (since, inline_note) = parse_deprecated_line(lines[dep_idx])?;

    // Look for an `@useinstead` body (raw) or `**Use instead:**` paragraph
    // (post-transform) somewhere after the `@deprecated` line. Either one wins
    // over any inline trailing text on the `@deprecated` line.
    let mut useinstead_note: Option<String> = None;
    for (j, line) in lines.iter().enumerate().skip(dep_idx + 1) {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("@useinstead") {
            let head = rest.trim();
            let mut body = head.to_string();
            let cont = collect_continuation(&lines, j);
            if !cont.is_empty() {
                if !body.is_empty() {
                    body.push(' ');
                }
                body.push_str(&cont);
            }
            useinstead_note = Some(body);
            break;
        }
        if let Some(rest) = trimmed.strip_prefix("**Use instead:**") {
            let head = rest.trim();
            let mut body = head.to_string();
            let cont = collect_continuation(&lines, j);
            if !cont.is_empty() {
                if !body.is_empty() {
                    body.push(' ');
                }
                body.push_str(&cont);
            }
            useinstead_note = Some(body);
            break;
        }
    }

    let note = if let Some(raw) = useinstead_note {
        let n = normalize_note(&raw);
        (!n.is_empty()).then(|| format!("Use instead: {n}"))
    } else {
        inline_note
            .map(|n| normalize_note(&n))
            .filter(|s| !s.is_empty())
    };
    Ok(Some(DeprecatedInfo { since, note }))
}

impl bindgen::callbacks::ParseCallbacks for DoxygenCommentCb {
    fn result_error_enum_name(&self, original_enum_name: &str) -> Option<String> {
        original_enum_name
            .strip_suffix("Result")
            .map(|base| format!("{}ErrorCode", base.trim_end_matches("_")))
    }

    fn enum_variant_name(
        &self,
        enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: EnumVariantValue,
    ) -> Option<String> {
        let enum_name = enum_name?.trim_start_matches("enum ");
        enum_prefix::ENUM_PREFIX_MAP
            .get(enum_name)
            .and_then(|prefix| original_variant_name.strip_prefix(prefix))
            .map(|stripped| stripped.to_string())
    }

    fn process_comment(&self, comment: &str) -> Option<String> {
        if comment.starts_with(" < ") {
            // The leading space breaks the ///< detection of clang-sys.
            warn!(
                "Invalid doxygen comment. Should apply to item on left, but malformed: `{comment}`"
            );
            return None;
        }
        // Replace manual linebreaks in doxygen with double linebreaks for markdown.
        let comment = comment.replace("\\n", "\n");
        Some(doxygen_rs::transform(comment.trim_end_matches("\n")))
    }

    fn parse_comments_for_attributes(&self, comment: &str) -> Vec<CodeGenAttributes> {
        let mut attributes: Vec<CodeGenAttributes> = vec![];
        let api_version = comment
            .lines()
            // TODO: Investigate why some comments appear to have already been processed!
            .find_map(|line| {
                line.split_once("@since")
                    .or_else(|| line.split_once("Available since API-level: "))
            })
            .map(|(_, since)| {
                let api_level_str = since.trim();
                let api_level: Result<OpenHarmonyApiLevel, _> = api_level_str
                    .try_into()
                    .inspect_err(|err| error!("Failed to parse OH API version: {:?}", err));
                api_level.expect("Failed to parse OH API version")
            });
        if let Some(api_version) = api_version {
            let cfg = format!("feature = \"api-{}\"", api_version as u32);
            // Our Minimum api-level is 10, so we don't feature guard things <= API level 10.
            if api_version > OpenHarmonyApiLevel::Ten {
                attributes.push(CodeGenAttributes::Cfg(cfg));
                attributes.push(CodeGenAttributes::CfgAttr(format!(
                    "docsrs, doc(cfg(feature = \"api-{}\"))",
                    api_version as u32
                )));
            }
        }

        if let Some(info) = parse_deprecated_info(comment).expect("Parse failed") {
            let payload = match (info.since, info.note) {
                (Some(s), Some(n)) => Some(format!("since = \"{}\", note = \"{}\"", s as u32, n)),
                (Some(s), None) => Some(format!("since = \"{}\"", s as u32)),
                (None, Some(n)) => Some(format!("note = \"{}\"", n)),
                (None, None) => None,
            };
            attributes.push(CodeGenAttributes::Deprecated(payload));
        }

        attributes
    }
}

fn base_bindgen_builder(sysroot_dir: &Path) -> anyhow::Result<bindgen::Builder> {
    let builder = bindgen::builder()
        .use_core()
        .layout_tests(false)
        .formatter(Formatter::None)
        .merge_extern_blocks(true)
        .allowlist_recursively(false)
        .rust_target(bindgen::RustTarget::from_str("1.78").expect("invalid rust target"))
        .generate_cstr(true)
        .blocklist_file(r".*stdint\.h")
        .blocklist_file(r".*stddef\.h")
        .blocklist_file(r".*stdarg\.h")
        .blocklist_file(r".*stdbool\.h")
        .blocklist_file(r".*stdbool\.h")
        .blocklist_file(r".*/std[a-z]{3,4}\.h")
        .blocklist_file(r".*/__std[a-z_]+\.h")
        .blocklist_item("__(BYTE_ORDER|LONG_MAX|LITTLE_ENDIAN|BIG_ENDIAN|USE_TIME_BITS64)")
        .blocklist_item("u?intmax_t")
        .blocklist_var("_REDIR_TIME64") // Might get generated in arm32 builds
        .raw_line("#![allow(non_upper_case_globals)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_snake_case)]")
        .parse_callbacks(Box::new(DoxygenCommentCb))
        .clang_arg(format!("--sysroot={}",
                           sysroot_dir.to_str().context("The OpenHarmony SDK directory must be encodable as utf-8")?)
        )
        .default_enum_style(EnumVariation::NewType {
            is_bitfield: false,
            is_global: false,
            is_result_type: false,
        })
        // TODO: How to detect / deal with target specific bindings - Could this be a problem?
        .clang_arg("--target=armv7-linux-ohos")
        // dynamic_library_name()
        // .prepend_enum_name()
        ;

    Ok(builder)
}

struct DirBindingsConf {
    /// Directory the bindings should be generated for relative to the sysroot
    directory: String,
    /// Directory the bindings should be written to.
    output_dir: String,
    /// Optionally transform the output file name stem
    rename_output_file: Option<Box<dyn Fn(&str) -> String>>,
    /// Options which apply to all or most files
    set_builder_opts: Box<dyn Fn(&str, &Path, bindgen::Builder) -> bindgen::Builder>,
    /// Optionally skip the following headers
    skip_files: Vec<String>,
    /// Known nested include directories, relative to the SDK include directory.
    known_nested_include_dirs: Vec<String>,
}

impl Default for DirBindingsConf {
    fn default() -> Self {
        DirBindingsConf {
            directory: "".to_string(),
            output_dir: "".to_string(),
            rename_output_file: None,
            set_builder_opts: Box::new(|_, _, builder| builder),
            skip_files: vec![],
            known_nested_include_dirs: Default::default(),
        }
    }
}

struct BindingConf {
    include_filename: String,
    /// Will be interpolated as "${ROOT_DIR}/${output_prefix}_ffi"
    output_prefix: String,
    set_builder_opts: Box<dyn FnOnce(bindgen::Builder) -> bindgen::Builder>,
}

/// Generate bindings for the helper crate with the opaque type definitions.
fn generate_opaque_types_bindings(
    root_dir: &Path,
    builder: bindgen::Builder,
    _sysroot_include_dir: &Path,
) -> anyhow::Result<()> {
    let mut builder = builder
        .header(
            root_dir
                .join("components/opaque-types/wrapper.h")
                .to_str()
                .unwrap(),
        )
        .ignore_functions()
        .ignore_methods()
        .derive_debug(false)
        .derive_copy(false)
        .generate_comments(true)
        .clang_args(&["-x", "c++"]);
    for ty_name in opaque_types::OPAQUE_TYPES {
        builder = builder.allowlist_type(ty_name)
    }
    let binding = builder.generate().context("Bindgen failed")?;
    binding
        .write_to_file(root_dir.join("components/opaque-types/src/opaque_types.rs"))
        .context("Failed to write bindings to file")?;
    Ok(())
}

fn generate_bindings(sdk_native_dir: &Path, api_version: u32) -> anyhow::Result<()> {
    let base_builder = base_bindgen_builder(&sdk_native_dir.join("sysroot"))?;
    let base_builder = base_builder
        .clang_arg("-D")
        .clang_arg(format!("BINDGEN_OHOS_API_LEVEL={api_version}"));
    let sysroot_include_dir = sdk_native_dir.join("sysroot/usr/include");
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let root_dir = manifest_dir
        .parent()
        .and_then(|d| d.parent())
        .ok_or(anyhow!("Could not determine root directory"))?
        .canonicalize()
        .context("Could not canonicalize root directory")?;

    generate_opaque_types_bindings(&root_dir, base_builder.clone(), &sysroot_include_dir)?;

    let mut base_builder = base_builder;
    for ty_name in opaque_types::OPAQUE_TYPES {
        base_builder = base_builder.blocklist_type(ty_name)
    }

    let only_gen_module = std::env::var("ONLY_MODULE").ok();

    for binding in get_bindings_config(api_version) {
        let header_filename = sysroot_include_dir.join(&binding.include_filename);
        let header_filename_str = header_filename.to_str().context("Unicode")?;
        if only_gen_module
            .as_ref()
            .and_then(|name| header_filename_str.contains(name).then_some(()))
            .is_some()
        {
            continue;
        }
        debug!("Generating binding: {}", binding.include_filename);
        let builder = base_builder.clone().header(header_filename_str);
        // Transitively pulled in.
        let builder = builder.blocklist_file(r".*/info/application_target_sdk_version\.h");
        let builder = (binding.set_builder_opts)(builder);
        let bindings = builder.generate().context("Bindgen failed")?;

        let out_path = root_dir.join(format!("{}_ffi.rs", binding.output_prefix));
        bindings
            .write_to_file(&out_path)
            .context("Failed to write bindings to file")?;
        rewrite_xcomponent_doc_links_if_needed(&out_path)?;
    }

    for binding in &get_module_bindings_config() {
        let module_dir = sysroot_include_dir.join(&binding.directory);
        if let Some(pattern) = &only_gen_module {
            if !module_dir.to_str().unwrap().contains(pattern) {
                continue;
            }
        }
        debug!("Generating binding: {}", binding.directory);
        if !module_dir.exists() {
            bail!(
                "Could not find directory {} at {}",
                binding.directory,
                module_dir.display()
            );
        }
        let paths = fs::read_dir(module_dir)?;
        'outer: for file in paths {
            let file = file.context("Failed to enumerate dir")?;
            if file.file_type()?.is_dir() {
                let nested_dir = file.path();
                let relative_nested_dir = nested_dir
                    .strip_prefix(&sysroot_include_dir)
                    .context("Nested include directory is not inside the SDK include directory")?;
                let relative_nested_dir = relative_nested_dir.to_str().context("Unicode")?;
                if binding
                    .known_nested_include_dirs
                    .iter()
                    .any(|known| known == relative_nested_dir)
                {
                    debug!(
                        "Skipping known nested include directory {}",
                        relative_nested_dir
                    );
                    continue;
                }
                bail!(
                    "Unknown nested include directory {} under module {}",
                    relative_nested_dir,
                    binding.directory
                );
            }
            let file_path = file
                .path()
                .canonicalize()
                .context("Failed to canonicalize path")?;
            let header_filename_str = file_path.to_str().context("Unicode")?;
            let file_stem = file_path
                .file_stem()
                .ok_or(anyhow!("Failed to get filestem"))?
                .to_str()
                .context("Unicode")?;
            for skip_header in &binding.skip_files {
                if header_filename_str.contains(skip_header) {
                    eprintln!("Skipping {header_filename_str}");
                    continue 'outer;
                }
            }
            let file_stem = binding
                .rename_output_file
                .as_ref()
                .map(|rename| rename(file_stem))
                .unwrap_or(file_stem.to_string());
            let builder = base_builder
                .clone()
                .header(header_filename_str)
                .allowlist_file(header_filename_str);
            let builder = (binding.set_builder_opts)(&file_stem, file_path.as_path(), builder);

            let bindings = builder.generate().context("Bindgen failed")?;
            let base_path = root_dir.join(&binding.output_dir).join(&file_stem);
            if !base_path.exists() {
                info!(
                    "Creating target directory for bindings: {}",
                    base_path.display()
                );
                fs::create_dir_all(&base_path)
                    .context("Failed to create target directory for bindings")?;
            }

            let out_path = base_path.join(format!("{file_stem}_ffi.rs"));
            bindings
                .write_to_file(&out_path)
                .context("Failed to write bindings to file")?;
            rewrite_xcomponent_doc_links_if_needed(&out_path)?;
        }
    }

    Ok(())
}

/// Rewrite `ARKUI_ERROR_CODE_*` intra-doc links in generated xcomponent files
/// to `arkui_sys::native_type::ArkUiErrorCode::*`. No-op for other crates.
fn rewrite_xcomponent_doc_links_if_needed(path: &Path) -> anyhow::Result<()> {
    let is_xcomponent = path.components().any(|c| c.as_os_str() == "xcomponent");
    if is_xcomponent {
        doc_links::rewrite_generated_file(path).with_context(|| {
            format!(
                "Failed to rewrite xcomponent doc links in {}",
                path.display()
            )
        })?;
    }
    Ok(())
}

/// Run `cargo +nightly fmt` against the workspace at `root_dir` until it
/// reaches a fixed point.
///
/// The unstable `normalize_comments` / `normalize_doc_attributes` features
/// enabled in `.rustfmt.toml` are not idempotent: rustfmt can leave stray blank
/// `///` and `////` markers behind on the first pass and only normalise them on
/// a subsequent pass. We run fmt, then `fmt -- --check`, and re-run fmt until
/// `--check` reports no further changes.
fn run_cargo_fmt(root_dir: &Path) -> anyhow::Result<()> {
    const MAX_PASSES: usize = 4;
    for pass in 1..=MAX_PASSES {
        info!("Running cargo +nightly fmt (pass {pass})");
        let status = Command::new("cargo")
            .current_dir(root_dir)
            .args(["+nightly", "fmt"])
            .status()
            .context("Failed to spawn `cargo +nightly fmt`")?;
        if !status.success() {
            bail!("`cargo +nightly fmt` failed with status {status}");
        }
        let check = Command::new("cargo")
            .current_dir(root_dir)
            .args(["+nightly", "fmt", "--", "--check"])
            .output()
            .context("Failed to spawn `cargo +nightly fmt -- --check`")?;
        if check.status.success() {
            return Ok(());
        }
        debug!("cargo +nightly fmt has not converged yet — re-running");
    }
    bail!(
        "`cargo +nightly fmt` did not converge after {MAX_PASSES} passes — \
         likely a rustfmt bug interacting with the unstable comment-normalisation \
         options in `.rustfmt.toml`. Inspect the latest `--check` output."
    );
}

/// Apply every `*.patch` file in `patches_dir` (sorted by filename) against
/// `root_dir` via `git apply`.
///
/// Patches exist to fix specific bindgen output that we cannot express through
/// the generator config — e.g. C headers with malformed doxygen comments where
/// the comment ends up attached to the wrong member.
///
/// Convention: each patch is a unified diff with paths relative to the repo
/// root and a `--- a/PATH` / `+++ b/PATH` header. Generate one with:
///
/// ```sh
/// REL=path/to/file.rs
/// cp "$REL" /tmp/before.rs
/// # ... hand-edit "$REL" into the desired state ...
/// diff -u --label "a/$REL" --label "b/$REL" /tmp/before.rs "$REL" \
///     > scripts/generator/patches/<descriptive-name>.patch
/// ```
fn apply_patches(root_dir: &Path, patches_dir: &Path) -> anyhow::Result<()> {
    if !patches_dir.is_dir() {
        debug!(
            "No patches directory at {}, skipping.",
            patches_dir.display()
        );
        return Ok(());
    }
    let mut patches: Vec<PathBuf> = fs::read_dir(patches_dir)
        .with_context(|| format!("Failed to read {}", patches_dir.display()))?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("patch"))
        .collect();
    patches.sort();

    for patch in patches {
        let name = patch.file_name().unwrap().to_string_lossy().into_owned();

        // Idempotency: if the patch already applies in reverse, the file is
        // already in the patched state (e.g. ONLY_MODULE skipped regenerating
        // it). Skip rather than fail.
        let reverse_check = Command::new("git")
            .current_dir(root_dir)
            .args(["apply", "-R", "--check"])
            .arg(&patch)
            .output()
            .with_context(|| format!("Failed to spawn `git apply -R --check` for {name}"))?;
        if reverse_check.status.success() {
            info!("Patch already applied, skipping: {name}");
            continue;
        }

        info!("Applying patch: {name}");
        let output = Command::new("git")
            .current_dir(root_dir)
            .args(["apply", "--whitespace=nowarn"])
            .arg(&patch)
            .output()
            .with_context(|| format!("Failed to spawn `git apply` for {name}"))?;
        if !output.status.success() {
            bail!(
                "Failed to apply patch {name}.\nstderr:\n{}\n\
                 Likely the bindgen output drifted from the patch context. \
                 Regenerate the patch — see the doc comment on `apply_patches`.",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let sdk_native_dir = std::env::var_os("OHOS_SDK_NATIVE").context(
        "Please set environment variable OHOS_SDK_NATIVE to the `native` \
        directory of the OpenHarmony SDK",
    )?;
    let sdk_native_dir = PathBuf::from(sdk_native_dir);
    if !sdk_native_dir.is_dir() {
        return Err(anyhow!(
            "OHOS_SDK_NATIVE: {:?} is not a directory",
            sdk_native_dir
        ));
    }

    let libclang_path = sdk_native_dir.join("llvm/lib");
    let clang_path = sdk_native_dir.join("llvm/bin/clang");
    let api_version = parse_api_version(&sdk_native_dir)?;
    debug!(
        "Libclang: {}, clang: {}",
        libclang_path.display(),
        clang_path.display()
    );
    std::env::set_var("LIBCLANG_PATH", libclang_path);
    std::env::set_var("CLANG_PATH", clang_path);
    generate_bindings(&sdk_native_dir, api_version)?;

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root_dir = manifest_dir
        .parent()
        .and_then(|d| d.parent())
        .ok_or(anyhow!("Could not determine root directory"))?
        .canonicalize()
        .context("Could not canonicalize root directory")?;
    run_cargo_fmt(&root_dir)?;
    apply_patches(&root_dir, &manifest_dir.join("patches"))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(comment: &str) -> Option<DeprecatedInfo> {
        parse_deprecated_info(comment).expect("parse failed")
    }

    #[test]
    fn no_deprecated_returns_none() {
        assert_eq!(parse("Just a regular comment.\n@since 10"), None);
    }

    #[test]
    fn bare_deprecated_keeps_since_and_note_empty() {
        let info = parse("@deprecated").expect("has deprecated");
        assert_eq!(info.since, None);
        assert_eq!(info.note, None);
    }

    #[test]
    fn parses_since_only() {
        let info = parse(" @deprecated since 13\n @since 11").expect("has deprecated");
        assert_eq!(info.since, Some(OpenHarmonyApiLevel::Thirteen));
        assert_eq!(info.note, None);
    }

    #[test]
    fn parses_since_and_useinstead() {
        let info =
            parse(" @deprecated since 13\n @useinstead OH_NetConn_RegisterDnsResolver\n @since 11")
                .expect("has deprecated");
        assert_eq!(info.since, Some(OpenHarmonyApiLevel::Thirteen));
        assert_eq!(
            info.note.as_deref(),
            Some("Use instead: OH_NetConn_RegisterDnsResolver")
        );
    }

    #[test]
    fn useinstead_strips_trailing_period() {
        let info = parse(
            " @deprecated since 12\n @useinstead OH_AVScreenCapture in native interface.\n @since 10",
        )
        .expect("has deprecated");
        assert_eq!(info.since, Some(OpenHarmonyApiLevel::Twelve));
        assert_eq!(
            info.note.as_deref(),
            Some("Use instead: OH_AVScreenCapture in native interface")
        );
    }

    #[test]
    fn useinstead_strips_doxygen_link() {
        let info = parse(
            " @deprecated since 11\n @useinstead {@link OH_NN_UNAVAILABLE_DEVICE}\n @since 9",
        )
        .expect("has deprecated");
        assert_eq!(
            info.note.as_deref(),
            Some("Use instead: OH_NN_UNAVAILABLE_DEVICE")
        );
    }

    #[test]
    fn useinstead_multiline_is_joined_with_spaces() {
        let info = parse(
            " @deprecated since 20\n \
             @useinstead Set the callback functions separately using OH_AudioStreamBuilder_SetRendererWriteDataCallback,\n \
             OH_AudioStreamBuilder_SetRendererInterruptCallback, OH_AudioStreamBuilder_SetRendererOutputDeviceChangeCallback\n \
             and OH_AudioStreamBuilder_SetRendererErrorCallback.\n \
             @since 10",
        )
        .expect("has deprecated");
        assert_eq!(info.since, Some(OpenHarmonyApiLevel::Twenty));
        let note = info.note.expect("note present");
        assert!(note.starts_with("Use instead: Set the callback functions separately using"));
        assert!(note.contains("OH_AudioStreamBuilder_SetRendererInterruptCallback"));
        assert!(!note.contains('\n'));
    }

    #[test]
    fn deprecated_with_inline_trailing_note() {
        let info =
            parse("@deprecated since 13 use OH_NetConn_RegisterDnsResolver instead\n@since 11")
                .expect("has deprecated");
        assert_eq!(info.since, Some(OpenHarmonyApiLevel::Thirteen));
        assert_eq!(
            info.note.as_deref(),
            Some("use OH_NetConn_RegisterDnsResolver instead")
        );
    }

    #[test]
    fn useinstead_wins_over_inline_trailing_note() {
        let info = parse(
            "@deprecated since 13 ignore me\n@useinstead OH_NetConn_RegisterDnsResolver\n@since 11",
        )
        .expect("has deprecated");
        assert_eq!(
            info.note.as_deref(),
            Some("Use instead: OH_NetConn_RegisterDnsResolver")
        );
    }

    #[test]
    fn post_transform_deprecated_with_use_instead() {
        let info = parse(
            "\n**Deprecated** since 20\n\n**Use instead:** OH_AudioStreamBuilder_SetRendererWriteDataCallback\n\nAvailable since API-level: 10",
        )
        .expect("has deprecated");
        assert_eq!(info.since, Some(OpenHarmonyApiLevel::Twenty));
        assert_eq!(
            info.note.as_deref(),
            Some("Use instead: OH_AudioStreamBuilder_SetRendererWriteDataCallback")
        );
    }

    #[test]
    fn post_transform_deprecated_since_only() {
        let info = parse("\n**Deprecated** since 20\n\nAvailable since API-level: 10")
            .expect("has deprecated");
        assert_eq!(info.since, Some(OpenHarmonyApiLevel::Twenty));
        assert_eq!(info.note, None);
    }

    #[test]
    fn legacy_paren_since_form() {
        let info = parse(" @deprecated(since = \"10\")\n").expect("has deprecated");
        assert_eq!(info.since, Some(OpenHarmonyApiLevel::Ten));
        assert_eq!(info.note, None);
    }

    #[test]
    fn note_escapes_quotes_and_backslashes() {
        // Raw note containing characters that would otherwise break a Rust string literal.
        let info = parse("@deprecated since 13\n@useinstead use \\path\\to\\Foo or \"OH_Foo\"")
            .expect("has deprecated");
        let note = info.note.expect("note present");
        assert!(!note.contains("\\p") || note.contains("\\\\path"));
        assert!(!note.contains("\"OH_Foo\"") || note.contains("\\\"OH_Foo\\\""));
    }
}
