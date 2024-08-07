/* automatically generated by rust-bindgen 0.69.4 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::types::*;

extern "C" {
    /** @brief Creates an <b>OH_Drawing_Point</b> object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param x Indicates the x-axis coordinates of the point.
    @param y Indicates the y-axis coordinates of the point.
    @return Returns the pointer to the <b>OH_Drawing_Point</b> object created.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_PointCreate(x: f32, y: f32) -> *mut OH_Drawing_Point;
    /** @brief Destroys an <b>OH_Drawing_Point</b> object and reclaims the memory occupied by the object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Point Indicates the pointer to an <b>OH_Drawing_Point</b> object.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_PointDestroy(arg1: *mut OH_Drawing_Point);
}
