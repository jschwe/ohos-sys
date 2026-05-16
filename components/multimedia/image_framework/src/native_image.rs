//! Bindings to [Image_NativeModule] on OpenHarmony
//!
//! [Image_NativeModule]: https://docs.openharmony.cn/pages/v5.0/en/application-dev/media/image/image-structure-c.md

pub mod common;
pub mod image;
#[cfg(feature = "image-packer")]
#[cfg_attr(docsrs, doc(cfg(feature = "image-packer")))]
pub mod image_packer;
#[cfg(feature = "image-receiver")]
#[cfg_attr(docsrs, doc(cfg(feature = "image-receiver")))]
pub mod image_receiver;
#[cfg(feature = "image-source")]
#[cfg_attr(docsrs, doc(cfg(feature = "image-source")))]
pub mod image_source;
#[cfg(all(feature = "api-13", feature = "picture"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "api-13", feature = "picture"))))]
pub mod picture;
#[cfg(feature = "pixelmap")]
#[cfg_attr(docsrs, doc(cfg(feature = "pixelmap")))]
pub mod pixelmap;
