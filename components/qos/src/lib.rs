//! Bindings to the OpenHarmony QoS (Quality of Service) thread scheduling APIs.
//!
//! See also the [QoS development guide](https://docs.openharmony.cn/pages/v5.0/en/application-dev/napi/qos-guidelines.md)
//! and the [Gewu development guide](https://docs.openharmony.cn/pages/v5.0/en/application-dev/napi/gewu-ndk-api-guidelines.md)
//! for the on-device LLM inference surface that ships in the same library.
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-12")]
#[link(name = "qos")]
unsafe extern "C" {}

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
mod qos_ffi;
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub use qos_ffi::*;
