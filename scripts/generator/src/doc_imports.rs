//! `#[cfg(doc)]` module-glob imports so rustdoc can resolve intra-doc links
//! to items that live in other modules of the same crate.
//!
//! The C headers refer to types and functions by unqualified name, but the
//! bindings are split across files. Import the whole sibling module under
//! `#[cfg(doc)]` (plus that module's own cfg, if any) so rustdoc can resolve
//! the links without listing individual items. See openharmony-rs/ohos-sys#71.

use bindgen::Builder;

pub(crate) fn apply_doc_imports(builder: Builder, output_rel: &str) -> Builder {
    let imports = doc_imports_for(output_rel);
    let mut builder = builder;
    for line in imports {
        builder = builder.raw_line(*line);
    }
    builder
}

fn doc_imports_for(output_rel: &str) -> &'static [&'static str] {
    match output_rel {
        "components/arkui/src/drag_and_drop/drag_and_drop_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::ui_input_event::*;",
        ],
        "components/arkui/src/native_node/native_node_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::drag_and_drop::*;",
            "#[cfg(doc)]",
            "use crate::drawable_descriptor::*;",
            "#[cfg(doc)]",
            "use crate::native_animate::*;",
            "#[cfg(all(doc, feature = \"api-20\"))]",
            "use crate::native_render::*;",
            "#[cfg(doc)]",
            "use crate::styled_string::*;",
            "#[cfg(doc)]",
            "use crate::ui_input_event::*;",
        ],
        "components/arkui/src/ui_input_event/ui_input_event_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::native_gesture::*;",
            "#[cfg(all(doc, feature = \"api-14\"))]",
            "use crate::native_key_event::*;",
            "#[cfg(doc)]",
            "use crate::native_node::*;",
        ],
        "components/basic-services-kit/src/scan/scan_ffi.rs" => &[
            "#[cfg(all(doc, feature = \"print\"))]",
            "use crate::print::*;",
        ],
        "components/inputmethod/src/inputmethod_proxy/inputmethod_proxy_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::controller::*;",
        ],
        "components/inputmethod/src/text_editor_proxy/text_editor_proxy_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::controller::*;",
        ],
        "components/ipckit/src/cparcel/cparcel_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::error_code::*;",
        ],
        "components/ipckit/src/cremote_object/cremote_object_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::error_code::*;",
        ],
        "components/ipckit/src/cskeleton/cskeleton_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::error_code::*;",
        ],
        "components/multimedia/player_framework/src/avbuffer/avbuffer_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::avformat::*;",
        ],
        "components/multimedia/player_framework/src/avcodec_audiocodec/avcodec_audiocodec_ffi.rs" => {
            &[
                "#[cfg(doc)]",
                "use crate::avcodec_base::*;",
                "#[cfg(doc)]",
                "use crate::avformat::*;",
            ]
        }
        "components/multimedia/player_framework/src/avcodec_audiodecoder/avcodec_audiodecoder_ffi.rs" => {
            &["#[cfg(doc)]", "use crate::avcodec_base::*;"]
        }
        "components/multimedia/player_framework/src/avcodec_audioencoder/avcodec_audioencoder_ffi.rs" => {
            &["#[cfg(doc)]", "use crate::avcodec_base::*;"]
        }
        "components/multimedia/player_framework/src/avcodec_base/avcodec_base_ffi.rs" => &[
            "#[cfg(all(doc, feature = \"api-11\"))]",
            "use crate::avbuffer::*;",
            "#[cfg(all(doc, feature = \"api-10\"))]",
            "use crate::avcapability::*;",
            "#[cfg(doc)]",
            "use crate::avcodec_videodecoder::*;",
            "#[cfg(doc)]",
            "use crate::avcodec_videoencoder::*;",
            "#[cfg(doc)]",
            "use crate::avformat::*;",
        ],
        "components/multimedia/player_framework/src/avcodec_videodecoder/avcodec_videodecoder_ffi.rs" => {
            &["#[cfg(doc)]", "use crate::avcodec_base::*;"]
        }
        "components/multimedia/player_framework/src/avcodec_videoencoder/avcodec_videoencoder_ffi.rs" => {
            &["#[cfg(doc)]", "use crate::avcodec_base::*;"]
        }
        "components/multimedia/player_framework/src/averrors/averrors_ffi.rs" => &[
            "#[cfg(all(doc, feature = \"api-11\"))]",
            "use crate::avcodec_audiocodec::*;",
            "#[cfg(doc)]",
            "use crate::avcodec_videodecoder::*;",
            "#[cfg(doc)]",
            "use crate::avcodec_videoencoder::*;",
        ],
        "components/multimedia/player_framework/src/avmetadata_extractor/avmetadata_extractor_ffi.rs" => {
            &["#[cfg(doc)]", "use crate::avformat::*;"]
        }
        "components/multimedia/player_framework/src/avmetadata_extractor_base/avmetadata_extractor_base_ffi.rs" => {
            &[
                "#[cfg(all(doc, feature = \"api-18\"))]",
                "use crate::media_types::*;",
            ]
        }
        "components/multimedia/player_framework/src/avplayer_base/avplayer_base_ffi.rs" => &[
            "#[cfg(all(doc, feature = \"api-11\"))]",
            "use crate::avplayer::*;",
        ],
        "components/multimedia/video_processing_engine/src/image_processing_types/image_processing_types_ffi.rs" => {
            &[
                "#[cfg(doc)]",
                "use crate::image_processing::*;",
                "#[cfg(all(doc, feature = \"video-processing\"))]",
                "use crate::video_processing::*;",
            ]
        }
        "components/multimedia/video_processing_engine/src/video_processing_types/video_processing_types_ffi.rs" => {
            &[
                "#[cfg(all(doc, feature = \"api-13\", feature = \"image-processing\"))]",
                "use crate::image_processing::*;",
                "#[cfg(doc)]",
                "use crate::video_processing::*;",
            ]
        }
        "components/ohaudio/src/audio_device_base/audio_device_base_ffi.rs" => &[
            "#[cfg(all(doc, feature = \"api-12\"))]",
            "use crate::audio_routing_manager::*;",
        ],
        "components/ohaudio/src/audio_resource_manager/audio_resource_manager_ffi.rs" => &[
            "#[cfg(all(doc, feature = \"api-12\"))]",
            "use crate::audio_routing_manager::*;",
        ],
        "components/pasteboard/src/pasteboard/pasteboard_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::pasteboard_err_code::*;",
        ],
        "components/rdb/src/data_asset/data_asset_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::relational_store_error_code::*;",
        ],
        "components/rdb/src/rdb_transaction/rdb_transaction_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::relational_store_error_code::*;",
        ],
        "components/rdb/src/relational_store/relational_store_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::rdb_types::*;",
            "#[cfg(doc)]",
            "use crate::relational_store_error_code::*;",
        ],
        "components/udmf/src/udmf/udmf_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::udmf_err_code::*;",
        ],
        "components/udmf/src/uds/uds_ffi.rs" => &[
            "#[cfg(doc)]",
            "use crate::udmf_err_code::*;",
        ],
        "components/xcomponent/src/xcomponent_ffi.rs" => &[
            "#[cfg(all(doc, feature = \"arkui\"))]",
            "use arkui_sys::ui_input_event::*;",
        ],
        _ => &[],
    }
}
