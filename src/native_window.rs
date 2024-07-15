#[link(name = "ace_ndk.z")]
#[link(name = "native_window")]
extern "C" {}

mod native_window_api10;
pub use native_window_api10::*;

mod native_window_api10_fixup;
pub use native_window_api10_fixup::NativeWindowOperation;

#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
mod api11_additions;
#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
pub use api11_additions::*;
