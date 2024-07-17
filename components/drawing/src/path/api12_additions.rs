#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::types::{
    OH_Drawing_Matrix, OH_Drawing_Path, OH_Drawing_Point2D, OH_Drawing_Rect, OH_Drawing_RoundRect,
};

impl OH_Drawing_PathDirection {
    /// clockwise direction for adding closed contours
    pub const PATH_DIRECTION_CW: OH_Drawing_PathDirection = OH_Drawing_PathDirection(0);
}
impl OH_Drawing_PathDirection {
    /// counter-clockwise direction for adding closed contours
    pub const PATH_DIRECTION_CCW: OH_Drawing_PathDirection = OH_Drawing_PathDirection(1);
}
#[repr(transparent)]
/** @brief Direction for adding closed contours.

@since 12
@version 1.0*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_PathDirection(pub ::core::ffi::c_uint);
impl OH_Drawing_PathFillType {
    /// Specifies that "inside" is computed by a non-zero sum of signed edge crossings
    pub const PATH_FILL_TYPE_WINDING: OH_Drawing_PathFillType = OH_Drawing_PathFillType(0);
}
impl OH_Drawing_PathFillType {
    /// Specifies that "inside" is computed by an odd number of edge crossings
    pub const PATH_FILL_TYPE_EVEN_ODD: OH_Drawing_PathFillType = OH_Drawing_PathFillType(1);
}
impl OH_Drawing_PathFillType {
    /// Same as Winding, but draws outside of the path, rather than inside
    pub const PATH_FILL_TYPE_INVERSE_WINDING: OH_Drawing_PathFillType = OH_Drawing_PathFillType(2);
}
impl OH_Drawing_PathFillType {
    /// Same as EvenOdd, but draws outside of the path, rather than inside
    pub const PATH_FILL_TYPE_INVERSE_EVEN_ODD: OH_Drawing_PathFillType = OH_Drawing_PathFillType(3);
}
#[repr(transparent)]
/** @brief FillType of path.

@since 12
@version 1.0*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_PathFillType(pub ::core::ffi::c_uint);
impl OH_Drawing_PathAddMode {
    /// Appended to destination unaltered
    pub const PATH_ADD_MODE_APPEND: OH_Drawing_PathAddMode = OH_Drawing_PathAddMode(0);
}
impl OH_Drawing_PathAddMode {
    /// Add line if prior contour is not closed
    pub const PATH_ADD_MODE_EXTEND: OH_Drawing_PathAddMode = OH_Drawing_PathAddMode(1);
}
#[repr(transparent)]
/** @brief Add mode of path.

@since 12
@version 1.0*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_PathAddMode(pub ::core::ffi::c_uint);
impl OH_Drawing_PathOpMode {
    /// Difference operation.
    pub const PATH_OP_MODE_DIFFERENCE: OH_Drawing_PathOpMode = OH_Drawing_PathOpMode(0);
}
impl OH_Drawing_PathOpMode {
    /// Intersect operation.
    pub const PATH_OP_MODE_INTERSECT: OH_Drawing_PathOpMode = OH_Drawing_PathOpMode(1);
}
impl OH_Drawing_PathOpMode {
    /// Union operation.
    pub const PATH_OP_MODE_UNION: OH_Drawing_PathOpMode = OH_Drawing_PathOpMode(2);
}
impl OH_Drawing_PathOpMode {
    /// Xor operation.
    pub const PATH_OP_MODE_XOR: OH_Drawing_PathOpMode = OH_Drawing_PathOpMode(3);
}
impl OH_Drawing_PathOpMode {
    /// Reverse difference operation.
    pub const PATH_OP_MODE_REVERSE_DIFFERENCE: OH_Drawing_PathOpMode = OH_Drawing_PathOpMode(4);
}
#[repr(transparent)]
/** @brief Operations when two paths are combined.

@since 12
@version 1.0*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_PathOpMode(pub ::core::ffi::c_uint);
impl OH_Drawing_PathMeasureMatrixFlags {
    /// Gets position.
    pub const GET_POSITION_MATRIX: OH_Drawing_PathMeasureMatrixFlags =
        OH_Drawing_PathMeasureMatrixFlags(0);
}
impl OH_Drawing_PathMeasureMatrixFlags {
    /// Gets tangent.
    pub const GET_TANGENT_MATRIX: OH_Drawing_PathMeasureMatrixFlags =
        OH_Drawing_PathMeasureMatrixFlags(1);
}
impl OH_Drawing_PathMeasureMatrixFlags {
    /// Gets both position and tangent.
    pub const GET_POSITION_AND_TANGENT_MATRIX: OH_Drawing_PathMeasureMatrixFlags =
        OH_Drawing_PathMeasureMatrixFlags(2);
}
#[repr(transparent)]
/** @brief Enumerates the matrix information corresponding to the path measurements.

@since 12
@version 1.0*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OH_Drawing_PathMeasureMatrixFlags(pub ::core::ffi::c_uint);

extern "C" {
    /** @brief Creates an <b>OH_Drawing_Path</b> copy object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Path Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    @return Returns the pointer to the <b>OH_Drawing_Path</b> object created.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PathCopy(arg1: *mut OH_Drawing_Path) -> *mut OH_Drawing_Path;
    /** @brief Draws a conic from the last point of a path to the target point.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param ctrlX Indicates the x coordinate of the control point.
    @param ctrlY Indicates the y coordinate of the control point.
    @param endX Indicates the x coordinate of the target point.
    @param endY Indicates the y coordinate of the target point.
    @param weight Indicates the weight of added conic.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PathConicTo(
        arg1: *mut OH_Drawing_Path,
        ctrlX: f32,
        ctrlY: f32,
        endX: f32,
        endY: f32,
        weight: f32,
    );
    /** @brief Sets the relative starting point of a path.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param x Indicates the x coordinate of the relative starting point.
    @param y Indicates the y coordinate of the relative starting point.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PathRMoveTo(arg1: *mut OH_Drawing_Path, x: f32, y: f32);
    /** @brief Draws a line segment from the last point of a path to the relative target point.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param x Indicates the x coordinate of the relative target point.
    @param y Indicates the y coordinate of the relative target point.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PathRLineTo(arg1: *mut OH_Drawing_Path, x: f32, y: f32);
    /** @brief Draws a quadratic bezier curve from the last point of a path to the relative target point.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param ctrlX Indicates the x coordinate of the relative control point.
    @param ctrlY Indicates the y coordinate of the relative control point.
    @param endX Indicates the x coordinate of the relative target point.
    @param endY Indicates the y coordinate of the relative target point.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PathRQuadTo(
        arg1: *mut OH_Drawing_Path,
        ctrlX: f32,
        ctrlY: f32,
        endX: f32,
        endY: f32,
    );
    /** @brief Draws a conic from the last point of a path to the relative target point.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param ctrlX Indicates the x coordinate of the relative control point.
    @param ctrlY Indicates the y coordinate of the relative control point.
    @param endX Indicates the x coordinate of the relative target point.
    @param endY Indicates the y coordinate of the relative target point.
    @param weight Indicates the weight of added conic.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PathRConicTo(
        arg1: *mut OH_Drawing_Path,
        ctrlX: f32,
        ctrlY: f32,
        endX: f32,
        endY: f32,
        weight: f32,
    );
    /** @brief Draws a cubic bezier curve from the last point of a path to the relative target point.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param ctrlX1 Indicates the x coordinate of the first relative control point.
    @param ctrlY1 Indicates the y coordinate of the first relative control point.
    @param ctrlX2 Indicates the x coordinate of the second relative control point.
    @param ctrlY2 Indicates the y coordinate of the second relative control point.
    @param endX Indicates the x coordinate of the relative target point.
    @param endY Indicates the y coordinate of the relative target point.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PathRCubicTo(
        arg1: *mut OH_Drawing_Path,
        ctrlX1: f32,
        ctrlY1: f32,
        ctrlX2: f32,
        ctrlY2: f32,
        endX: f32,
        endY: f32,
    );
    /** @brief Adds a new contour to the path, defined by the rect, and wound in the specified direction.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param left Indicates the left coordinate of the upper left corner of the rectangle.
    @param top Indicates the top coordinate of the upper top corner of the rectangle.
    @param right Indicates the right coordinate of the lower right corner of the rectangle.
    @param bottom Indicates the bottom coordinate of the lower bottom corner of the rectangle.
    @param OH_Drawing_PathDirection Indicates the path direction.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PathAddRect(
        arg1: *mut OH_Drawing_Path,
        left: f32,
        top: f32,
        right: f32,
        bottom: f32,
        arg2: OH_Drawing_PathDirection,
    );
    /** @brief Adds a new contour to the path, defined by the rect, and wound in the specified direction.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param OH_Drawing_Rect Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    @param OH_Drawing_PathDirection Indicates the path direction.
    @param start Indicates initial corner of rect to add.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PathAddRectWithInitialCorner(
        arg1: *mut OH_Drawing_Path,
        arg2: *const OH_Drawing_Rect,
        arg3: OH_Drawing_PathDirection,
        start: u32,
    );
    /** @brief Adds a new contour to the path, defined by the round rect, and wound in the specified direction.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param OH_Drawing_RoundRect Indicates the pointer to an <b>OH_Drawing_RoundRect</b> object.
    @param OH_Drawing_PathDirection Indicates the path direction.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PathAddRoundRect(
        arg1: *mut OH_Drawing_Path,
        roundRect: *const OH_Drawing_RoundRect,
        arg2: OH_Drawing_PathDirection,
    );
    /** @brief Adds a oval to the path, defined by the rect, and wound in the specified direction.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param OH_Drawing_Rect Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    @param start Index of initial point of ellipse.
    @param OH_Drawing_PathDirection Indicates the path direction.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PathAddOvalWithInitialPoint(
        arg1: *mut OH_Drawing_Path,
        arg2: *const OH_Drawing_Rect,
        start: u32,
        arg3: OH_Drawing_PathDirection,
    );
    /** @brief Adds a oval to the path, defined by the rect, and wound in the specified direction.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param OH_Drawing_Rect Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    @param OH_Drawing_PathDirection Indicates the path direction.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PathAddOval(
        arg1: *mut OH_Drawing_Path,
        arg2: *const OH_Drawing_Rect,
        arg3: OH_Drawing_PathDirection,
    );
    /** @brief Appends arc to path, as the start of new contour.Arc added is part of ellipse bounded by oval,
    from startAngle through sweepAngle. Both startAngle and sweepAngle are measured in degrees, where zero degrees
    is aligned with the positive x-axis, and positive sweeps extends arc clockwise.If sweepAngle <= -360, or
    sweepAngle >= 360; and startAngle modulo 90 is nearly zero, append oval instead of arc. Otherwise, sweepAngle
    values are treated modulo 360, and arc may or may not draw depending on numeric rounding.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param OH_Drawing_Rect Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    @param startAngle Indicates the starting angle of arc in degrees.
    @param sweepAngle Indicates the sweep, in degrees. Positive is clockwise.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PathAddArc(
        arg1: *mut OH_Drawing_Path,
        arg2: *const OH_Drawing_Rect,
        startAngle: f32,
        sweepAngle: f32,
    );
    /** @brief Appends src path to path, transformed by matrix. Transformed curves may have different verbs,
    point, and conic weights.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param src Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param OH_Drawing_Matrix Indicates the length of the <b>OH_Drawing_Matrix</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PathAddPath(
        arg1: *mut OH_Drawing_Path,
        src: *const OH_Drawing_Path,
        arg2: *const OH_Drawing_Matrix,
    );
    /** @brief Appends src path to path, transformed by matrix and mode. Transformed curves may have different verbs,
    point, and conic weights.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param src Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param OH_Drawing_Matrix Indicates the length of the <b>OH_Drawing_Matrix</b> object.
    @param OH_Drawing_PathAddMode Indicates the add path's add mode.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PathAddPathWithMatrixAndMode(
        path: *mut OH_Drawing_Path,
        src: *const OH_Drawing_Path,
        arg1: *const OH_Drawing_Matrix,
        arg2: OH_Drawing_PathAddMode,
    );
    /** @brief Appends src path to path, transformed by mode. Transformed curves may have different verbs,
    point, and conic weights.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param src Indicates the pointer to an <b>OH_Drawing_Path</b> object, which is Appends src path to path.
    @param OH_Drawing_PathAddMode Indicates the add path's add mode.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PathAddPathWithMode(
        path: *mut OH_Drawing_Path,
        src: *const OH_Drawing_Path,
        arg1: OH_Drawing_PathAddMode,
    );
    /** @brief Appends src path to path, transformed by offset and mode. Transformed curves may have different verbs,
    point, and conic weights.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param src Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param dx Indicates offset added to src path x-axis coordinates.
    @param dy Indicates offset added to src path y-axis coordinates.
    @param OH_Drawing_PathAddMode Indicates the add path's add mode.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PathAddPathWithOffsetAndMode(
        path: *mut OH_Drawing_Path,
        src: *const OH_Drawing_Path,
        dx: f32,
        dy: f32,
        arg1: OH_Drawing_PathAddMode,
    );
    /** @brief Adds contour created from point array, adding (count - 1) line segments.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param points Indicates the point array.
    @param count Indicates the size of point array.
    @param isClosed Indicates Whether to add lines that connect the end and start.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PathAddPolygon(
        path: *mut OH_Drawing_Path,
        points: *const OH_Drawing_Point2D,
        count: u32,
        isClosed: bool,
    );
    /** @brief  Adds a circle to the path, and wound in the specified direction.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param x Indicates the x coordinate of the center of the circle.
    @param y Indicates the y coordinate of the center of the circle.
    @param radius Indicates the radius of the circle.
    @param OH_Drawing_PathDirection Indicates the path direction.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PathAddCircle(
        path: *mut OH_Drawing_Path,
        x: f32,
        y: f32,
        radius: f32,
        arg1: OH_Drawing_PathDirection,
    );
    /** @brief Parses the svg path from the string.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param str Indicates the string of the SVG path.
    @return Returns true if build path is successful, returns false otherwise.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PathBuildFromSvgString(
        path: *mut OH_Drawing_Path,
        str_: *const ::core::ffi::c_char,
    ) -> bool;
    /** @brief Return the status that point (x, y) is contained by path.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param x Indicates the x-axis value of containment test.
    @param y Indicates the y-axis value of containment test.
    @return Returns true if the point (x, y) is contained by path.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PathContains(arg1: *mut OH_Drawing_Path, x: f32, y: f32) -> bool;
    /** @brief Transforms verb array, point array, and weight by matrix. transform may change verbs
    and increase their number. path is replaced by transformed data.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param OH_Drawing_Matrix Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PathTransform(arg1: *mut OH_Drawing_Path, arg2: *const OH_Drawing_Matrix);
    /** @brief Transforms verb array, point array, and weight by matrix.
    Transform may change verbs and increase their number.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param src Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param OH_Drawing_Matrix Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
    @param dst Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param applyPerspectiveClip Indicates whether to apply perspective clip.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PathTransformWithPerspectiveClip(
        src: *mut OH_Drawing_Path,
        arg1: *const OH_Drawing_Matrix,
        dst: *mut OH_Drawing_Path,
        applyPerspectiveClip: bool,
    );
    /** @brief Sets FillType, the rule used to fill path.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param OH_Drawing_PathFillType Indicates the add path's fill type.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PathSetFillType(arg1: *mut OH_Drawing_Path, arg2: OH_Drawing_PathFillType);
    /** @brief Gets the length of the current path object.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param forceClosed Indicates whether free to modify/delete the path after this call.
    @return Returns the length of the current path object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PathGetLength(arg1: *mut OH_Drawing_Path, forceClosed: bool) -> f32;
    /** @brief Gets the smallest bounding box that contains the path.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param OH_Drawing_Path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param OH_Drawing_Rect Indicates the pointer to an <b>OH_Drawing_Rect</b> object.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PathGetBounds(arg1: *mut OH_Drawing_Path, arg2: *mut OH_Drawing_Rect);
    /** @brief Offset path replaces dst.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param dst Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param dx Indicates offset added to dst path x-axis coordinates.
    @param dy Indicates offset added to dst path y-axis coordinates.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PathOffset(
        path: *mut OH_Drawing_Path,
        dst: *mut OH_Drawing_Path,
        dx: f32,
        dy: f32,
    );
    /** @brief Determines whether the path current contour is closed.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param forceClosed Whether to close the Path.
    @return Returns <b>true</b> if the path current contour is closed; returns <b>false</b> otherwise.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PathIsClosed(path: *mut OH_Drawing_Path, forceClosed: bool) -> bool;
    /** @brief Gets the position and tangent of the distance from the starting position of the Path.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param forceClosed Whether to close the Path.
    @param distance The distance from the start of the Path.
    @param position Sets to the position of distance from the starting position of the Path.
    @param tangent Sets to the tangent of distance from the starting position of the Path.
    @return Returns <b>true</b> if succeeded; returns <b>false</b> otherwise.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PathGetPositionTangent(
        path: *mut OH_Drawing_Path,
        forceClosed: bool,
        distance: f32,
        position: *mut OH_Drawing_Point2D,
        tangent: *mut OH_Drawing_Point2D,
    ) -> bool;
    /** @brief Combines two paths.

    @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
    @param path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param srcPath Indicates the pointer to an <b>OH_Drawing_Path</b> object.
    @param op Indicates the operation to apply to combine.
    @return Returns <b>true</b> if constructed path is not empty; returns <b>false</b> otherwise.
    @since 12
    @version 1.0*/
    pub fn OH_Drawing_PathOp(
        path: *mut OH_Drawing_Path,
        srcPath: *const OH_Drawing_Path,
        op: OH_Drawing_PathOpMode,
    ) -> bool;
    /** @brief Computes the corresponding matrix at the specified distance.

     @syscap SystemCapability.Graphic.Graphic2D.NativeDrawing
     @param path Indicates the pointer to an <b>OH_Drawing_Path</b> object.
     @param forceClosed Whether to close the Path.
     @param distance The distance from the start of the Path.
     @param matrix Indicates the pointer to an <b>OH_Drawing_Matrix</b> object.
     @param flag Indicates what should be returned in the matrix.
     @return Returns <b>false</b> if path is nullptr or zero-length;
    returns <b>true</b> if path is not nullptr and not zero-length.
     @since 12
     @version 1.0*/
    pub fn OH_Drawing_PathGetMatrix(
        path: *mut OH_Drawing_Path,
        forceClosed: bool,
        distance: f32,
        matrix: *mut OH_Drawing_Matrix,
        flag: OH_Drawing_PathMeasureMatrixFlags,
    ) -> bool;
}