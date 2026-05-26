use std::collections::HashMap;
use std::sync::LazyLock;

/// Defines the prefixes that each enum has (which we probably want to remove)
/// This essentially finds the enum given by .0 in the C code and strips the prefix .1 of
/// all elements in it
pub(crate) static ENUM_PREFIX_MAP: LazyLock<HashMap<&'static str, &'static str>> =
    LazyLock::new(|| {
        HashMap::from([
            ("Image_ErrorCode", "IMAGE_"),
            ("OH_Drawing_ErrorCode", "OH_DRAWING_ERROR_"),
            ("AbilityRuntime_ErrorCode", "ABILITY_RUNTIME_ERROR_CODE_"),
            ("AbilityBase_ErrorCode", "ABILITY_BASE_ERROR_CODE_"),
            ("ArkUI_DragResult", "ARKUI_DRAG_RESULT_"),
            ("ArkUI_GestureInterruptResult", "GESTURE_INTERRUPT_RESULT_"),
            ("ArkUI_ErrorCode", "ARKUI_ERROR_CODE_"),
            ("ArkUI_ListItemGroupArea", "ARKUI_LIST_ITEM_"),
            (
                "Image_CropAndScaleStrategy",
                "IMAGE_CROP_AND_SCALE_STRATEGY_",
            ),
            ("InputMethod_ErrorCode", "IME_ERR_"),
            ("OH_AudioCommon_Result", "AUDIOCOMMON_RESULT_ERROR_"),
            ("OH_AudioStream_Result", "AUDIOSTREAM_ERROR_"),
            ("ArkWeb_ErrorCode", "ARKWEB_"),
            ("Input_Result", "INPUT_"),
            (
                "OH_Drawing_FontConfigInfoErrorCode",
                "ERROR_FONT_CONFIG_INFO_",
            ),
            ("Ability_NativeChildProcess_ErrCode", "NCP_ERR_"),
            ("Udmf_Intention", "UDMF_INTENTION_"),
            ("Udmf_ShareOption", "SHARE_OPTIONS_"),
            ("Udmf_ErrCode", "UDMF_"),
            ("Location_ResultCode", "LOCATION_"),
            ("PASTEBOARD_ErrCode", "ERR_"),
            ("Pasteboard_NotifyType", "NOTIFY_"),
            ("OH_AVCodecBufferFlags", "AVCODEC_BUFFER_FLAGS_"),
            ("NativeDisplayManager_ErrorCode", "DISPLAY_MANAGER_ERROR_"),
            (
                "WindowManager_AvoidAreaType",
                "WINDOW_MANAGER_AVOID_AREA_TYPE_",
            ),
            ("WindowManager_WindowType", "WINDOW_MANAGER_WINDOW_TYPE_"),
            ("OH_Crypto_ErrCode", "CRYPTO_"),
            ("OH_QoS_GewuErrorCode", "OH_QOS_GEWU_"),
        ])
    });
