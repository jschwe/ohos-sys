# OpenHarmony VideoProcessingEngine bindings

Raw FFI bindings to the OpenHarmony VideoProcessingEngine subsystem:

- `multimedia/video_processing_engine/video_processing.h`
  (`libvideo_processing.so`) — color space conversion, HDR vivid metadata generation,
  and video scaling / detail enhancement. Available since API-level 12.
- `multimedia/video_processing_engine/image_processing.h`
  (`libimage_processing.so`) — color space conversion, HDR composition / decomposition,
  metadata generation, and detail enhancement for `OH_PixelmapNative` images.
  Available since API-level 13.

These two libraries ship in the same SDK subdirectory and share the same syscap
(`SystemCapability.Multimedia.VideoProcessingEngine`), but they are documented
under different "kits" (Media Kit and Image Kit respectively) and link against
separate shared libraries.

C API references:

- Video processing: <https://gitcode.com/openharmony/docs/blob/master/en/application-dev/reference/apis-media-kit/capi-video-processing-h.md>
- Image processing: <https://gitcode.com/openharmony/docs/blob/master/en/application-dev/reference/apis-image-kit/capi-image-processing-h.md>

## Features

Pick the desired `api-XX` feature and one or both of `video-processing` /
`image-processing`. Enabling no module produces an empty crate (useful for
feature composability in the umbrella `ohos-sys` crate).

## License

Licensed under the Apache-2.0 license, matching the license of OpenHarmony.
