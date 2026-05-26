use crate::dir_conf::ResultEnumParseCallbacks;
use crate::BindingConf;
use bindgen::EnumVariation;

pub(crate) fn get_bindings_config(_api_version: u32) -> Vec<BindingConf> {
    vec![
        BindingConf {
            include_filename: "deviceinfo.h".to_string(),
            output_prefix: "components/deviceinfo/src/deviceinfo".to_string(),
            set_builder_opts: Box::new(|builder| builder),
        },
        BindingConf {
            include_filename: "syscap_ndk.h".to_string(),
            output_prefix: "src/syscap/syscap".to_string(),
            set_builder_opts: Box::new(|builder| builder),
        },
        BindingConf {
            include_filename: "hilog/log.h".to_string(),
            output_prefix: "components/hilog/src/hilog".to_string(),
            set_builder_opts: Box::new(|builder| 
                builder
                    .blocklist_var("LOG_DOMAIN")
                    // blocklist because of va_list.
                    .blocklist_function("OH_LOG_VPrint")
            )},
        BindingConf {
            include_filename: "napi/native_api.h".to_string(),
            output_prefix: "src/napi/napi".to_string(),
            set_builder_opts: Box::new(|builder| {
                builder
                    .prepend_enum_name(false)
                    .no_copy("napi_property_descriptor")
                    .no_copy("napi_extended_error_info")
                    .no_copy("napi_node_version")
                    .no_copy("napi_module")
                    // `NAPI_AUTO_LENGTH` is `#define NAPI_AUTO_LENGTH SIZE_MAX`. 
                    // Usages expect `size_t`.
                    .blocklist_var("NAPI_AUTO_LENGTH")
                    .raw_line("pub const NAPI_AUTO_LENGTH: usize = usize::MAX;")
                    .raw_line("pub use ohos_sys_opaque_types::{napi_env, napi_value};")
            }),
        },
        BindingConf {
            include_filename: "hitrace/trace.h".to_string(),
            output_prefix: "components/hitrace/src/hitrace".to_string(),
            set_builder_opts: Box::new(|builder| {
                builder
                    .bitfield_enum("^HiTrace_Flag$")
                    .rustified_non_exhaustive_enum("HiTrace_Tracepoint_Type")
                    .blocklist_var("LOG_DOMAIN")
                    .clang_arg("-include")
                    .clang_arg("stdbool.h")
            }),
        },
        BindingConf {
            include_filename: "ace/xcomponent/native_interface_xcomponent.h".to_string(),
            output_prefix: "components/xcomponent/src/xcomponent".to_string(),
            set_builder_opts: Box::new(|builder| {
                builder
                    .allowlist_file(r".*/xcomponent/native_.*xcomponent.*\.h")
                    .no_copy("^OH_NativeXComponent$")
                    .no_copy("^OH_NativeXComponent_KeyEvent$")
                    .no_debug("^OH_NativeXComponent$")
                    .no_debug("^OH_NativeXComponent_KeyEvent$")
                    .blocklist_item("ArkUI_.*")
                    // Requires a patch to bindgen
                    .blocklist_item("OH_NATIVEXCOMPONENT_RESULT.*")
                    .blocklist_function("OH_NativeXComponent_.*NativeRootNode")
                    .blocklist_function("OH_NativeXComponent_RegisterUIInputEventCallback")
                    .blocklist_function("OH_NativeXComponent_RegisterOnTouchInterceptCallback")
                    .blocklist_function("OH_NativeXComponent_GetNativeXComponent")
                    .blocklist_function("OH_ArkUI_XComponent_StartImageAnalyzer")
                    .blocklist_function("OH_ArkUI_XComponent_StopImageAnalyzer")
                    .raw_line("#[cfg(feature = \"api-13\")]")
                    .raw_line("use ohos_sys_opaque_types::ArkUI_AccessibilityProvider;")
                    .raw_line("#[cfg(feature = \"api-19\")]")
                    .raw_line("use ohos_sys_opaque_types::{ArkUI_NodeHandle,OHNativeWindow};")
                    .raw_line("#[cfg(feature = \"api-22\")]")
                    .raw_line("use ohos_sys_opaque_types::ArkUI_XComponentSurfaceConfig;")
                    .clang_args(&["-x", "c++"])
            }),
        },
        BindingConf {
            include_filename: "ace/xcomponent/native_interface_xcomponent.h".to_string(),
            output_prefix: "components/xcomponent/src/xcomponent_result".to_string(),
            set_builder_opts: Box::new(|builder| {
                builder
                    .raw_line("#![allow(unused)]")
                    .allowlist_var("OH_NATIVEXCOMPONENT_RESULT_.*")
                    .default_enum_style(EnumVariation::Consts)
                    .clang_args(&["-x", "c++"])
            }),
        },
        BindingConf {
            include_filename: "ace/xcomponent/native_interface_xcomponent.h".to_string(),
            output_prefix: "components/xcomponent/src/xcomponent_arkui".to_string(),
            set_builder_opts: Box::new(|builder| {
                builder
                    .raw_line("use super::xcomponent_ffi::*;")
                    .raw_line("use arkui_sys::native_type::ArkUI_NodeHandle;")
                    .raw_line("use arkui_sys::ui_input_event::{ArkUI_UIInputEvent, ArkUI_UIInputEvent_Type, HitTestMode};")
                    .allowlist_function("OH_NativeXComponent_.*NativeRootNode")
                    .allowlist_function("OH_NativeXComponent_RegisterUIInputEventCallback")
                    .allowlist_function("OH_NativeXComponent_RegisterOnTouchInterceptCallback")
                    .allowlist_function("OH_ArkUI_XComponent_StartImageAnalyzer")
                    .allowlist_function("OH_ArkUI_XComponent_StopImageAnalyzer")
                    // block all types except this one.
                    .blocklist_type("?!(ArkUI_XComponent_ImageAnalyzerState)")
                    .allowlist_type("ArkUI_XComponent_ImageAnalyzerState")
                    .clang_args(&["-x", "c++"])
            }),
        },
        BindingConf {
            include_filename: "native_vsync/native_vsync.h".to_string(),
            output_prefix: "components/vsync/src/vsync".to_string(),
            set_builder_opts: Box::new(move |builder| {
                builder
                    .clang_args(["-include", "stdbool.h", "-include", "stdint.h"])
            }),
        },
        BindingConf {
            include_filename: "qos/qos.h".to_string(),
            output_prefix: "components/qos/src/qos".to_string(),
            set_builder_opts: Box::new(|builder| {
                builder
                    .result_error_enum("OH_QoS_GewuErrorCode")
                    .parse_callbacks(Box::new(ResultEnumParseCallbacks {
                        rename_item: Box::new(|name| match name {
                            "OH_QoS_GewuErrorCode" => Some("OH_QoS_GewuResult".to_string()),
                            _ => None,
                        }),
                        rename_enum_variant: None,
                    }))
                    // `OH_QOS_GEWU_INVALID_{SESSION,REQUEST}_ID` are defined with
                    // C++ `static_cast<…>(0xffffffffU)` which bindgen drops
                    // silently in both C and C++ modes (its macro evaluator
                    // doesn't handle `static_cast` even though clang parses it).
                    // Blocklist and re-emit as plain Rust constants — gated on
                    // api-20 since both ids and their types are `@since 20`.
                    .blocklist_item("OH_QOS_GEWU_INVALID_SESSION_ID")
                    .blocklist_item("OH_QOS_GEWU_INVALID_REQUEST_ID")
                    .raw_line("#[cfg(feature = \"api-20\")]")
                    .raw_line("#[cfg_attr(docsrs, doc(cfg(feature = \"api-20\")))]")
                    .raw_line("pub const OH_QOS_GEWU_INVALID_SESSION_ID: OH_QoS_GewuSession = 0xffff_ffff;")
                    .raw_line("#[cfg(feature = \"api-20\")]")
                    .raw_line("#[cfg_attr(docsrs, doc(cfg(feature = \"api-20\")))]")
                    .raw_line("pub const OH_QOS_GEWU_INVALID_REQUEST_ID: OH_QoS_GewuRequest = 0xffff_ffff;")
            }),
        },
        BindingConf {
            include_filename: "arkui/ui_input_event.h".to_string(),
            output_prefix: "components/arkui/src/ui_input_event/ui_input_event_anon_enums"
                .to_string(),
            set_builder_opts: Box::new(move |builder| {
                builder
                    .allowlist_var("UI_TOUCH_EVENT_ACTION_.*")
                    .allowlist_var("UI_INPUT_EVENT_TOOL_TYPE_.*")
                    .allowlist_var("UI_INPUT_EVENT_SOURCE_TYPE_.*")
                    .allowlist_var("UI_MOUSE_EVENT_ACTION_.*")
                    .allowlist_var("UI_MOUSE_EVENT_BUTTON_*")
                    .allowlist_recursively(true)
                    .clang_args(["-include", "stdbool.h"])
            }),
        },
        BindingConf {
            include_filename: "arkui/styled_string.h".to_string(),
            output_prefix: "components/arkui/src/styled_string/drawing".to_string(),
            set_builder_opts: Box::new(|builder| {
                builder
                    .allowlist_function("OH_ArkUI_StyledString_Create")
                    .allowlist_function("OH_ArkUI_StyledString_PushTextStyle")
                    .allowlist_function("OH_ArkUI_StyledString_CreateTypography")
                    .allowlist_function("OH_ArkUI_StyledString_AddPlaceholder")
                    .clang_args(["-x", "c++"])
                    .allowlist_recursively(false)
                    .raw_line("pub use super::ArkUI_StyledString;")
                    .raw_line("pub use ohos_drawing_sys::text_typography::OH_Drawing_PlaceholderSpan;")
                    .raw_line("pub use ohos_drawing_sys::text_declaration::{OH_Drawing_FontCollection, OH_Drawing_TextStyle, OH_Drawing_Typography, OH_Drawing_TypographyStyle};")
            }),
        },
        BindingConf {
            include_filename: "rawfile/raw_file.h".to_string(),
            output_prefix: "components/rawfile/src/raw_file_types".to_string(),
            set_builder_opts: Box::new(|builder| {
                builder
                    .ignore_functions()
                    .clang_args(["-x", "c++"])
                    .blocklist_var("_LIBCPP.*")
                    .raw_line("#[cfg(doc)]")
                    .raw_line("use crate::{raw_file::{OH_ResourceManager_GetRawFileDescriptor,OH_ResourceManager_GetRawFileDescriptor64},\
                        raw_file_manager::OH_ResourceManager_OpenRawFile};")
            }),
        },
        BindingConf {
            include_filename: "AbilityKit/native_child_process.h".to_string(),
            output_prefix: "components/abilitykit/src/childprocess/childprocess".to_string(),
            set_builder_opts: Box::new(|builder| {
                builder
                    .result_error_enum("Ability_NativeChildProcess_ErrCode")
                    .allowlist_file(".*AbilityKit/native_child_process.h")
                    .clang_args(["-x", "c++"])
                    .parse_callbacks(Box::new(ResultEnumParseCallbacks {
                        rename_item: Box::new(|name| { // Ability_NativeChildProcess_ErrCode
                            name.strip_suffix("_ErrCode").map(|name | {
                                let mut s = name.to_string();
                                s.push_str("Result");
                                s
                            })
                        }),
                        rename_enum_variant: None,
                    }))
                    .raw_line("use ohos_sys_opaque_types::OHIPCRemoteProxy;")
            }),
        },
    ]
}
