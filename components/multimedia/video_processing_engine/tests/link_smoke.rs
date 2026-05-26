use std::ptr;

use ohos_video_processing_engine_sys as vpe;

#[test]
fn link_smoke() {
    #[cfg(all(feature = "api-12", feature = "video-processing"))]
    unsafe {
        // libvideo_processing.so
        let _ = vpe::video_processing::OH_VideoProcessing_InitializeEnvironment();
        let _ = vpe::video_processing::OH_VideoProcessing_DeinitializeEnvironment();
        let mut vp: *mut vpe::video_processing_types::OH_VideoProcessing = ptr::null_mut();
        let _ = vpe::video_processing::OH_VideoProcessing_Create(&mut vp, 0);
    }

    #[cfg(all(feature = "api-13", feature = "image-processing"))]
    unsafe {
        // libimage_processing.so
        let _ = vpe::image_processing::OH_ImageProcessing_InitializeEnvironment();
        let _ = vpe::image_processing::OH_ImageProcessing_DeinitializeEnvironment();
        let mut ip: *mut vpe::image_processing_types::OH_ImageProcessing = ptr::null_mut();
        let _ = vpe::image_processing::OH_ImageProcessing_Create(&mut ip, 0);
    }
}
