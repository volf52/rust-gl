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
    pub a: f32,

    /// Position (0, 1) in a 3x3 affine transformation matrix.
    pub b: f32,

    /// Position (1, 0) in a 3x3 affine transformation matrix.
    pub c: f32,

    /// Position (1, 1) in a 3x3 affine transformation matrix.
    pub d: f32,

    /// Position (2, 0) in a 3x3 affine transformation matrix.
    pub tx: f32,

    /// Position (2, 1) in a 3x3 affine transformation matrix.
    pub ty: f32,
}

impl Matrix {
    pub fn new() -> Self {
        Matrix {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            tx: 0.0,
            ty: 0.0,
        }
    }
    /**
     * Returns a projection matrix.
     *
     * @param {number} width - Width of the canvas.
     * @param {number} y - Height of the canvas.
     * @return {Matrix} This matrix.
     */
    pub fn projection(width: f32, height: f32) -> Matrix {
        Matrix {
            a: 2.0 / width,
            b: 0.0,
            c: 0.0,
            d: 2.0 / height,
            tx: 0.0,
            ty: 0.0,
        }
    }

    pub fn project(&self, width: f32, height: f32) -> Matrix {
        self.scale(2.0 / width, -2.0 / height)
    }

    pub fn transpose(&self) -> [f32; 9] {
        [
            self.a, self.c, self.tx, self.b, self.d, self.ty, 0.0, 0.0, 1.0,
        ]
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
     * @return {Matrix} This matrix. Good for chaining method calls.
     */

    pub fn scale(&self, x: f32, y: f32) -> Matrix {
        Matrix {
            a: self.a * x,
            b: self.b * y,
            c: self.c * x,
            d: self.d * y,
            tx: self.tx * x,
            ty: self.ty * y,
        }
    }

    pub fn scale_inplace(&mut self, x: f32, y: f32) {
        self.a *= x;
        self.b *= y;
        self.c *= x;
        self.d *= y;
        self.tx *= x;
        self.ty *= y;
    }

    /**
     * Applies a rotation transformation to the matrix.
     *
     * @param {number} angle - The angle in radians.
     * @return {PIXI.Matrix} This matrix. Good for chaining method calls.
     */
    pub fn rotate(&self, angle: f32) -> Matrix {
        let sin = angle.sin();
        let cos = angle.cos();

        let a1 = self.a;
        let c1 = self.c;
        let tx1 = self.tx;

        Matrix {
            a: (a1 * cos) - (self.b * sin),
            b: (a1 * sin) + (self.b * cos),
            c: (c1 * cos) - (self.d * sin),
            d: (c1 * sin) + (self.d * cos),
            tx: (tx1 * cos) - (self.ty * sin),
            ty: (tx1 * sin) + (self.ty * cos),
        }
    }
    /**
     * Translates the matrix on the x and y.
     *
     * @param x - How much to translate x by (+ive right, -ive left)
     * @param y - How much to translate y by (+ive up, -ive down)
     * @return This matrix. Good for chaining method calls.
     */
    pub fn translate(&self, x: f32, y: f32) -> Matrix {
        Matrix {
            a: self.a,
            b: self.b,
            c: self.c,
            d: self.d,
            tx: self.tx + x,
            ty: self.ty + y,
        }
    }

    pub fn translate_inplace(&mut self, x: f32, y: f32) {
        self.tx += x;
        self.ty += y;
    }

    pub fn translation(x: f32, y: f32) -> Matrix {
        Matrix {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            tx: x,
            ty: y,
        }
    }

    pub fn mul(&self, mat2: &Matrix) -> Matrix {
        let a = self.to_array();
        let b = mat2.transpose();

        Matrix {
            a: Matrix::dot_product(&a[0..3], &b[0..3]).unwrap(),
            b: Matrix::dot_product(&a[0..3], &b[3..6]).unwrap(),
            c: Matrix::dot_product(&a[3..6], &b[0..3]).unwrap(),
            d: Matrix::dot_product(&a[3..6], &b[3..6]).unwrap(),
            tx: Matrix::dot_product(&a[6..9], &b[0..3]).unwrap(),
            ty: Matrix::dot_product(&a[6..9], &b[3..6]).unwrap(),
        }
    }

    pub fn mul_inplace(&mut self, mat2: &Matrix) {
        let a = self.to_array();
        let b = mat2.transpose();

        self.a = Matrix::dot_product(&a[0..3], &b[0..3]).unwrap();
        self.b = Matrix::dot_product(&a[0..3], &b[3..6]).unwrap();
        self.c = Matrix::dot_product(&a[3..6], &b[0..3]).unwrap();
        self.d = Matrix::dot_product(&a[3..6], &b[3..6]).unwrap();
        self.tx = Matrix::dot_product(&a[6..9], &b[0..3]).unwrap();
        self.ty = Matrix::dot_product(&a[6..9], &b[3..6]).unwrap();
    }

    pub fn multiply(mat1: &Matrix, mat2: &Matrix) -> Matrix {
        let a = mat1.to_array();
        let b = mat2.transpose();

        Matrix {
            a: Matrix::dot_product(&a[0..3], &b[0..3]).unwrap(),
            b: Matrix::dot_product(&a[0..3], &b[3..6]).unwrap(),
            c: Matrix::dot_product(&a[3..6], &b[0..3]).unwrap(),
            d: Matrix::dot_product(&a[3..6], &b[3..6]).unwrap(),
            tx: Matrix::dot_product(&a[6..9], &b[0..3]).unwrap(),
            ty: Matrix::dot_product(&a[6..9], &b[3..6]).unwrap(),
        }
    }

    fn dot_product(a: &[f32], b: &[f32]) -> Option<f32> {
        match (a.len(), b.len()) {
            t if t.0 != t.1 => None,
            _ => Some(
                a.iter()
                    .zip(b.iter())
                    .fold(0.0, |sum, (el_a, el_b)| sum + el_a * el_b),
            ),
        }
    }

    pub fn determinant(&self) -> f32 {
        self.a * self.d - self.b * self.c
    }

    pub fn inverse(&self) -> Result<Matrix, &str> {
        let det = self.determinant();
        if det == 0.0 {
            return Err("Cannot invert a matrix with determinant 0");
        };

        Ok(Matrix {
            a: self.d / det,
            b: -self.b / det,
            c: -self.c / det,
            d: self.a / det,
            tx: ((self.c * self.ty) - (self.d * self.tx)) / det,
            ty: -((self.a * self.ty) - (self.b * self.tx)) / det,
        })
    }

    pub fn inverse_affine_point(&self, x: f32, y: f32) -> (f32, f32) {
        let inv = self.inverse().unwrap();

        let x_p = inv.a * x + inv.c * y + inv.tx;
        let y_p = inv.b * x + inv.d * y + inv.ty;

        (x_p, y_p)
    }
}
