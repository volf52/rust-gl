use super::Matrix;
use super::ipoint::Point;

struct Transform {
    /** The world transformation matrix. */
    world_transform: Matrix,
    /** The local transformation matrix. */
    local_transform: Matrix,
    
    /** The coordinate of the object relative to the local coordinates of the parent. */
    position: Point,

    /** The scale factor of the object. */
    scale:  Point,

    /** The pivot point of the displayObject that it rotates around. */
    pivot:  Point,

    /** The skew amount, on the x and y axis. */
    skew:  Point,

    parent_id: i32,

    /** The locally unique ID of the world transform. */
    world_id: i32,

    /** The rotation amount. */
    _rotation: i32,

    /**
     * The X-coordinate value of the normalized local X axis,
     * the first column of the local transformation matrix without a scale.
     */
    _cx: i32,

    /**
     * The Y-coordinate value of the normalized local X axis,
     * the first column of the local transformation matrix without a scale.
     */
    _sx: i32,

    /**
     * The X-coordinate value of the normalized local Y axis,
     * the second column of the local transformation matrix without a scale.
     */
    _cy: i32,

    /**
     * The Y-coordinate value of the normalized local Y axis,
     * the second column of the local transformation matrix without a scale.
     */
    _sy: i32,

    /** The locally unique ID of the local transform. */
    local_id: i32,

    /**
     * The locally unique ID of the local transform
     * used to calculate the current local transformation matrix.
     */
    current_local_id: i32,
}

