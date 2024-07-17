/* automatically generated by rust-bindgen 0.69.4 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::drawing::types::*;

extern "C" {
    /** @brief Creates an <b>OH_Drawing_ShadowLayer</b> object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param blurRadius Indicates the blur radius of the shadow.
    @param x Indicates the offset point on x-axis.
    @param y Indicates the offset point on y-axis.
    @param color Indicates the shadow color.
    @return Returns the pointer to the <b>OH_Drawing_ShadowLayer</b> object created.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_ShadowLayerCreate(
        blurRadius: f32,
        x: f32,
        y: f32,
        color: u32,
    ) -> *mut OH_Drawing_ShadowLayer;
    /** @brief Destroys an <b>OH_Drawing_ShadowLayer</b> object and reclaims the memory occupied by the object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_ShadowLayer Indicates the pointer to an <b>OH_Drawing_ShadowLayer</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_ShadowLayerDestroy(arg1: *mut OH_Drawing_ShadowLayer);
}