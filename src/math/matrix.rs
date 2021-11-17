/**
 * An affine transformation matrix (projective coordinates).
 *
 * Here is a representation of it:
 * | a | b | tx|
 * | c | d | ty|
 * | 0 | 0 | 1 |
 */

/**
 * @param {number} [a=1] - x scale
 * @param {number} [b=0] - y skew
 * @param {number} [c=0] - x skew
 * @param {number} [d=1] - y scale
 * @param {number} [tx=0] - x translation
 * @param {number} [ty=0] - y translation
 */
#[derive(Debug, Clone)]
pub struct Matrix {
    /// Position (0, 0) in a 3x3 affine transformation matrix.
    a: f32,

    /// Position (0, 1) in a 3x3 affine transformation matrix.
    b: f32,

    /// Position (1, 0) in a 3x3 affine transformation matrix.
    c: f32,

    /// Position (1, 1) in a 3x3 affine transformation matrix.
    d: f32,

    /// Position (2, 0) in a 3x3 affine transformation matrix.
    tx: f32,

    /// Position (2, 1) in a 3x3 affine transformation matrix.
    ty: f32,
    // list: [f32; 6]
}

impl Matrix {
    pub fn new() -> Matrix {
        Matrix {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            tx: 0.0,
            ty: 0.0,
        }
    }
    pub fn from_array(arr: [f32; 6]) -> Matrix {
        Matrix {
            a: arr[0],
            b: arr[1],
            c: arr[2],
            d: arr[3],
            tx: arr[4],
            ty: arr[5],
        }
    }

    pub fn to_array(&self) -> [f32; 9] {
        [
            self.a, self.b, 0.0, self.c, self.d, 0.0, self.tx, self.ty, 1.0,
        ]
    }

    /**
     * Applies a scale transformation to the matrix.
     *
     * @param {number} x - The amount to scale horizontally
     * @param {number} y - The amount to scale vertically
     * @return {PIXI.Matrix} This matrix. Good for chaining method calls.
     */

    pub fn scale(mat: &Matrix, x: f32, y: f32) -> Matrix {
        Matrix {
            a: mat.a * x,
            b: mat.b * y,
            c: mat.c * x,
            d: mat.d * y,
            tx: mat.tx * x,
            ty: mat.ty * y,
        }
    }

    /**
     * Applies a rotation transformation to the matrix.
     *
     * @param {number} angle - The angle in radians.
     * @return {PIXI.Matrix} This matrix. Good for chaining method calls.
     */
    pub fn rotate(mat: &Matrix, angle: f32) -> Matrix {
        let sin = angle.sin();
        let cos = angle.cos();

        let a1 = mat.a;
        let c1 = mat.c;
        let tx1 = mat.tx;

        Matrix {
            a: (a1 * cos) - (mat.b * sin),
            b: (a1 * sin) + (mat.b * cos),
            c: (c1 * cos) - (mat.d * sin),
            d: (c1 * sin) + (mat.d * cos),
            tx: (tx1 * cos) - (mat.ty * sin),
            ty: (tx1 * sin) + (mat.ty * cos),
        }
    }
    /**
     * Translates the matrix on the x and y.
     *
     * @param x - How much to translate x by
     * @param y - How much to translate y by
     * @return This matrix. Good for chaining method calls.
     */
    pub fn translate(mat: &Matrix, x: f32, y: f32) -> Matrix {
        Matrix {
            a: mat.a,
            b: mat.b,
            c: mat.c,
            d: mat.d,
            tx: mat.tx + x,
            ty: mat.ty + y,
        }
    }
}
