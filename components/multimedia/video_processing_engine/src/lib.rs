//! Bindings to the OpenHarmony VideoProcessingEngine.
//!
//! Provides GPU-accelerated color space conversion, HDR metadata generation,
//! composition / decomposition, and detail enhancement for both video streams
//! and individual pixmap images.
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(all(feature = "api-12", feature = "video-processing"))]
#[cfg_attr(
    docsrs,
    doc(cfg(all(feature = "api-12", feature = "video-processing")))
)]
pub mod video_processing;
#[cfg(all(feature = "api-12", feature = "video-processing"))]
#[cfg_attr(
    docsrs,
    doc(cfg(all(feature = "api-12", feature = "video-processing")))
)]
pub mod video_processing_types;

#[cfg(all(feature = "api-13", feature = "image-processing"))]
#[cfg_attr(
    docsrs,
    doc(cfg(all(feature = "api-13", feature = "image-processing")))
)]
pub mod image_processing;
#[cfg(all(feature = "api-13", feature = "image-processing"))]
#[cfg_attr(
    docsrs,
    doc(cfg(all(feature = "api-13", feature = "image-processing")))
)]
pub mod image_processing_types;
