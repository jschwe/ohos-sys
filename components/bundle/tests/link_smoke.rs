use ohos_libbundle_sys as bundle;

#[test]
fn link_smoke_application_info() {
    unsafe {
        let _ = bundle::native_interface_bundle::OH_NativeBundle_GetCurrentApplicationInfo();
    }
}

#[cfg(feature = "api-11")]
#[test]
fn link_smoke_app_identifier() {
    unsafe {
        let _ = bundle::native_interface_bundle::OH_NativeBundle_GetAppIdentifier();
    }
}

#[cfg(feature = "api-13")]
#[test]
fn link_smoke_main_element_name() {
    unsafe {
        let _ = bundle::native_interface_bundle::OH_NativeBundle_GetMainElementName();
    }
}

#[cfg(feature = "api-14")]
#[test]
fn link_smoke_compatible_device_type() {
    unsafe {
        let _ = bundle::native_interface_bundle::OH_NativeBundle_GetCompatibleDeviceType();
    }
}

#[cfg(feature = "api-20")]
#[test]
fn link_smoke_debug_mode() {
    let mut is_debug = false;
    unsafe {
        let _ = bundle::native_interface_bundle::OH_NativeBundle_IsDebugMode(&mut is_debug);
    }
}

#[cfg(feature = "api-21")]
#[test]
fn link_smoke_ability_resource_info() {
    let mut size: usize = 0;
    let mut info_ptr: *mut bundle::ability_resource_info::OH_NativeBundle_AbilityResourceInfo =
        core::ptr::null_mut();
    let file_type = c"application/pdf";
    unsafe {
        let _ = bundle::native_interface_bundle::OH_NativeBundle_GetAbilityResourceInfo(
            file_type.as_ptr().cast_mut(),
            &mut info_ptr,
            &mut size,
        );
    }
}

#[cfg(feature = "api-21")]
#[test]
fn link_smoke_get_drawable_descriptor() {
    let mut drawable: *mut ohos_sys_opaque_types::ArkUI_DrawableDescriptor = core::ptr::null_mut();
    unsafe {
        let _ = bundle::ability_resource_info::OH_NativeBundle_GetDrawableDescriptor(
            core::ptr::null_mut(),
            &mut drawable,
        );
    }
}
