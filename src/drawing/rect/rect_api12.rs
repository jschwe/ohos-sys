/* automatically generated by rust-bindgen 0.69.4 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::drawing::types::*;

extern "C" {
    /** @brief Creates an <b>OH_Drawing_Rect</b> object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param left Indicates the left position of the rect.
    @param top Indicates the top position of the rect.
    @param right Indicates the right position of the rect.
    @param bottom Indicates the bottom position of the rect.
    @return Returns the pointer to the <b>OH_Drawing_Rect</b> object created.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_RectCreate(
        left: f32,
        top: f32,
        right: f32,
        bottom: f32,
    ) -> *mut OH_Drawing_Rect;
    /** @brief If rect intersects other, sets rect to intersection.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param rect Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    @param other Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    @return Returns true if have area in common.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_RectIntersect(
        rect: *mut OH_Drawing_Rect,
        other: *const OH_Drawing_Rect,
    ) -> bool;
    /** @brief Sets rect to the union of rect and other.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param rect Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    @param other Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    @return Returns true if rect and other are not nullptr, and other is not empty;
            false if rect or other is nullptr, or other is empty.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_RectJoin(rect: *mut OH_Drawing_Rect, other: *const OH_Drawing_Rect) -> bool;
    /** @brief Set the left position of the rect.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param rect Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    @param left Indicates the left position of the rect.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_RectSetLeft(rect: *mut OH_Drawing_Rect, left: f32);
    /** @brief Set the top position of the rect.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param rect Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    @param top Indicates the top position of the rect.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_RectSetTop(rect: *mut OH_Drawing_Rect, top: f32);
    /** @brief Set the right position of the rect.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param rect Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    @param right Indicates the right position of the rect.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_RectSetRight(rect: *mut OH_Drawing_Rect, right: f32);
    /** @brief Set the bottom position of the rect.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param rect Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    @param bottom Indicates the bottom position of the rect.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_RectSetBottom(rect: *mut OH_Drawing_Rect, bottom: f32);
    /** @brief Get the left position of the rect.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Rect Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    @return Return the left position of the rect.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_RectGetLeft(arg1: *mut OH_Drawing_Rect) -> f32;
    /** @brief Get the top position of the rect.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Rect Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    @return Return the top position of the rect.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_RectGetTop(arg1: *mut OH_Drawing_Rect) -> f32;
    /** @brief Get the right position of the rect.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Rect Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    @return Return the right position of the rect.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_RectGetRight(arg1: *mut OH_Drawing_Rect) -> f32;
    /** @brief Get the bottom position of the rect.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Rect Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    @return Return the bottom position of the rect.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_RectGetBottom(arg1: *mut OH_Drawing_Rect) -> f32;
    /** @brief Get the height position of the rect.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Rect Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_RectGetHeight(arg1: *mut OH_Drawing_Rect) -> f32;
    pub fn OH_Drawing_RectGetWidth(arg1: *mut OH_Drawing_Rect) -> f32;
    /** @brief Copy the original rectangular object to the destination rectangular object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param src Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    @param dst Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_RectCopy(src: *mut OH_Drawing_Rect, dst: *mut OH_Drawing_Rect);
    /** @brief Destroys an <b>OH_Drawing_Rect</b> object and reclaims the memory occupied by the object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Rect Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    @since 11
    @version 1.0*/
    pub fn OH_Drawing_RectDestroy(arg1: *mut OH_Drawing_Rect);
}
