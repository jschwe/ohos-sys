/* automatically generated by rust-bindgen 0.69.4 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::drawing::types::*;

extern "C" {
    /** @brief Creates an <b>OH_Drawing_Image</b> object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @return Returns the pointer to the <b>OH_Drawing_Image</b> object created.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_ImageCreate() -> *mut OH_Drawing_Image;
    /** @brief Destroys an <b>OH_Drawing_Image</b> object and reclaims the memory occupied by the object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Image Indicates the pointer to an <b>OH_Drawing_Image</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_ImageDestroy(arg1: *mut OH_Drawing_Image);
    /** @brief Rebuilds an <b>OH_Drawing_Image</b> object, sharing or copying bitmap pixels.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Image Indicates the pointer to an <b>OH_Drawing_Image</b> object.
    @param OH_Drawing_Bitmap Indicates the pointer to an <b>OH_Drawing_Bitmap</b> object.
    @return Returns true if successed.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_ImageBuildFromBitmap(
        arg1: *mut OH_Drawing_Image,
        arg2: *mut OH_Drawing_Bitmap,
    ) -> bool;
    /** @brief Gets pixel count in each row of image.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Image Indicates the pointer to an <b>OH_Drawing_Image</b> object.
    @return Returns the width.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_ImageGetWidth(arg1: *mut OH_Drawing_Image) -> i32;
    /** @brief Gets pixel row count of image.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Image Indicates the pointer to an <b>OH_Drawing_Image</b> object.
    @return Returns the height.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_ImageGetHeight(arg1: *mut OH_Drawing_Image) -> i32;
    /** @brief Gets the image info.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Image Indicates the pointer to an <b>OH_Drawing_Image</b> object.
    @param OH_Drawing_Image_Info Indicates the pointer to an <b>OH_Drawing_Image_Info</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_ImageGetImageInfo(
        arg1: *mut OH_Drawing_Image,
        arg2: *mut OH_Drawing_Image_Info,
    );
}
