//! Convert doxygen-derived intra-doc links that still use C names into rustdoc
//! markdown links that target the renamed Rust items.
//!
//! [`doxygen_rs::transform`] turns `{@link ARKUI_ERROR_CODE_PARAM_INVALID}`
//! into `` [`ARKUI_ERROR_CODE_PARAM_INVALID`] ``. That C identifier is not
//! a Rust item: bindgen strips the enum prefix and (for result-error enums)
//! renames the type. rustdoc then reports an unresolved link.
//!
//! This module rewrites a scoped, unambiguous family — ArkUI
//! `ARKUI_ERROR_CODE_*` — as they appear in **xcomponent-sys**, to
//! `` [`ARKUI_ERROR_CODE_*`](arkui_sys::native_type::ArkUiErrorCode::*) ``.
//!
//! The rustdoc path is `arkui_sys::…`, not `crate::…`: the error type lives
//! in the `arkui-sys` dependency, not in xcomponent-sys itself. Attach this
//! rewrite only when generating xcomponent bindings so arkui-sys keeps its
//! crate-relative links (ohos-sys#123).
//!
//! `ARKUI_ERROR_CODE_NO_ERROR` is left alone: `result_error_enum` folds
//! success into `Ok(())` on `ArkUiResult`, so there is no associated constant
//! to link.
//!
//! `ARKUI_ERROR_CODE_NOT_SUPPROTED_FOR_ARKTS_NODE` is rewritten to
//! `ArkUiErrorCode::ARKTS_NODE_NOT_SUPPORTED`: the C headers use that
//! misspelled identifier, but bindgen emits the correctly spelled constant.

use bindgen::callbacks::ParseCallbacks;

/// Prefix stripped from `ArkUI_ErrorCode` variants (see `ENUM_PREFIX_MAP`).
const ARKUI_ERROR_PREFIX: &str = "ARKUI_ERROR_CODE_";

/// Cross-crate path of the renamed error-code newtype in `arkui-sys`.
const ARKUI_ERROR_TYPE: &str = "arkui_sys::native_type::ArkUiErrorCode";

/// C identifiers whose suffix is not a generated associated constant.
const ARKUI_ERROR_SKIP: &[&str] = &["NO_ERROR"];

/// C-header identifier → generated associated-constant name, when the suffix
/// after prefix strip is not itself the Rust name.
const ARKUI_ERROR_RENAME: &[(&str, &str)] =
    &[("NOT_SUPPROTED_FOR_ARKTS_NODE", "ARKTS_NODE_NOT_SUPPORTED")];

/// Bindgen callback that rewrites xcomponent-sys `ARKUI_ERROR_CODE_*` links.
///
/// Intended as an additional `parse_callbacks` entry on xcomponent binding
/// configs (bindgen appends callbacks). Also used to post-process written
/// xcomponent files so the rewrite is crate-scoped even if `process_comment`
/// is not chained.
#[derive(Debug)]
pub(crate) struct XcomponentDocLinkCb;

impl ParseCallbacks for XcomponentDocLinkCb {
    fn process_comment(&self, comment: &str) -> Option<String> {
        let rewritten = rewrite_enum_variant_links(comment);
        (rewritten != comment).then_some(rewritten)
    }
}

/// Rewrite `` [`ARKUI_ERROR_CODE_FOO`] `` to a markdown link whose target is
/// `arkui_sys::native_type::ArkUiErrorCode::FOO`. Already-targeted markdown
/// links are left unchanged so the transform is idempotent.
pub(crate) fn rewrite_enum_variant_links(comment: &str) -> String {
    let mut out = String::with_capacity(comment.len() + comment.len() / 8);
    let mut rest = comment;
    while let Some(start) = rest.find("[`") {
        out.push_str(&rest[..start]);
        rest = &rest[start + 2..];
        let Some(end) = rest.find("`]") else {
            out.push_str("[`");
            out.push_str(rest);
            return out;
        };
        let ident = &rest[..end];
        rest = &rest[end + 2..];
        if rest.starts_with('(') {
            out.push_str("[`");
            out.push_str(ident);
            out.push_str("`]");
            continue;
        }
        if let Some(variant) = arkui_error_variant(ident) {
            out.push_str("[`");
            out.push_str(ident);
            out.push_str("`](");
            out.push_str(ARKUI_ERROR_TYPE);
            out.push_str("::");
            out.push_str(variant);
            out.push(')');
            continue;
        }
        out.push_str("[`");
        out.push_str(ident);
        out.push_str("`]");
    }
    out.push_str(rest);
    out
}

/// Apply [`rewrite_enum_variant_links`] to a generated xcomponent file.
pub(crate) fn rewrite_generated_file(path: &std::path::Path) -> anyhow::Result<()> {
    let text = std::fs::read_to_string(path)?;
    let rewritten = rewrite_enum_variant_links(&text);
    if rewritten != text {
        std::fs::write(path, rewritten)?;
    }
    Ok(())
}

fn arkui_error_variant(ident: &str) -> Option<&str> {
    let variant = ident.strip_prefix(ARKUI_ERROR_PREFIX)?;
    if variant.is_empty() {
        return None;
    }
    if ARKUI_ERROR_SKIP.contains(&variant) {
        return None;
    }
    if !variant
        .chars()
        .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
    {
        return None;
    }
    Some(
        ARKUI_ERROR_RENAME
            .iter()
            .find_map(|(from, to)| (*from == variant).then_some(*to))
            .unwrap_or(variant),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rewrites_arkui_error_intra_doc_link_to_arkui_sys() {
        let input = "Returns [`ARKUI_ERROR_CODE_PARAM_INVALID`] if a parameter error occurs.";
        let out = rewrite_enum_variant_links(input);
        assert_eq!(
            out,
            "Returns [`ARKUI_ERROR_CODE_PARAM_INVALID`](arkui_sys::native_type::ArkUiErrorCode::PARAM_INVALID) if a parameter error occurs."
        );
    }

    #[test]
    fn rewrites_xcomponent_state_invalid() {
        let input = "[`ARKUI_ERROR_CODE_XCOMPONENT_STATE_INVALID`] if the node has finalized.";
        let out = rewrite_enum_variant_links(input);
        assert_eq!(
            out,
            "[`ARKUI_ERROR_CODE_XCOMPONENT_STATE_INVALID`](arkui_sys::native_type::ArkUiErrorCode::XCOMPONENT_STATE_INVALID) if the node has finalized."
        );
    }

    #[test]
    fn leaves_no_error_link_alone() {
        let input = "Returns [`ARKUI_ERROR_CODE_NO_ERROR`] if the operation is successful.";
        assert_eq!(rewrite_enum_variant_links(input), input);
    }

    #[test]
    fn rewrites_c_docs_typo_to_generated_const() {
        let input = "Returns [`ARKUI_ERROR_CODE_NOT_SUPPROTED_FOR_ARKTS_NODE`] if unsupported.";
        let out = rewrite_enum_variant_links(input);
        assert_eq!(
            out,
            "Returns [`ARKUI_ERROR_CODE_NOT_SUPPROTED_FOR_ARKTS_NODE`](arkui_sys::native_type::ArkUiErrorCode::ARKTS_NODE_NOT_SUPPORTED) if unsupported."
        );
    }

    #[test]
    fn is_idempotent() {
        let once = rewrite_enum_variant_links(
            "Returns [`ARKUI_ERROR_CODE_PARAM_INVALID`] if a parameter error occurs.",
        );
        let twice = rewrite_enum_variant_links(&once);
        assert_eq!(once, twice);
        assert!(once.contains("(arkui_sys::native_type::ArkUiErrorCode::PARAM_INVALID)"));
    }

    #[test]
    fn ignores_unrelated_intra_doc_links() {
        let input = "See [`ArkUI_NodeHandle`] and [`OH_DRAWING_ERROR_INVALID_PARAMETER`].";
        assert_eq!(rewrite_enum_variant_links(input), input);
    }

    #[test]
    fn rejects_empty_or_mixed_case_suffix() {
        assert_eq!(
            rewrite_enum_variant_links("[`ARKUI_ERROR_CODE_`]"),
            "[`ARKUI_ERROR_CODE_`]"
        );
        assert_eq!(
            rewrite_enum_variant_links("[`ARKUI_ERROR_CODE_invalid`]"),
            "[`ARKUI_ERROR_CODE_invalid`]"
        );
    }

    #[test]
    fn doxygen_link_tag_becomes_markdown_path() {
        let transformed = doxygen_rs::transform(
            "Returns {@link ARKUI_ERROR_CODE_PARAM_INVALID} if a parameter error occurs.",
        );
        assert!(
            transformed.contains("[`ARKUI_ERROR_CODE_PARAM_INVALID`]"),
            "doxygen-rs should turn {{@link}} into an intra-doc link, got {transformed:?}"
        );
        let out = rewrite_enum_variant_links(&transformed);
        assert_eq!(
            out,
            "Returns [`ARKUI_ERROR_CODE_PARAM_INVALID`](arkui_sys::native_type::ArkUiErrorCode::PARAM_INVALID) if a parameter error occurs."
        );
    }
}
