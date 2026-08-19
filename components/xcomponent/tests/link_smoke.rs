// The referenced symbols only resolve against the OpenHarmony SDK sysroot.
#![cfg(target_env = "ohos")]

use std::ptr;

use xcomponent_sys as xcomponent;

#[test]
fn link_smoke() {
    unsafe {
        let _ = xcomponent::OH_NativeXComponent_GetXComponentId(
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
        );
    }

    #[cfg(feature = "arkui")]
    unsafe {
        let _ =
            xcomponent::OH_NativeXComponent_AttachNativeRootNode(ptr::null_mut(), ptr::null_mut());
    }
}
