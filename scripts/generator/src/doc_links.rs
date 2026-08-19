//! Convert doxygen-derived intra-doc links that still use C names into rustdoc
//! markdown links that target the renamed Rust items.
//!
//! [`doxygen_rs::transform`] turns `{@link OH_DRAWING_ERROR_INVALID_PARAMETER}`
//! into `` [`OH_DRAWING_ERROR_INVALID_PARAMETER`] ``. That C identifier is not
//! a Rust item: bindgen strips the enum prefix and (for result-error enums)
//! renames the type. rustdoc then reports an unresolved link.
//!
//! This module rewrites a scoped, unambiguous family — drawing
//! `OH_DRAWING_ERROR_*` — to
//! `` [`OH_DRAWING_ERROR_*`](crate::error_code::DrawingErrorCode::*) ``.
//!
//! `OH_DRAWING_SUCCESS` is left alone: `result_error_enum` folds success into
//! `Ok(())` on `DrawingResult`, so there is no associated constant to link.

/// Prefix stripped from `OH_Drawing_ErrorCode` variants (see `ENUM_PREFIX_MAP`).
const DRAWING_ERROR_PREFIX: &str = "OH_DRAWING_ERROR_";

/// Crate-relative path of the renamed error-code newtype in `ohos-drawing-sys`.
const DRAWING_ERROR_TYPE: &str = "crate::error_code::DrawingErrorCode";

/// Rewrite `` [`OH_DRAWING_ERROR_FOO`] `` to a markdown link whose target is
/// `crate::error_code::DrawingErrorCode::FOO`. Already-targeted markdown links
/// are left unchanged so the transform is idempotent.
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
        if let Some(variant) = drawing_error_variant(ident) {
            out.push_str("[`");
            out.push_str(ident);
            out.push_str("`](");
            out.push_str(DRAWING_ERROR_TYPE);
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

fn drawing_error_variant(ident: &str) -> Option<&str> {
    let variant = ident.strip_prefix(DRAWING_ERROR_PREFIX)?;
    if variant.is_empty() {
        return None;
    }
    if !variant
        .chars()
        .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
    {
        return None;
    }
    Some(variant)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rewrites_drawing_error_intra_doc_link() {
        let input = "Returns [`OH_DRAWING_ERROR_INVALID_PARAMETER`] if font is nullptr.";
        let out = rewrite_enum_variant_links(input);
        assert_eq!(
            out,
            "Returns [`OH_DRAWING_ERROR_INVALID_PARAMETER`](crate::error_code::DrawingErrorCode::INVALID_PARAMETER) if font is nullptr."
        );
    }

    #[test]
    fn leaves_success_link_alone() {
        let input = "Returns [`OH_DRAWING_SUCCESS`] if the operation is successful.";
        assert_eq!(rewrite_enum_variant_links(input), input);
    }

    #[test]
    fn is_idempotent() {
        let once = rewrite_enum_variant_links(
            "Returns [`OH_DRAWING_ERROR_ALLOCATION_FAILED`] if no memory.",
        );
        let twice = rewrite_enum_variant_links(&once);
        assert_eq!(once, twice);
        assert!(once.contains("(crate::error_code::DrawingErrorCode::ALLOCATION_FAILED)"));
    }

    #[test]
    fn ignores_unrelated_intra_doc_links() {
        let input = "See [`OH_Drawing_Canvas`] and [`OH_AVFormat`].";
        assert_eq!(rewrite_enum_variant_links(input), input);
    }

    #[test]
    fn rejects_empty_or_mixed_case_suffix() {
        assert_eq!(
            rewrite_enum_variant_links("[`OH_DRAWING_ERROR_`]"),
            "[`OH_DRAWING_ERROR_`]"
        );
        assert_eq!(
            rewrite_enum_variant_links("[`OH_DRAWING_ERROR_invalid`]"),
            "[`OH_DRAWING_ERROR_invalid`]"
        );
    }

    #[test]
    fn doxygen_link_tag_becomes_markdown_path() {
        let transformed = doxygen_rs::transform(
            "Returns {@link OH_DRAWING_ERROR_INVALID_PARAMETER} if font is nullptr.",
        );
        assert!(
            transformed.contains("[`OH_DRAWING_ERROR_INVALID_PARAMETER`]"),
            "doxygen-rs should turn {{@link}} into an intra-doc link, got {transformed:?}"
        );
        let out = rewrite_enum_variant_links(&transformed);
        assert_eq!(
            out,
            "Returns [`OH_DRAWING_ERROR_INVALID_PARAMETER`](crate::error_code::DrawingErrorCode::INVALID_PARAMETER) if font is nullptr."
        );
    }
}
