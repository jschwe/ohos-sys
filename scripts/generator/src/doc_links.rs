//! Retargeting for intra-doc links that doxygen generated from C identifiers
//! which have no public Rust binding.
//!
//! The C headers document return values by naming the C constant, but some of
//! those constants live in a private module and are replaced by a hand-written
//! public item. Point the link at that item instead, so it resolves rather
//! than warning about a link to a private item.

/// `(C identifier, path of the public Rust item that replaces it)`.
const RETARGETS: &[(&str, &str)] = &[
    (
        "OH_NATIVEXCOMPONENT_RESULT_SUCCESS",
        "crate::XcomponentResult::SUCCESS",
    ),
    (
        "OH_NATIVEXCOMPONENT_RESULT_FAILED",
        "crate::XcomponentResult::FAILED",
    ),
    (
        "OH_NATIVEXCOMPONENT_RESULT_BAD_PARAMETER",
        "crate::XcomponentResult::BAD_PARAMETER",
    ),
];

/// Rewrite `[`C_NAME`]` links to `[`Item`](crate::path::Item)`. Only existing
/// links are retargeted; plain mentions in prose are left as they are.
pub(crate) fn retarget_doc_links(comment: String) -> String {
    RETARGETS.iter().fold(comment, |comment, (c_name, path)| {
        let link = format!("[`{c_name}`]");
        if !comment.contains(&link) {
            return comment;
        }
        let display = path.strip_prefix("crate::").unwrap_or(path);
        comment.replace(&link, &format!("[`{display}`]({path})"))
    })
}
