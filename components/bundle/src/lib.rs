//! Bindings to the OpenHarmony BundleManager (native bundle) NDK.
//!
//! ## Overview
//!
//! BundleManager exposes the calling application's bundle metadata —
//! `bundleName`, `appId`, `appIdentifier`, signing fingerprint, compatible
//! device type, debug-mode flag, the main element name (bundle + module +
//! ability), and per-module metadata — plus a file-type intent discovery flow
//! (`OH_NativeBundle_GetAbilityResourceInfo`, available from API-21).
//!
//! All entry points are gated on the `api-9` feature (`@since 9` at the
//! header level); individual symbols added in later API levels are gated
//! further by bindgen via the upstream `availability` attributes. The
//! `ability_resource_info` and `bundle_manager_common` modules are gated on
//! `api-21`.
//!
//! See also the [official Native_Bundle documentation](https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-ability-kit/capi-native-interface-bundle-h.md).
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod native_interface_bundle;

#[cfg(feature = "api-21")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-21")))]
pub mod ability_resource_info;

#[cfg(feature = "api-21")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-21")))]
pub mod bundle_manager_common;

#[link(name = "bundle_ndk.z")]
unsafe extern "C" {}
