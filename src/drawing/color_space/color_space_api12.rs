/* automatically generated by rust-bindgen 0.69.4 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::drawing::types::*;

extern "C" {
    /** @brief Creates an <b>OH_Drawing_ColorSpace</b> object that represents the SRGB color space.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @return Returns the pointer to the <b>OH_Drawing_ColorSpace</b> object created.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_ColorSpaceCreateSrgb() -> *mut OH_Drawing_ColorSpace;
    /** @brief Creates an <b>OH_Drawing_ColorSpace</b> object with the SRGB primaries, but a linear (1.0) gamma.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @return Returns the pointer to the <b>OH_Drawing_ColorSpace</b> object created.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_ColorSpaceCreateSrgbLinear() -> *mut OH_Drawing_ColorSpace;
    /** @brief Destroy an <b>OH_Drawing_ColorSpace</b> object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_ColorSpace Indicates the pointer to an <b>OH_Drawing_ColorSpace</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_ColorSpaceDestroy(arg1: *mut OH_Drawing_ColorSpace);
}
