/* automatically generated by rust-bindgen 0.69.4 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::types::*;

extern "C" {
    /** @brief Creates an <b>OH_Drawing_Filter</b> object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @return Returns the pointer to the <b>OH_Drawing_Filter</b> object created.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_FilterCreate() -> *mut OH_Drawing_Filter;
    /** @brief Sets an <b>OH_Drawing_MaskFilter</b> object for an <b>OH_Drawing_Filter</b> object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Filter Indicates the pointer to an <b>OH_Drawing_Filter</b> object.
    @param OH_Drawing_MaskFilter Indicates the pointer to an <b>OH_Drawing_MaskFilter</b> object.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_FilterSetMaskFilter(
        arg1: *mut OH_Drawing_Filter,
        arg2: *mut OH_Drawing_MaskFilter,
    );
    /** @brief Sets an <b>OH_Drawing_ColorFilter</b> object for an <b>OH_Drawing_Filter</b> object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Filter Indicates the pointer to an <b>OH_Drawing_Filter</b> object.
    @param OH_Drawing_ColorFilter Indicates the pointer to an <b>OH_Drawing_ColorFilter</b> object.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_FilterSetColorFilter(
        arg1: *mut OH_Drawing_Filter,
        arg2: *mut OH_Drawing_ColorFilter,
    );
    /** @brief Destroys an <b>OH_Drawing_Filter</b> object and reclaims the memory occupied by the object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Filter Indicates the pointer to an <b>OH_Drawing_Filter</b> object.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_FilterDestroy(arg1: *mut OH_Drawing_Filter);
}
