//! Configuration file for OpenHarmony modules with multiple header files in a directory.
//!
//! Add new bindings to `get_module_bindings_config()` by appending a new `DirBindingsConf`.

use crate::DirBindingsConf;
use std::default::Default;
use std::fmt::{Debug, Formatter};

/// Convenience method for stripping am optional suffix and returning an owned String
fn strip_suffix(input: &str, suffix: &str) -> String {
    match input.strip_suffix(suffix) {
        None => input.to_string(),
        Some(stripped) => stripped.to_string(),
    }
}

/// Convenience method for stripping am optional suffix and returning an owned String
fn strip_prefix(input: &str, prefix: &str) -> String {
    match input.strip_prefix(prefix) {
        None => input.to_string(),
        Some(stripped) => stripped.to_string(),
    }
}

pub struct ResultEnumParseCallbacks {
    /// fn item_name(&self, original_item_name: &str) -> Option<String> {
    pub(crate) rename_item: Box<dyn Fn(&str) -> Option<String>>,
    /// Custom renaming logic for enum variants.
    ///
    /// By default, we just try to lookup the prefix in `ENUM_PREFIX_MAP` and remove that.
    pub(crate) rename_enum_variant: Option<Box<dyn Fn(&str, &str) -> Option<String>>>,
}

impl Default for ResultEnumParseCallbacks {
    fn default() -> Self {
        Self {
            rename_item: Box::new(|_| None),
            rename_enum_variant: None,
        }
    }
}

impl Debug for ResultEnumParseCallbacks {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("ResultEnumParseCallbacks")
    }
}

impl bindgen::callbacks::ParseCallbacks for ResultEnumParseCallbacks {
    fn item_name(&self, original_item_name: &str) -> Option<String> {
        (self.rename_item)(original_item_name)
    }
}

pub(crate) fn get_module_bindings_config() -> Vec<DirBindingsConf> {
    vec![
        DirBindingsConf {
            directory: "multimedia/player_framework".to_string(),
            output_dir: "components/multimedia/player_framework/src".to_string(),
            rename_output_file: Some(Box::new(|stem| strip_prefix(stem, "native_"))),
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let builder = builder
                    .allowlist_file(header_path.to_str().unwrap())
                    .clang_args(["-x", "c++"]);
                let builder = if file_stem != "averrors" {
                    builder
                        // silence deprecation warnings, which are due to us using exposing
                        // deprecated APIs which in turn use deprecated types.
                        .raw_line("#![allow(deprecated)]")
                        .raw_line("#[allow(unused_imports)]use crate::averrors::OH_AVErrCode;")
                } else {
                    builder
                };
                match file_stem {
                    "avcodec_audiocodec" => builder
                        .raw_line("#[allow(unused_imports)]use crate::avbuffer::OH_AVBuffer;")
                        .raw_line("#[allow(unused_imports)]use crate::avcodec_base::{OH_AVCodec, OH_AVCodecCallback};")
                        .raw_line("#[allow(unused_imports)]use crate::avformat::OH_AVFormat;"),
                    "avcodec_audiodecoder" => builder
                        .raw_line("#[allow(unused_imports)]use crate::avcodec_base::{OH_AVCodec, OH_AVCodecAsyncCallback};")
                        .raw_line("#[allow(unused_imports)]use crate::avbuffer_info::OH_AVCodecBufferAttr;")
                        .raw_line("#[allow(unused_imports)]use crate::avformat::OH_AVFormat;"),
                    "avcodec_audioencoder" => builder
                        .raw_line("#[allow(unused_imports)]use crate::avcodec_base::{OH_AVCodec, OH_AVCodecAsyncCallback};")
                        .raw_line("#[allow(unused_imports)]use crate::avbuffer_info::OH_AVCodecBufferAttr;")
                        .raw_line("#[allow(unused_imports)]use crate::avformat::OH_AVFormat;"),
                    "avcodec_videodecoder" => builder
                        .raw_line("#[cfg(feature = \"api-11\")]#[allow(unused_imports)]use crate::avbuffer::OH_AVBuffer;")
                        .raw_line("#[allow(unused_imports)]use crate::avbuffer_info::OH_AVCodecBufferAttr;")
                        .raw_line("#[allow(unused_imports)]use crate::avcodec_base::{OH_AVCodec, OH_AVCodecAsyncCallback};")
                        .raw_line("#[cfg(feature = \"api-11\")]#[allow(unused_imports)]use crate::avcodec_base::OH_AVCodecCallback;")
                        .raw_line("#[allow(unused_imports)]use crate::avformat::OH_AVFormat;")
                        .raw_line("use ohos_sys_opaque_types::OHNativeWindow;"),
                    "avcodec_videoencoder" => builder
                        .raw_line("#[cfg(feature = \"api-11\")]#[allow(unused_imports)]use crate::avbuffer::OH_AVBuffer;")
                        .raw_line("#[allow(unused_imports)]use crate::avbuffer_info::OH_AVCodecBufferAttr;")
                        .raw_line("#[allow(unused_imports)]use crate::avcodec_base::{OH_AVCodec, OH_AVCodecAsyncCallback};")
                        .raw_line("#[cfg(feature = \"api-11\")]#[allow(unused_imports)]use crate::avcodec_base::OH_AVCodecCallback;")
                        .raw_line("#[allow(unused_imports)]use crate::avformat::OH_AVFormat;")
                        .raw_line("use ohos_sys_opaque_types::OHNativeWindow;"),
                    "avimage_generator" => builder
                        .raw_line("#[allow(unused_imports)]use crate::avimage_generator_base::OH_AVImageGenerator_QueryOptions;")
                        .raw_line("use ohos_sys_opaque_types::OH_PixelmapNative;"),
                    "avmetadata_extractor" => builder
                        .raw_line("#[allow(unused_imports)]use crate::avformat::OH_AVFormat;")
                        .raw_line("use ohos_sys_opaque_types::OH_PixelmapNative;")
                        .raw_line("#[cfg(feature = \"api-23\")]")
                        .raw_line("use crate::avmetadata_extractor_base::{OH_AVMetadataExtractor_FrameInfo, OH_AVMetadataExtractor_OutputParam};")
                        .raw_line("#[cfg(feature = \"api-23\")]")
                        .raw_line("use crate::avmedia_base::OH_AVMedia_SeekMode;")
                        .raw_line("#[cfg(feature = \"api-23\")]")
                        .raw_line("use crate::avmedia_source::OH_AVMediaSource;"),
                    "avmuxer" => builder
                        .raw_line("#[cfg(feature = \"api-11\")]#[allow(unused_imports)]use crate::avbuffer::OH_AVBuffer;")
                        .raw_line("#[allow(unused_imports)]use crate::avbuffer_info::OH_AVCodecBufferAttr;")
                        .raw_line("#[allow(unused_imports)]use crate::avcodec_base::OH_AVOutputFormat;")
                        .raw_line("#[allow(unused_imports)]use crate::avformat::OH_AVFormat;")
                        .raw_line("#[allow(unused_imports)]use crate::avmemory::OH_AVMemory;"),
                    "avrecorder" => builder
                        .raw_line("#[allow(unused_imports)]use crate::avrecorder_base::{OH_AVRecorder, OH_AVRecorder_Config, OH_AVRecorder_EncoderInfo, OH_AVRecorder_OnError, OH_AVRecorder_OnStateChange};")
                        .raw_line("use ohos_sys_opaque_types::OHNativeWindow;")
                        // missing media library bindings; blocklist dependent callback registration
                        .blocklist_function("OH_AVRecorder_SetUriCallback"),
                    "avrecorder_base" => builder
                        // missing media library bindings; blocklist dependent callback type
                        .blocklist_type("OH_MediaAsset")
                        .blocklist_type("OH_AVRecorder_OnUri"),
                    "avscreen_capture" => builder
                        .raw_line("#[allow(unused_imports)]use crate::avscreen_capture_base::{OH_AudioBuffer, OH_AudioCaptureSourceType, OH_AVScreenCapture, OH_AVScreenCaptureCallback, OH_AVScreenCaptureConfig, OH_Rect};")
                        .raw_line("#[cfg(feature = \"api-12\")]#[allow(unused_imports)]use crate::avscreen_capture_base::{OH_AVScreenCapture_ContentFilter, OH_AVScreenCaptureFilterableAudioContent, OH_AVScreenCapture_OnBufferAvailable, OH_AVScreenCapture_OnError, OH_AVScreenCapture_OnStateChange};")
                        .raw_line("#[cfg(feature = \"api-15\")]#[allow(unused_imports)]use crate::avscreen_capture_base::OH_AVScreenCapture_OnDisplaySelected;")
                        .raw_line("#[cfg(feature = \"api-20\")]#[allow(unused_imports)]use crate::avscreen_capture_base::{OH_AVScreenCapture_CaptureStrategy, OH_AVScreenCapture_FillMode, OH_AVScreenCapture_OnCaptureContentChanged, OH_AVScreenCapture_OnUserSelected, OH_AVScreenCapture_UserSelectionInfo};")
                        .raw_line("#[cfg(feature = \"api-22\")]use crate::avscreen_capture_base::{OH_AVScreenCaptureHighlightConfig, OH_CapturePickerMode};")
                        .raw_line("#[allow(unused_imports)]use crate::avscreen_capture_errors::OH_AVSCREEN_CAPTURE_ErrCode;")
                        .raw_line("#[allow(unused_imports)]use ohos_sys_opaque_types::{OHNativeWindow, OH_NativeBuffer};"),
                    "avscreen_capture_base" => builder
                        .raw_line("#[cfg(feature = \"api-11\")]#[allow(unused_imports)]use crate::avbuffer::OH_AVBuffer;"),
                    "avtranscoder" => builder
                        .raw_line("#[allow(unused_imports)]use crate::avcodec_base::OH_AVOutputFormat;")
                        .raw_line("#[allow(unused_imports)]use crate::avtranscoder_base::{OH_AVTranscoder, OH_AVTranscoder_Config, OH_AVTranscoder_OnError, OH_AVTranscoder_OnProgressUpdate, OH_AVTranscoder_OnStateChange};"),
                    "avplayer" => builder.raw_line("use ohos_sys_opaque_types::OHNativeWindow;")
                        .raw_line("use crate::avplayer_base::{AVPlaybackSpeed, AVPlayerCallback, AVPlayerSeekMode, AVPlayerState, OH_AVPlayer};")
                        .raw_line("#[cfg(feature = \"api-12\")]use crate::avplayer_base::{OH_AVPlayerOnErrorCallback, OH_AVPlayerOnInfoCallback};")
                        .raw_line("#[cfg(feature = \"api-20\")]")
                        .raw_line("#[allow(unused_imports)]use crate::avcodec_base::OH_AVDataSourceExt;")
                        .raw_line("#[cfg(feature = \"api-22\")]")
                        .raw_line("use crate::avformat::OH_AVFormat;")
                        .raw_line("#[cfg(feature = \"api-23\")]")
                        .raw_line("use crate::avplayer_base::{AVPlayerTrackSwitchMode, OH_AVPlaybackStrategy, OH_AVPlayerOnAmplitudeUpdateCallback, OH_AVPlayerOnSeiMessageReceivedCallback, OH_AVSeiMessageArray};")
                        .raw_line("#[cfg(feature = \"api-23\")]")
                        .raw_line("use crate::avmedia_source::OH_AVMediaSource;")
                        .raw_line("#[cfg(feature = \"api-23\")]")
                        .raw_line("use crate::avcodec_base::OH_MediaType;")
                        // require bindings to OH audio.
                        .blocklist_function("OH_AVPlayer_SetVolumeMode")
                        .blocklist_function("OH_AVPlayer_SetAudioRendererInfo")
                        .blocklist_function("OH_AVPlayer_SetAudioInterruptMode")
                        .blocklist_function("OH_AVPlayer_SetAudioEffectMode")
                    ,
                    "avplayer_base" => builder.raw_line("#[cfg(feature = \"api-12\")]use crate::avformat::OH_AVFormat;"),
                    "avcapability" => builder
                        .raw_line("#[cfg(feature = \"api-12\")]use crate::avformat::OH_AVFormat;")
                        .raw_line("use crate::avcodec_base::OH_BitrateMode;")
                        // Requires OH_NativeBuffer_Format from ohos-window-sys (not a dependency of this crate).
                        .blocklist_function("OH_AVCapability_GetVideoSupportedNativeBufferFormats"),

                    "avcodec_base" => builder
                        .raw_line("use crate::avbuffer_info::OH_AVCodecBufferAttr;")
                        .raw_line("use crate::avmemory::OH_AVMemory;")
                        .raw_line("use crate::avformat::OH_AVFormat;")
                        .raw_line("#[cfg(feature = \"api-11\")]use crate::avbuffer::OH_AVBuffer;")
                    ,
                    "avsource" => builder
                        .raw_line("#[cfg(feature = \"api-12\")]use crate::avcodec_base::OH_AVDataSource;")
                        .raw_line("use crate::avformat::OH_AVFormat;")
                        .raw_line("#[cfg(feature = \"api-20\")]")
                        .raw_line("use crate::avcodec_base::OH_AVDataSourceExt;")
                    ,
                    "avmedia_source" => builder
                        .raw_line("use crate::avcodec_base::OH_AVDataSource;"),
                    "avmetadata_extractor_base" => builder
                        .raw_line("#[cfg(feature = \"api-23\")]")
                        .raw_line("use ohos_sys_opaque_types::OH_PixelmapNative;"),
                    "avbuffer" => builder.raw_line("use ohos_sys_opaque_types::OH_NativeBuffer;")
                        .raw_line("use crate::avbuffer_info::OH_AVCodecBufferAttr;")
                        .raw_line("use crate::avformat::OH_AVFormat;"),
                    "avbuffer_info" => builder
                        .bitfield_enum("OH_AVCodecBufferFlags")
                    ,
                    "avdemuxer" => builder
                        .raw_line("#[cfg(feature = \"api-11\")]use crate::avbuffer::OH_AVBuffer;")
                        .raw_line("use crate::avbuffer_info::OH_AVCodecBufferAttr;")
                        .raw_line("use crate::avcodec_base::OH_AVSeekMode;")
                        .raw_line("use crate::avsource::OH_AVSource;")
                        .raw_line("use crate::avmemory::OH_AVMemory;")
                    ,
                    "lowpower_audio_sink" => builder
                        .raw_line("#[allow(unused_imports)]use crate::avformat::OH_AVFormat;")
                        .raw_line("#[allow(unused_imports)]use crate::lowpower_audio_sink_base::{OH_LowPowerAudioSink, OH_LowPowerAudioSinkCallback, OH_LowPowerAudioSink_OnDataNeeded, OH_LowPowerAudioSink_OnEos, OH_LowPowerAudioSink_OnError, OH_LowPowerAudioSink_OnPositionUpdated};")
                        .raw_line("#[allow(unused_imports)]use crate::lowpower_avsink_base::OH_AVSamplesBuffer;")
                        // missing ohaudio bindings; blocklist dependent callbacks
                        .blocklist_function("OH_LowPowerAudioSinkCallback_SetInterruptListener")
                        .blocklist_function("OH_LowPowerAudioSinkCallback_SetDeviceChangeListener"),
                    "lowpower_audio_sink_base" => builder
                        .raw_line("#[allow(unused_imports)]use crate::lowpower_avsink_base::OH_AVSamplesBuffer;")
                        // missing ohaudio bindings; blocklist dependent types
                        .blocklist_type("OH_AudioInterrupt_ForceType")
                        .blocklist_type("OH_AudioInterrupt_Hint")
                        .blocklist_type("OH_AudioStream_DeviceChangeReason")
                        .blocklist_type("OH_LowPowerAudioSink_OnInterrupted")
                        .blocklist_type("OH_LowPowerAudioSink_OnDeviceChanged"),
                    "lowpower_avsink_base" => builder
                        .raw_line("#[allow(unused_imports)]use crate::avbuffer::OH_AVBuffer;"),
                    "lowpower_video_sink" => builder
                        .raw_line("#[allow(unused_imports)]use crate::avformat::OH_AVFormat;")
                        .raw_line("#[allow(unused_imports)]use crate::lowpower_audio_sink_base::OH_LowPowerAudioSink;")
                        .raw_line("#[allow(unused_imports)]use crate::lowpower_avsink_base::OH_AVSamplesBuffer;")
                        .raw_line("#[allow(unused_imports)]use crate::lowpower_video_sink_base::{OH_LowPowerVideoSink, OH_LowPowerVideoSinkCallback, OH_LowPowerVideoSink_OnDataNeeded, OH_LowPowerVideoSink_OnEos, OH_LowPowerVideoSink_OnError, OH_LowPowerVideoSink_OnFirstFrameDecoded, OH_LowPowerVideoSink_OnRenderStarted, OH_LowPowerVideoSink_OnStreamChanged, OH_LowPowerVideoSink_OnTargetArrived};")
                        .raw_line("use ohos_sys_opaque_types::OHNativeWindow;"),
                    "lowpower_video_sink_base" => builder
                        .raw_line("#[allow(unused_imports)]use crate::avformat::OH_AVFormat;")
                        .raw_line("#[allow(unused_imports)]use crate::lowpower_avsink_base::OH_AVSamplesBuffer;"),
                    _ => builder,
                }
            }),
            ..Default::default()
        },
        DirBindingsConf {
            directory: "native_window/".to_string(),
            output_dir: "components/window/src/native_window".to_string(),
            rename_output_file: None,
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let builder = builder
                    .allowlist_file(header_path.to_str().unwrap())
                    .constified_enum_module("^NativeWindowOperation$");
                match file_stem {
                    "external_window" => builder
                        .raw_line("use crate::native_window::BufferHandle;")
                        .raw_line("use ohos_sys_opaque_types::{OHNativeWindow, OHNativeWindowBuffer};")
                        .raw_line("#[cfg(feature = \"api-12\")]")
                        .raw_line("use crate::native_buffer::buffer_common::{OH_NativeBuffer_ColorSpace, OH_NativeBuffer_MetadataKey};")
                        .raw_line("#[cfg(feature = \"api-12\")]")
                        .raw_line("use ohos_sys_opaque_types::OHIPCParcel;")
                        .raw_line("#[cfg(feature = \"api-11\")]")
                        .raw_line("use ohos_sys_opaque_types::{OH_NativeBuffer};")
                        .blocklist_type("OHIPCParcel")
                    ,
                    _ => builder,
                }
            }),
            ..Default::default()
        },
        DirBindingsConf {
            directory: "native_buffer/".to_string(),
            output_dir: "components/window/src/native_buffer".to_string(),
            rename_output_file: None,
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let builder = builder
                    .allowlist_file(header_path.to_str().unwrap())
                    .blocklist_file(".*graphic_error_code.h")
                    .clang_args(["-include", "stdbool.h"]);
                match file_stem {
                    "native_buffer" => builder
                        .raw_line("use ohos_sys_opaque_types::OH_NativeBuffer;")
                        .raw_line("#[cfg(feature = \"api-11\")]")
                        .raw_line(
                            "use crate::native_buffer::buffer_common::OH_NativeBuffer_ColorSpace;",
                        )
                        .raw_line("#[cfg(feature = \"api-12\")]")
                        .raw_line("use ohos_sys_opaque_types::OHNativeWindowBuffer;")
                        .raw_line("#[cfg(feature = \"api-12\")]")
                        .raw_line(
                            "use crate::native_buffer::buffer_common::OH_NativeBuffer_MetadataKey;",
                        )
                        .raw_line("#[cfg(feature = \"api-23\")]")
                        .raw_line("use ohos_sys_opaque_types::OHIPCParcel;")
                        .blocklist_type("OHIPCParcel")
                        .bitfield_enum("OH_NativeBuffer_Usage"),
                    _ => builder,
                }
            }),
            skip_files: vec!["graphic_error_code.h".to_string()],
            ..Default::default()
        },
        DirBindingsConf {
            directory: "native_fence/".to_string(),
            output_dir: "components/window/src/native_fence".to_string(),
            rename_output_file: None,
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                builder
                    .allowlist_file(header_path.to_str().unwrap())
                    .blocklist_file(".buffer_common.h")
                    .blocklist_file(".*graphic_error_code.h")
                    .clang_args(["-include", "stdbool.h"])
            }),
            skip_files: vec!["graphic_error_code.h".to_string()],
            ..Default::default()
        },
        DirBindingsConf {
            directory: "native_image/".to_string(),
            output_dir: "components/window/src/native_image".to_string(),
            rename_output_file: None,
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let builder = builder
                    .allowlist_file(header_path.to_str().unwrap())
                    .blocklist_file(".*graphic_error_code.h");
                match file_stem {
                    "native_image" => builder
                        .raw_line("use ohos_sys_opaque_types::{OHNativeWindow, OH_NativeImage};")
                        .raw_line("#[cfg(feature = \"api-12\")]")
                        .raw_line("use ohos_sys_opaque_types::OHNativeWindowBuffer;")
                        .raw_line("#[cfg(feature = \"api-22\")]")
                        .raw_line(
                            "use crate::native_buffer::buffer_common::OH_NativeBuffer_ColorSpace;",
                        )
                        .clang_args(["-include", "stdbool.h"])
                        .no_copy("^OH_OnFrameAvailableListener"),
                    _ => builder,
                }
            }),
            skip_files: vec!["graphic_error_code.h".to_string()],
            ..Default::default()
        },
        DirBindingsConf {
            directory: "database/pasteboard".to_string(),
            output_dir: "components/pasteboard/src".to_string(),
            rename_output_file: Some(Box::new(|stem| strip_prefix(stem, "oh_"))),
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let builder = builder.allowlist_file(header_path.to_str().unwrap());
                match file_stem {
                    "pasteboard" => builder.raw_line("use ohos_sys_opaque_types::OH_UdmfData;"),
                    _ => builder,
                }
            }),
            ..Default::default()
        },
        DirBindingsConf {
            directory: "database/data".to_string(),
            output_dir: "components/rdb/src".to_string(),
            rename_output_file: Some(Box::new(|stem| strip_prefix(stem, "oh_"))),
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let builder = builder
                    .allowlist_file(header_path.to_str().unwrap())
                    .clang_args(["-include", "stdbool.h"]);

                match file_stem {
                    "data_asset" => builder.raw_line("use crate::rdb_types::Data_Asset;"),
                    "data_value" => builder
                        .blocklist_type("OH_ColumnType")
                        .raw_line("use crate::rdb_types::{Data_Asset, OH_ColumnType, OH_Data_Value};"),
                    "data_values" => builder.raw_line(
                        "use crate::rdb_types::{Data_Asset, OH_ColumnType, OH_Data_Value, OH_Data_Values};",
                    ),
                    "data_values_buckets" => builder
                        .raw_line("use crate::values_bucket::{OH_Data_VBuckets, OH_VBucket};"),
                    _ => builder,
                }
            }),
            ..Default::default()
        },
        DirBindingsConf {
            directory: "database/rdb".to_string(),
            output_dir: "components/rdb/src".to_string(),
            rename_output_file: Some(Box::new(|stem| strip_prefix(stem, "oh_"))),
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let builder = builder
                    .allowlist_file(header_path.to_str().unwrap())
                    .clang_args(["-include", "stdbool.h"]);

                match file_stem {
                    "cursor" => builder.raw_line("use crate::rdb_types::{Data_Asset, OH_ColumnType};"),
                    "rdb_types" => builder
                        .raw_line("#[cfg(feature = \"api-23\")]")
                        .raw_line("use crate::cursor::OH_Cursor;"),
                    "predicates" => builder
                        .raw_line("#[cfg(feature = \"api-20\")]")
                        .raw_line("use crate::rdb_types::OH_Data_Values;")
                        .raw_line("use crate::value_object::OH_VObject;"),
                    "rdb_transaction" => builder
                        .raw_line("#[cfg(feature = \"api-18\")]")
                        .raw_line("use crate::cursor::OH_Cursor;")
                        .raw_line("#[cfg(feature = \"api-18\")]")
                        .raw_line("use crate::predicates::OH_Predicates;")
                        .raw_line("#[cfg(feature = \"api-18\")]")
                        .raw_line("use crate::rdb_types::{OH_Data_Value, OH_Data_Values, Rdb_ConflictResolution};")
                        .raw_line("#[cfg(feature = \"api-18\")]")
                        .raw_line("use crate::values_bucket::{OH_Data_VBuckets, OH_VBucket};")
                        .raw_line("#[cfg(feature = \"api-23\")]")
                        .raw_line("use crate::rdb_types::OH_RDB_ReturningContext;"),
                    "relational_store" => builder
                        .raw_line("use crate::cursor::OH_Cursor;")
                        .raw_line("use crate::predicates::OH_Predicates;")
                        .raw_line("#[cfg(feature = \"api-20\")]")
                        .raw_line("use crate::rdb_crypto_param::OH_Rdb_CryptoParam;")
                        .raw_line("#[cfg(feature = \"api-18\")]")
                        .raw_line("use crate::rdb_transaction::{OH_RDB_TransOptions, OH_Rdb_Transaction};")
                        .raw_line("#[cfg(feature = \"api-18\")]")
                        .raw_line("use crate::rdb_types::{OH_Data_Value, OH_Data_Values, Rdb_ConflictResolution};")
                        .raw_line("use crate::value_object::OH_VObject;")
                        .raw_line("#[cfg(feature = \"api-18\")]")
                        .raw_line("use crate::values_bucket::OH_Data_VBuckets;")
                        .raw_line("use crate::values_bucket::OH_VBucket;")
                        .raw_line("#[cfg(feature = \"api-23\")]")
                        .raw_line("use crate::rdb_types::OH_RDB_ReturningContext;"),
                    "values_bucket" => builder
                        .raw_line("#[cfg(feature = \"api-11\")]")
                        .raw_line("use crate::rdb_types::Data_Asset;"),
                    _ => builder,
                }
            }),
            ..Default::default()
        },
        DirBindingsConf {
            directory: "database/udmf".to_string(),
            output_dir: "components/udmf/src".to_string(),
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let builder = builder.allowlist_file(header_path.to_str().unwrap());

                match file_stem {
                    "udmf" => builder.raw_line("use ohos_sys_opaque_types::*;"),
                    "uds" => builder
                        .raw_line("pub use ohos_sys_opaque_types::{OH_UdsAppItem, OH_UdsHtml, OH_UdsHyperlink, OH_UdsPlainText};")
                        .raw_line("#[cfg(feature = \"api-13\")]use ohos_sys_opaque_types::OH_PixelmapNative;")
                        .raw_line("#[cfg(feature = \"api-13\")]pub use ohos_sys_opaque_types::{OH_UdsPixelMap, OH_UdsArrayBuffer, OH_UdsFileUri};")
                        .raw_line("#[cfg(feature = \"api-14\")]pub use ohos_sys_opaque_types::OH_UdsContentForm;")
                    ,
                    "utd" => builder.raw_line("pub use ohos_sys_opaque_types::OH_Utd;"),
                    _ => builder,
                }
            }),
            ..Default::default()
        },
        DirBindingsConf {
            directory: "multimedia/image_framework/image".to_string(),
            output_dir: "components/multimedia/image_framework/src/native_image".to_string(),
            rename_output_file: Some(Box::new(|stem| strip_suffix(stem, "_native"))),
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let builder = if file_stem != "image_common" {
                    builder.raw_line("use crate::native_image::common::*;")
                } else {
                    builder
                };
                let builder = builder.parse_callbacks(Box::new(ResultEnumParseCallbacks {
                    rename_item: Box::new(|original_item_name| match original_item_name {
                        "Image_ErrorCode" => Some("ImageResult".to_string()),
                        _ => None,
                    }),
                    ..Default::default()
                }));
                let builder = match file_stem {
                    "pixelmap" => builder.raw_line(
                        "use ohos_sys_opaque_types::{napi_env, napi_value, \
                            OH_NativeBuffer, OH_PixelmapNative, OH_NativeColorSpaceManager};",
                    ),
                    "picture" => {
                        builder
                            .raw_line("use ohos_sys_opaque_types::{OH_PixelmapNative, OH_PictureNative};")
                            .raw_line("use crate::native_image::pixelmap::PIXEL_FORMAT;")
                            .raw_line("pub use crate::native_image::image_source::Image_AuxiliaryPictureType;")
                            .blocklist_item("Image_AuxiliaryPictureType")
                        // OH_PictureNative is blocklisted globally via OPAQUE_TYPES
                    }
                    "image_source" => builder
                        .raw_line("pub use ohos_sys_opaque_types::OH_ImageSourceNative;")
                        .raw_line("use ohos_sys_opaque_types::OH_PixelmapNative;")
                        .raw_line("use ohos_rawfile_sys::RawFileDescriptor;")
                        .raw_line("#[cfg(feature = \"api-13\")]")
                        .raw_line("use ohos_sys_opaque_types::OH_PictureNative;")
                        .allowlist_item("Image_AuxiliaryPictureType"),
                    "image_receiver" => {
                        builder.raw_line("use crate::native_image::image::OH_ImageNative;")
                    }
                    "image_packer" => builder
                        .raw_line("use ohos_sys_opaque_types::OH_PixelmapNative;")
                        .raw_line("#[cfg(feature = \"api-12\")]")
                        .raw_line("use ohos_sys_opaque_types::OH_ImageSourceNative;")
                        .raw_line("#[cfg(feature = \"api-13\")]")
                        .raw_line("use ohos_sys_opaque_types::OH_PictureNative;"),
                    "image" => {
                        builder
                            .raw_line("use ohos_sys_opaque_types::OH_NativeBuffer;")
                            // API-23 `OH_ImageNative_GetFormat` returns `OH_NativeBuffer_Format`
                            // from `ohos-window-sys`, which is not a dependency of this crate.
                            .blocklist_function("OH_ImageNative_GetFormat")
                    }
                    _ => builder,
                };
                builder
                    .allowlist_file(header_path.to_str().unwrap())
                    .derive_copy(false)
                    .prepend_enum_name(false)
                    .clang_args(&["-x", "c++"])
            }),
            ..Default::default()
        },
        DirBindingsConf {
            directory: "inputmethod".to_string(),
            output_dir: "components/inputmethod/src".to_string(),
            rename_output_file: Some(Box::new(|stem| {
                let stem = strip_suffix(stem, "_capi");

                strip_prefix(&stem, "inputmethod_")
            })),
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let builder = if file_stem != "types" {
                    builder.raw_line("use crate::types::*;")
                } else {
                    builder.result_error_enum("InputMethod_ErrorCode")
                }
                .parse_callbacks(Box::new(ResultEnumParseCallbacks {
                    rename_item: Box::new(|enum_name| match enum_name {
                        "InputMethod_ErrorCode" => Some("InputMethodResult".to_string()),
                        _ => None,
                    }),
                    ..Default::default()
                }));
                let builder = match file_stem {
                    "text_editor_proxy" => builder
                        .raw_line("use crate::private_command::InputMethod_PrivateCommand;")
                        .raw_line("use crate::text_config::InputMethod_TextConfig;"),
                    "text_config" => builder
                        .raw_line("use crate::text_avoid_info::InputMethod_TextAvoidInfo;")
                        .raw_line("use crate::cursor_info::InputMethod_CursorInfo;"),
                    "inputmethod_proxy" => builder
                        .raw_line("use crate::private_command::InputMethod_PrivateCommand;")
                        .raw_line("use crate::cursor_info::InputMethod_CursorInfo;")
                        .raw_line("#[cfg(feature = \"api-15\")]")
                        .raw_line("use crate::attach_options::InputMethod_AttachOptions;"),
                    "controller" => builder
                        .raw_line("use crate::inputmethod_proxy::InputMethod_InputMethodProxy;")
                        .raw_line("use crate::text_editor_proxy::InputMethod_TextEditorProxy;")
                        .raw_line("use crate::attach_options::InputMethod_AttachOptions;")
                        .raw_line("#[cfg(feature = \"api-23\")]")
                        .raw_line("use ohos_sys_opaque_types::ArkUI_ContextHandle;"),
                    _ => builder,
                };
                builder
                    .allowlist_file(header_path.to_str().unwrap())
                    .prepend_enum_name(false)
                    .clang_args(&["-x", "c++"])
            }),
            ..Default::default()
        },
        DirBindingsConf {
            directory: "native_drawing".to_string(),
            output_dir: "components/drawing/src".to_string(),
            rename_output_file: Some(Box::new(|stem| strip_prefix(stem, "drawing_"))),
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let builder = if file_stem != "types" {
                    let builder = builder.raw_line("use crate::types::*;");
                    if file_stem != "error_code" {
                        builder.parse_callbacks(Box::new(ResultEnumParseCallbacks {
                            rename_item: Box::new(|original_item_name| match original_item_name {
                                "OH_Drawing_ErrorCode" => {
                                    Some("crate::error_code::DrawingResult".to_string())
                                }
                                _ => None,
                            }),
                            rename_enum_variant: None,
                        }))
                    } else {
                        builder
                            .result_error_enum("OH_Drawing_ErrorCode")
                            .parse_callbacks(Box::new(ResultEnumParseCallbacks {
                                rename_item: Box::new(
                                    |original_item_name| match original_item_name {
                                        "OH_Drawing_ErrorCode" => Some("DrawingResult".to_string()),
                                        _ => None,
                                    },
                                ),
                                ..Default::default()
                            }))
                    }
                } else {
                    builder
                };
                let builder = match file_stem {
                    "brush" => builder
                        .raw_line("#[cfg(feature = \"api-20\")]")
                        .raw_line("use ohos_sys_opaque_types::OH_NativeColorSpaceManager;")
                    ,
                    "canvas" => builder
                        .raw_line("#[cfg(feature = \"api-18\")]")
                        .raw_line("use crate::sampling_options::OH_Drawing_FilterMode;"),
                    "font_collection" => builder.raw_line("use crate::text_declaration::*;"),
                    "text_typography" => builder
                        .raw_line("use crate::text_declaration::*;")
                        .raw_line("#[cfg(feature = \"api-12\")]")
                        .raw_line("use crate::font::OH_Drawing_Font_Metrics;")
                        .raw_line("#[cfg(feature = \"api-11\")]")
                        .raw_line("use ohos_sys_opaque_types::{OH_Drawing_TextBox, OH_Drawing_PositionAndAffinity};"),
                    "text_font_descriptor" => builder
                        .raw_line("use crate::text_typography::OH_Drawing_FontDescriptor;")
                        .raw_line("#[cfg(feature = \"api-22\")]")
                        .raw_line("use crate::text_declaration::OH_Drawing_FontFullDescriptor;"),
                    "register_font" => builder.raw_line("use crate::text_declaration::*;"),
                    "image_filter" => builder.raw_line("use crate::shader_effect::*;"),
                    "font_mgr" => builder.raw_line("use crate::text_typography::*;"),
                    "pen" => builder
                        .raw_line("#[cfg(feature = \"api-20\")]")
                        .raw_line("use ohos_sys_opaque_types::OH_NativeColorSpaceManager;"),
                    "pixel_map" => builder.raw_line(
                        "use ohos_sys_opaque_types::{OH_PixelmapNative, NativePixelMap_};",
                    ),
                    "text_blob" => builder.no_copy("OH_Drawing_RunBuffer"),
                    "text_line" => builder.raw_line("use crate::text_declaration::{OH_Drawing_Run, OH_Drawing_TextLine, OH_Drawing_Typography};"),
                    "text_lineTypography" => builder.raw_line("use crate::text_declaration::{OH_Drawing_LineTypography, OH_Drawing_TextLine, OH_Drawing_TypographyCreate};"),
                    "text_run" => builder
                        .raw_line("use crate::text_declaration::OH_Drawing_Run;")
                        .raw_line("#[cfg(feature = \"api-20\")]")
                        .raw_line("use crate::text_typography::OH_Drawing_TextDirection;")
                    ,
                    _ => builder,
                };
                builder
                    .allowlist_file(header_path.to_str().unwrap())
                    .prepend_enum_name(false)
                    .clang_args(&["-x", "c++"])
                    .clang_args(["-include", "stddef.h"])
            }),
            ..Default::default()
        },
        DirBindingsConf {
            directory: "arkui".to_string(),
            output_dir: "components/arkui/src".to_string(),
            rename_output_file: None,
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let builder = if file_stem != "native_type" {
                    builder.raw_line("use crate::native_type::*;")
                } else {
                    builder
                        .raw_line("pub use ohos_sys_opaque_types::{ArkUI_Node, ArkUI_NodeHandle};")
                };
                let builder = builder
                    .allowlist_file(header_path.to_str().unwrap())
                    .prepend_enum_name(false)
                    .parse_callbacks(Box::new(ResultEnumParseCallbacks {
                        rename_item: Box::new(|original_item_name| match original_item_name {
                            "ArkUI_ErrorCode" => Some("ArkUiResult".to_string()),
                            _ => None,
                        }),
                        ..Default::default()
                    }))
                    .clang_args(&["-x", "c++"]);
                match file_stem {
                    "drag_and_drop" => {
                        builder
                            // Pixelmap is from image-kit
                            .raw_line("pub use ohos_sys_opaque_types::OH_PixelmapNative;")
                            .raw_line("use ohos_sys_opaque_types::OH_UdmfData;")
                            .raw_line("#[cfg(feature =\"api-12\")]")
                            .raw_line("use ohos_sys_opaque_types::ArkUI_ContextHandle;")
                            .raw_line("#[cfg(feature =\"api-15\")]")
                            .raw_line("use ohos_sys_opaque_types::OH_UdmfGetDataParams;")
                            .raw_line("#[cfg(feature =\"api-20\")]")
                            .raw_line("use ohos_sys_opaque_types::OH_UdmfDataLoadParams;")
                    }
                    "native_animate" => builder
                        .no_debug("ArkUI_NativeAnimateAPI_.*")
                        .no_copy("ArkUI_NativeAnimateAPI_.*")
                        .raw_line("#[cfg(feature =\"api-12\")]")
                        .raw_line("use ohos_sys_opaque_types::ArkUI_ContextHandle;"),
                    "native_interface_focus" => builder
                        .raw_line("#[cfg(feature =\"api-15\")]")
                        .raw_line("use ohos_sys_opaque_types::ArkUI_ContextHandle;"),
                    "drawable_descriptor" => {
                        builder.raw_line("pub use ohos_sys_opaque_types::OH_PixelmapNative;")
                    }
                    "native_dialog" => builder
                        .no_debug("ArkUI_NativeDialogAPI_.*")
                        .no_copy("ArkUI_NativeDialogAPI_.*")
                        .raw_line("#[cfg(feature =\"api-19\")]")
                        .raw_line("use crate::native_node::ArkUI_AttributeItem;"),
                    "native_gesture" => builder
                        .raw_line("use crate::ui_input_event::ArkUI_UIInputEvent;")
                        .blocklist_function("^OH_ArkUI_GestureEvent_GetNode")
                        .blocklist_function("^OH_ArkUI_GestureEvent_SetNode")
                        .no_debug("ArkUI_NativeGestureAPI_1")
                        .no_copy("ArkUI_NativeGestureAPI_1"),
                    "native_interface_accessibility" => {
                        builder.raw_line("use ohos_sys_opaque_types::ArkUI_AccessibilityProvider;")
                    }
                    "native_key_event" => {
                        builder.raw_line("use crate::ui_input_event::ArkUI_UIInputEvent;")
                    }
                    "native_node" => builder
                        .blocklist_var("MAX_NODE_SCOPE_NUM")
                        .blocklist_var("MAX_COMPONENT_EVENT_ARG_NUM")
                        .raw_line("#[cfg(feature =\"api-12\")]")
                        .raw_line("use ohos_sys_opaque_types::ArkUI_ContextHandle;")
                        .raw_line("#[cfg(feature =\"api-15\")]")
                        .raw_line("use ohos_sys_opaque_types::OH_PixelmapNative;")
                        .raw_line("use crate::ui_input_event::ArkUI_UIInputEvent;")
                        .raw_line("#[cfg(feature = \"api-22\")]")
                        .raw_line("use crate::ui_input_event::ArkUI_TouchTestInfo;"),
                    "native_node_napi" => builder
                        .raw_line("use ohos_sys_opaque_types::{napi_env, napi_value};")
                        .raw_line("#[cfg(feature =\"api-12\")]")
                        .raw_line("use ohos_sys_opaque_types::ArkUI_ContextHandle;")
                        .raw_line("use crate::drawable_descriptor::ArkUI_DrawableDescriptor;"),
                    "native_type" => {
                        builder
                            .raw_line("use crate::drawable_descriptor::ArkUI_DrawableDescriptor;")
                            // We want copy for the union type `ArkUI_NumberValue`
                            .derive_copy(true)
                            .no_copy("ArkUI_ContextCallback")
                            .no_copy("ARKUI_TextPickerRangeContent")
                            .no_copy("ARKUI_TextPickerCascadeRangeContent")
                            .no_copy("ArkUI_ColorStop")
                            .result_error_enum("ArkUI_ErrorCode")
                    }
                    "styled_string" => builder
                        .blocklist_function("OH_ArkUI_StyledString_Create")
                        .blocklist_function("OH_ArkUI_StyledString_PushTextStyle")
                        .blocklist_function("OH_ArkUI_StyledString_CreateTypography")
                        .blocklist_function("OH_ArkUI_StyledString_AddPlaceholder")
                        // API-22 TextLayoutManager APIs use drawing types from `ohos-drawing-sys`,
                        // which is not a dependency of this crate; blocklist for now.
                        .blocklist_function("OH_ArkUI_TextLayoutManager_GetRectsForRange")
                        .blocklist_function(
                            "OH_ArkUI_TextLayoutManager_GetGlyphPositionAtCoordinate",
                        )
                        .blocklist_function("OH_ArkUI_TextLayoutManager_GetLineMetrics"),
                    "ui_input_event" => builder
                        .bitfield_enum("ArkUI_ModifierKeyName")
                        .blocklist_item("UI_TOUCH_EVENT_ACTION_.*")
                        .blocklist_item("UI_INPUT_EVENT_TOOL_TYPE_.*")
                        .blocklist_item("UI_INPUT_EVENT_SOURCE_TYPE_.*")
                        .blocklist_item("UI_MOUSE_EVENT_ACTION_.*")
                        .blocklist_item("UI_MOUSE_EVENT_BUTTON_.*"),
                    _ => builder,
                }
            }),
            ..Default::default()
        },
        DirBindingsConf {
            directory: "rawfile".to_string(),
            output_dir: "components/rawfile/src".to_string(),
            rename_output_file: None,
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let builder = builder
                    .allowlist_file(header_path.to_str().unwrap())
                    .prepend_enum_name(false)
                    .clang_args(&["-x", "c++"]);
                match file_stem {
                         "raw_file" => {
                             builder
                                 // Types are generated separately, since they might be shared.
                                 .blocklist_var(".*")
                                 .blocklist_type(".*")
                                 .raw_line("use crate::raw_file_types_ffi::*;")
                                 .raw_line("#[cfg(doc)]")
                                 .raw_line("use crate::raw_file_manager::{OH_ResourceManager_OpenRawFile, OH_ResourceManager_OpenRawDir};")
                                 .raw_line("#[cfg(doc)]")
                                 .raw_line("use crate::raw_file_manager::OH_ResourceManager_OpenRawFile64;")
                         },
                         "raw_dir" => {
                             builder
                                 .raw_line("#[cfg(doc)]")
                                 .raw_line("use crate::raw_file_manager::{OH_ResourceManager_OpenRawFile, OH_ResourceManager_OpenRawDir};")

                         }
                         "raw_file_manager" => {
                             builder
                                 .raw_line("use ohos_sys_opaque_types::{napi_env, napi_value};")
                                 .raw_line("use crate::raw_dir::RawDir;")
                                 .raw_line("use crate::RawFile;")
                                 .raw_line("#[cfg(doc)]")
                                 .raw_line("use crate::raw_dir::OH_ResourceManager_CloseRawDir;")
                                 .raw_line("#[cfg(doc)]")
                                 .raw_line("use crate::raw_file::{OH_ResourceManager_CloseRawFile, OH_ResourceManager_CloseRawFile64};")
                                 .raw_line("#[cfg(feature = \"api-11\")]")
                                 .raw_line("use crate::RawFile64;")

                         }
                         _ => builder,
                     }
            }),
            ..Default::default()
        },
        DirBindingsConf {
            directory: "multimodalinput".to_string(),
            output_dir: "components/multimodal-input/src".to_string(),
            rename_output_file: Some(Box::new(|name| name.trim_start_matches("oh_").to_string())),
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let builder = builder
                    .allowlist_file(header_path.to_str().unwrap())
                    .prepend_enum_name(false)
                    .clang_args(["-include", "stdbool.h"]);
                match file_stem {
                         "input_manager" => {
                             builder
                                 .result_error_enum("Input_Result")
                                 .parse_callbacks(Box::new(ResultEnumParseCallbacks::default()))
                                 .raw_line("use crate::axis_type::{InputEvent_AxisAction, InputEvent_AxisEventType, InputEvent_AxisType};")
                                 .raw_line("use ohos_sys_opaque_types::{Input_AxisEvent, Input_KeyEvent, Input_KeyState, Input_MouseEvent, Input_TouchEvent};")
                                 .raw_line("#[cfg(feature = \"api-14\")]")
                                 .raw_line("use ohos_sys_opaque_types::Input_Hotkey;")
                                 .raw_line("#[cfg(feature = \"api-22\")]")
                                 .raw_line("use crate::pointer_style::Input_PointerStyle;")
                                 .raw_line("#[cfg(feature = \"api-22\")]")
                                 .raw_line("use ohos_sys_opaque_types::OH_PixelmapNative;")
                         },
                         "key_code" => {
                             builder
                                 // Input_KeyCode is not directly exposed via FFI, instead a
                                 // raw integer is used there. Hence, we can use a rust enum here
                                 // which is much nicer to use.
                                 .rustified_enum("Input_KeyCode")
                         }
                         _ => builder,
                     }
            }),
            ..Default::default()
        },
        DirBindingsConf {
            directory: "AbilityKit/ability_base".to_string(),
            output_dir: "components/abilitykit/src/base".to_string(),
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let builder = builder
                    .allowlist_file(format!("{}", header_path.to_str().unwrap()))
                    .result_error_enum("AbilityBase_ErrorCode")
                    .parse_callbacks(Box::new(ResultEnumParseCallbacks {
                        rename_item: Box::new(|name| {
                            name.strip_suffix("_ErrorCode").map(|name| {
                                let mut s = name.to_string();
                                s.push_str("Result");
                                s
                            })
                        }),
                        rename_enum_variant: None,
                    }));
                match file_stem {
                    "want" => builder
                        .raw_line("use crate::base::common::AbilityBaseResult;")
                        .clang_args(["-include", "stdbool.h"]),
                    _ => builder,
                }
            }),
            ..Default::default()
        },
        DirBindingsConf {
            directory: "AbilityKit/ability_runtime".to_string(),
            output_dir: "components/abilitykit/src/runtime".to_string(),
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let builder = builder
                    .allowlist_file(header_path.to_str().unwrap())
                    .result_error_enum("AbilityRuntime_ErrorCode")
                    .clang_args(["-include", "stdbool.h", "-x", "c++"])
                    .parse_callbacks(Box::new(ResultEnumParseCallbacks {
                        rename_item: Box::new(|name| {
                            name.strip_suffix("_ErrorCode").map(|name| {
                                let mut s = name.to_string();
                                s.push_str("Result");
                                s
                            })
                        }),
                        rename_enum_variant: None,
                    }));
                match file_stem {
                    "application_context" => builder
                        .raw_line(
                            "use crate::runtime::{AbilityRuntimeResult, AbilityRuntime_AreaMode};",
                        )
                        .raw_line("#[cfg(feature = \"api-15\")]")
                        .raw_line("use crate::base::want::AbilityBase_Want;")
                        .raw_line("#[cfg(feature = \"api-17\")]")
                        .raw_line("use crate::runtime::start_options::AbilityRuntime_StartOptions;")
                    ,
                    "start_options" => builder
                        .raw_line("use ohos_sys_opaque_types::OH_PixelmapNative;")
                        .raw_line("use crate::runtime::{AbilityRuntimeResult, AbilityRuntime_StartVisibility, AbilityRuntime_SupportedWindowMode, AbilityRuntime_WindowMode};")
                    ,
                    _ => builder,
                }
            }),
            ..Default::default()
        },
        DirBindingsConf {
            directory: "window_manager/".to_string(),
            output_dir: "components/window_manager/src".to_string(),
            rename_output_file: Some(Box::new(|stem| strip_prefix(stem, "oh_"))),
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let builder = builder
                    .allowlist_file(header_path.to_str().unwrap())
                    .blocklist_file(".*graphic_error_code.h")
                    .clang_args(["-include", "stdbool.h", "-include", "stddef.h"])
                    // oh_window_comm has a typedef without a name.
                    .clang_args(["-x", "c++"])
                    .result_error_enum("NativeDisplayManager_ErrorCode")
                    .result_error_enum("WindowManager_ErrorCode")
                    .parse_callbacks(Box::new(ResultEnumParseCallbacks {
                        rename_item: Box::new(|original_item_name| match original_item_name {
                            "NativeDisplayManager_ErrorCode" => {
                                Some("NativeDisplayManagerResult".to_string())
                            }
                            "WindowManager_ErrorCode" => Some("WindowManagerResult".to_string()),
                            _ => None,
                        }),
                        ..Default::default()
                    }));
                match file_stem {
                    "display_manager" => builder
                        .raw_line(
                            "use crate::display_info::NativeDisplayManagerResult;",
                        )
                        .raw_line(
                            "use crate::display_info::NativeDisplayManager_CutoutInfo;",
                        )
                        .raw_line("use crate::display_info::NativeDisplayManager_FoldDisplayMode;")
                        .raw_line("use crate::display_info::NativeDisplayManager_Orientation;")
                        .raw_line("use crate::display_info::NativeDisplayManager_Rotation;")
                        .raw_line("#[cfg(feature=\"api-14\")]")
                        .raw_line("use crate::display_info::NativeDisplayManager_DisplayInfo;")
                        .raw_line("#[cfg(feature=\"api-14\")]")
                        .raw_line("use crate::display_info::NativeDisplayManager_DisplaysInfo;")
                        .raw_line("#[cfg(feature=\"api-20\")]")
                        .raw_line("use crate::display_info::{NativeDisplayManager_Rect, NativeDisplayManager_SourceMode};")
                    ,
                    "display_capture" => builder
                        .raw_line("#[cfg(feature=\"api-12\")]")
                        .raw_line("use crate::display_info::NativeDisplayManagerResult;")
                        .raw_line("#[cfg(feature=\"api-14\")]")
                        .raw_line("use ohos_sys_opaque_types::OH_PixelmapNative;"),
                    "window_event_filter" => builder
                        .raw_line("use crate::window_comm::WindowManagerResult;")
                        .raw_line("#[cfg(feature = \"api-12\")]")
                        .raw_line("use ohos_sys_opaque_types::Input_KeyEvent;")
                        .raw_line("#[cfg(feature=\"api-15\")]")
                        .raw_line("use ohos_sys_opaque_types::{Input_MouseEvent, Input_TouchEvent};"),
                    "display_info" => builder.no_copy("^NativeDisplayManager_DisplayInfo"),
                    "window" => builder
                        .raw_line("use ohos_sys_opaque_types::OH_PixelmapNative;")
                        .raw_line("use crate::window_comm::{WindowManager_AvoidArea, WindowManager_AvoidAreaType, WindowManager_WindowProperties};")
                        .raw_line("#[cfg(feature=\"api-17\")]")
                        .raw_line("use crate::window_comm::WindowManager_Rect;")
                        .raw_line("#[cfg(feature=\"api-20\")]")
                        .raw_line("use ohos_sys_opaque_types::Input_TouchEvent;")
                        .raw_line("#[cfg(feature=\"api-21\")]")
                        .raw_line("use crate::window_comm::{WindowManager_MainWindowInfo, WindowManager_WindowSnapshotConfig};")
                    ,
                    _ => builder,
                }
            }),

            skip_files: vec!["graphic_error_code.h".to_string()],
            ..Default::default()
        },
        DirBindingsConf {
            directory: "sensors".to_string(),
            output_dir: "components/sensors/src".to_string(),
            rename_output_file: Some(Box::new(|stem| strip_prefix(stem, "oh_"))),
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let builder = builder
                    .allowlist_file(header_path.to_str().unwrap())
                    .clang_args(["-include", "stdbool.h"]);

                match file_stem {
                    "sensor" => builder.raw_line("use crate::sensor_type::*;"),
                    "vibrator" => builder.raw_line("use crate::vibrator_type::*;"),
                    _ => builder,
                }
            }),
            ..Default::default()
        },
        DirBindingsConf {
            directory: "IPCKit".to_string(),
            output_dir: "components/ipckit/src".to_string(),
            rename_output_file: Some(Box::new(|stem| strip_prefix(stem, "ipc_"))),
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let builder = builder.allowlist_file(header_path.to_str().unwrap());
                match file_stem {
                    "cparcel" => builder
                        .raw_line("use ohos_sys_opaque_types::{OHIPCParcel, OHIPCRemoteProxy};"),
                    "cremote_object" => builder
                        .raw_line("use crate::cparcel::{OHIPCRemoteStub, OH_IPC_MemAllocator};")
                        .raw_line("use ohos_sys_opaque_types::{OHIPCParcel, OHIPCRemoteProxy};"),
                    "cskeleton" => builder.raw_line("use crate::cparcel::OH_IPC_MemAllocator;"),
                    _ => builder,
                }
            }),
            skip_files: vec!["ipc_kit.h".to_string()],
            ..Default::default()
        },
        DirBindingsConf {
            directory: "network/netmanager".to_string(),
            output_dir: "components/netmanager/src".to_string(),
            rename_output_file: None,
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let builder = builder
                    .allowlist_file(header_path.to_str().unwrap())
                    .clang_args(["-include", "stdbool.h"]);
                match file_stem {
                    "net_connection" => builder
                        .raw_line("use crate::net_connection_type::*;")
                        .raw_line("use libc::addrinfo;"),
                    "net_connection_type" => builder.raw_line("use libc::addrinfo;"),
                    _ => builder,
                }
            }),
            ..Default::default()
        },
        DirBindingsConf {
            directory: "network/netstack".to_string(),
            output_dir: "components/netstack/src".to_string(),
            rename_output_file: None,
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let builder = builder
                    .allowlist_file(header_path.to_str().unwrap())
                    .clang_args([
                        "-include",
                        "stdbool.h",
                        "-include",
                        "stdint.h",
                        "-include",
                        "stddef.h",
                    ]);
                match file_stem {
                    "net_http" => builder.raw_line("use crate::net_http_type::*;"),
                    "net_http_type" => builder.blocklist_var("NET_HTTP_METHOD_PATCH").raw_line(
                        "// SDK currently defines NET_HTTP_METHOD_PATCH as \"CONNECT\"; \
                             blocklisted until this is re-evaluated against upstream headers.",
                    ),
                    "net_websocket" => builder
                        .raw_line("use crate::net_websocket_type::*;")
                        .clang_args(["-x", "c++"]),
                    "net_websocket_type" => builder.clang_args(["-x", "c++"]),
                    _ => builder,
                }
            }),
            known_nested_include_dirs: vec!["network/netstack/net_ssl".to_string()],
            ..Default::default()
        },
        DirBindingsConf {
            directory: "network/netstack/net_ssl".to_string(),
            output_dir: "components/net_ssl/src".to_string(),
            rename_output_file: None,
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let builder = builder
                    .allowlist_file(header_path.to_str().unwrap())
                    .clang_args([
                        "-include",
                        "stdbool.h",
                        "-include",
                        "stdint.h",
                        "-include",
                        "stddef.h",
                    ]);
                match file_stem {
                    "net_ssl_c" => builder.raw_line("use crate::net_ssl_c_type::*;"),
                    _ => builder,
                }
            }),
            ..Default::default()
        },
        DirBindingsConf {
            directory: "LocationKit".to_string(),
            output_dir: "components/locationkit/src".to_string(),
            rename_output_file: Some(Box::new(|stem| strip_prefix(stem, "oh_"))),
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let builder = builder
                    .allowlist_file(header_path.to_str().unwrap())
                    .clang_args(["-include", "stdbool.h"]);
                match file_stem {
                    "location" => builder.raw_line("use crate::location_type::*;"),
                    "location_type" => builder.result_error_enum("Location_ResultCode"),
                    _ => builder,
                }
            }),
            ..Default::default()
        },
        DirBindingsConf {
            directory: "web".to_string(),
            output_dir: "components/web/src".to_string(),
            rename_output_file: None,
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let builder = builder
                    .allowlist_file(header_path.to_str().unwrap())
                    .clang_args(["-include", "stdbool.h"]);

                match file_stem {
                    "arkweb_error_code" => builder.result_error_enum("ArkWeb_ErrorCode"),
                    "arkweb_interface" => builder
                        .raw_line("#[cfg(feature = \"api-18\")]\nuse crate::arkweb_type::ArkWeb_OnScrollCallback;"),
                    "arkweb_scheme_handler" => {
                        builder.raw_line("use crate::arkweb_net_error_list::ArkWeb_NetError;")
                    }
                    "arkweb_type" => builder
                        .raw_line("use crate::arkweb_error_code::ArkWeb_ErrorCode;")
                        // This type is referenced from an ungated field but is itself feature-gated by bindgen.
                        // Provide a fallback placeholder for lower API levels.
                        .raw_line("#[cfg(not(feature = \"api-18\"))]\n#[repr(C)]\npub struct ArkWeb_ProxyObjectWithResult { _unused: [u8; 0], }"),
                    "native_interface_arkweb" => builder
                        .raw_line("#[cfg(feature = \"api-15\")]\nuse crate::arkweb_error_code::ArkWeb_ErrorCode;")
                        .raw_line("#[cfg(feature = \"api-20\")]\nuse crate::arkweb_error_code::ArkWeb_BlanklessErrorCode;")
                        .raw_line("#[cfg(feature = \"api-20\")]\nuse crate::arkweb_type::ArkWeb_ProxyObjectWithResult;"),
                    _ => builder,
                }
            }),
            ..Default::default()
        },
        DirBindingsConf {
            directory: "huks".to_string(),
            output_dir: "components/huks/src".to_string(),
            rename_output_file: None,
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let builder = builder
                    .allowlist_file(header_path.to_str().unwrap())
                    .clang_args(["-include", "stdbool.h"]);
                match file_stem {
                    "native_huks_api" | "native_huks_param" => {
                        builder.raw_line("use crate::native_huks_type::*;")
                    }
                    "native_huks_external_crypto_api" => builder
                        .raw_line("use crate::native_huks_type::*;")
                        .raw_line("use crate::native_huks_external_crypto_type::*;"),
                    "native_huks_external_crypto_type" => {
                        builder.raw_line("use crate::native_huks_type::*;")
                    }
                    _ => builder,
                }
            }),
            ..Default::default()
        },
        DirBindingsConf {
            directory: "asset".to_string(),
            output_dir: "components/asset/src".to_string(),
            rename_output_file: None,
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let builder = builder
                    .allowlist_file(header_path.to_str().unwrap())
                    .clang_args(["-include", "stdbool.h"]);
                match file_stem {
                    "asset_api" => builder.raw_line("use crate::asset_type::*;"),
                    _ => builder,
                }
            }),
            ..Default::default()
        },
        DirBindingsConf {
            directory: "accesstoken".to_string(),
            output_dir: "components/accesstoken/src".to_string(),
            rename_output_file: None,
            set_builder_opts: Box::new(|_file_stem, header_path, builder| {
                builder
                    .allowlist_file(header_path.to_str().unwrap())
                    .clang_args(["-include", "stdbool.h"])
            }),
            ..Default::default()
        },
        DirBindingsConf {
            directory: "BasicServicesKit".to_string(),
            output_dir: "components/basic-services-kit/src".to_string(),
            rename_output_file: Some(Box::new(|stem| {
                strip_prefix(&strip_prefix(stem, "oh_"), "oh")
            })),
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let builder = builder
                    .allowlist_file(header_path.to_str().unwrap())
                    .clang_args(["-include", "stdbool.h"]);
                match file_stem {
                    "os_account" => builder.raw_line("use crate::os_account_common::*;"),
                    _ => builder,
                }
            }),
            ..Default::default()
        },
        DirBindingsConf {
            directory: "CryptoArchitectureKit".to_string(),
            output_dir: "components/crypto/src".to_string(),
            rename_output_file: Some(Box::new(|stem| strip_prefix(stem, "crypto_"))),
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let builder = builder
                    .allowlist_file(header_path.to_str().unwrap())
                    .result_error_enum("OH_Crypto_ErrCode")
                    .parse_callbacks(Box::new(ResultEnumParseCallbacks {
                        rename_item: Box::new(|name| match name {
                            "OH_Crypto_ErrCode" => Some("CryptoResult".to_string()),
                            _ => None,
                        }),
                        ..Default::default()
                    }));
                match file_stem {
                    "asym_cipher" => builder
                        .raw_line("use crate::common::{Crypto_CipherMode, Crypto_DataBlob, CryptoResult};")
                        .raw_line("use crate::asym_key::OH_CryptoKeyPair;"),
                    "asym_key" => builder
                        .raw_line("use crate::common::{Crypto_DataBlob, CryptoResult};"),
                    "digest" => builder
                        .raw_line("use crate::common::{Crypto_DataBlob, CryptoResult};"),
                    "kdf" => builder
                        .raw_line("use crate::common::{Crypto_DataBlob, CryptoResult};"),
                    "key_agreement" => builder
                        .raw_line("use crate::common::{Crypto_DataBlob, CryptoResult};")
                        .raw_line("use crate::asym_key::{OH_CryptoPrivKey, OH_CryptoPubKey};"),
                    "mac" => builder
                        .raw_line("use crate::common::{Crypto_DataBlob, CryptoResult};")
                        .raw_line("use crate::sym_key::OH_CryptoSymKey;"),
                    "rand" => builder
                        .raw_line("use crate::common::{Crypto_DataBlob, CryptoResult};"),
                    "signature" => builder
                        .raw_line("use crate::common::{Crypto_DataBlob, CryptoResult};")
                        .raw_line("use crate::asym_key::OH_CryptoPubKey;")
                        .raw_line("#[cfg(feature = \"api-20\")]")
                        .raw_line("use crate::asym_key::OH_CryptoPrivKey;"),
                    "sym_cipher" => builder
                        .raw_line("use crate::common::{Crypto_CipherMode, Crypto_DataBlob, CryptoResult};")
                        .raw_line("use crate::sym_key::OH_CryptoSymKey;"),
                    "sym_key" => builder
                        .raw_line("use crate::common::{Crypto_DataBlob, CryptoResult};"),
                    _ => builder,
                }
            }),
            // This header just includes all the other ones.
            skip_files: vec!["crypto_architecture_kit.h".to_string()],
            ..Default::default()
        },
        DirBindingsConf {
            directory: "ohaudio".to_string(),
            output_dir: "components/ohaudio/src".to_string(),
            rename_output_file: Some(Box::new(|stem| strip_prefix(stem, "native_"))),
            set_builder_opts: Box::new(|file_stem, header_path, builder| {
                let mut builder = builder
                    .allowlist_file(header_path.to_str().unwrap())
                    .clang_args(["-include", "stdbool.h"]);

                if file_stem == "audiostream_base" {
                    if let Some(include_dir) = header_path.parent().and_then(|path| path.parent()) {
                        let channel_layout_header =
                            include_dir.join("multimedia/native_audio_channel_layout.h");
                        if channel_layout_header.exists() {
                            builder =
                                builder.allowlist_file(channel_layout_header.to_str().unwrap());
                        }
                    }
                }

                match file_stem {
                    "audiocapturer" => builder
                        .raw_line("use crate::audiostream_base::*;")
                        .raw_line("use libc::clockid_t;")
                        .raw_line("#[cfg(feature = \"api-20\")]\nuse crate::audio_device_base::OH_AudioDeviceDescriptorArray;"),
                    "audiorenderer" => builder
                        .raw_line("use crate::audiostream_base::*;")
                        .raw_line("use libc::clockid_t;")
                        .raw_line("#[cfg(feature = \"api-12\")]\nuse crate::audio_device_base::OH_AudioDevice_Type;"),
                    "audiostreambuilder" => builder
                        .raw_line("use crate::audiostream_base::*;")
                        .raw_line(
                            "#[cfg(feature = \"api-20\")]\nuse crate::audiocapturer::{OH_AudioCapturer_OnDeviceChangeCallback, OH_AudioCapturer_OnErrorCallback, OH_AudioCapturer_OnFastStatusChange, OH_AudioCapturer_OnInterruptCallback, OH_AudioCapturer_OnReadDataCallback};",
                        )
                        .raw_line(
                            "#[cfg(feature = \"api-20\")]\nuse crate::audiorenderer::{OH_AudioRenderer_OnErrorCallback, OH_AudioRenderer_OnFastStatusChange, OH_AudioRenderer_OnInterruptCallback, OH_AudioRenderer_OnWriteDataCallbackAdvanced};",
                        ),
                    "audio_common" => builder.result_error_enum("OH_AudioCommon_Result"),
                    "audiostream_base" => builder.result_error_enum("OH_AudioStream_Result"),
                    "audio_device_base" => builder
                        .raw_line("use crate::audio_common::OH_AudioCommon_Result;")
                        .raw_line("use crate::audiostream_base::OH_AudioStream_EncodingType;"),
                    "audio_manager" => builder
                        .raw_line("use crate::audio_common::{OH_AudioCommon_Result, OH_AudioScene};"),
                    "audio_routing_manager" => builder
                        .raw_line("use crate::audio_common::OH_AudioCommon_Result;")
                        .raw_line("use crate::audio_device_base::{OH_AudioDeviceDescriptorArray, OH_AudioDevice_ChangeType, OH_AudioDevice_Flag, OH_AudioDevice_Usage};")
                        .raw_line("#[cfg(feature = \"api-13\")]\nuse crate::audio_device_base::OH_AudioDevice_BlockStatus;")
                        .raw_line("use crate::audiostream_base::{OH_AudioStream_Usage, OH_AudioStream_SourceType};"),
                    "audio_session_manager" => builder
                        .raw_line("use crate::audio_common::OH_AudioCommon_Result;")
                        .raw_line("#[cfg(feature = \"api-20\")]\nuse crate::audio_device_base::{OH_AudioDeviceDescriptorArray, OH_AudioDevice_Type};")
                        .raw_line("#[cfg(feature = \"api-21\")]\nuse crate::audio_device_base::{OH_AudioDevice_ChangeType, OH_AudioDeviceDescriptor, OH_AudioDevice_Usage};")
                        .raw_line("#[cfg(feature = \"api-20\")]\nuse crate::audiostream_base::OH_AudioStream_DeviceChangeReason;"),
                    "audio_stream_manager" => builder
                        .raw_line("use crate::audio_common::OH_AudioCommon_Result;")
                        .raw_line("use crate::audiostream_base::{OH_AudioStreamInfo, OH_AudioStream_Usage, OH_AudioStream_DirectPlaybackMode, OH_AudioStream_SourceType};"),
                    "audio_resource_manager" => builder
                        .raw_line("use crate::audio_common::OH_AudioCommon_Result;"),
                    "audio_volume_manager" => builder
                        .raw_line("use crate::audio_common::{OH_AudioCommon_Result, OH_AudioRingerMode};")
                        .raw_line("use crate::audiostream_base::OH_AudioStream_Usage;"),
                    _ => builder,
                }
            }),
            ..Default::default()
        },
    ]
}
