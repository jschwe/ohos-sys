/* automatically generated by rust-bindgen 0.69.4 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::drawing::types::*;

extern "C" {
    /** @brief Converts four variables (alpha, red, green, and blue) into a 32-bit (ARGB) variable that describes a color.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param alpha Indicates a variable that describes alpha. The value ranges from 0x00 to 0xFF.
    @param red Indicates a variable that describes red. The value ranges from 0x00 to 0xFF.
    @param green Indicates a variable that describes green. The value ranges from 0x00 to 0xFF.
    @param blue Indicates a variable that describes blue. The value ranges from 0x00 to 0xFF.
    @return Returns a 32-bit (ARGB) variable that describes the color.
    @since 8
    @version 1.0*/
    pub fn OH_Drawing_ColorSetArgb(alpha: u32, red: u32, green: u32, blue: u32) -> u32;
}
