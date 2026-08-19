// Native XComponent bindings
//
// ## When to Use
//
// NativeXComponent provides an instance for the <XComponent> at the native layer, which can be
// used as a bridge for binding with the <XComponent> at the JS layer. The NDK APIs provided by the
// <XComponent> depend on this instance. The provided APIs include those for obtaining a native
// window, obtaining the layout or event information of the <XComponent>, registering the lifecycle
// callbacks of the <XComponent>, and registering the callbacks for the touch, mouse, and key events
// of the <XComponent>. You can use the provided APIs in the following scenarios:
//
//  * Register the lifecycle and event callbacks of the <XComponent>.
//  * Initialize the environment, obtain the current state, and respond to various events via these callbacks.
//  * Use the native window and EGL APIs to develop custom drawing content, and apply for and submit buffers to the graphics queue.
//! Source:
//!
//! [English Documentation](https://docs.openharmony.cn/pages/v5.0/en/application-dev/ui/napi-xcomponent-guidelines.md)
//! [Chinese Documentation](https://docs.openharmony.cn/pages/v5.0/zh-cn/application-dev/ui/napi-xcomponent-guidelines.md)
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(target_env = "ohos")]
#[link(name = "ace_ndk.z")]
extern "C" {}

mod xcomponent_ffi;
pub use xcomponent_ffi::*;

#[cfg(feature = "arkui")]
mod xcomponent_arkui_ffi;
#[cfg(feature = "arkui")]
pub use xcomponent_arkui_ffi::*;

mod xcomponent_result_ffi;

/// Enumerates the API access states.
///
/// Available since API 8.
#[repr(transparent)]
pub struct XcomponentResult(pub std::ffi::c_int);

impl XcomponentResult {
    pub const SUCCESS: Self = Self(0);
    pub const FAILED: Self = Self(-1);
    pub const BAD_PARAMETER: Self = Self(-2);
}

// assert that our handwritten binding matches the size of the generated binding.
// needs to be updated when regenerating the bindings, since the bindgen type name
// may change.
#[allow(dead_code)]
const ASSERT_SIZE_OK: () =
    assert!(size_of::<XcomponentResult>() == size_of::<xcomponent_result_ffi::_bindgen_ty_14>());

#[cfg(feature = "keyboard-types")]
#[cfg_attr(docsrs, doc(cfg(feature = "keyboard-types")))]
pub mod keyboard_types_compat;
