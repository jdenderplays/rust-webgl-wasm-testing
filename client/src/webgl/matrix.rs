
type RawMatrix = [f32; 16];

pub struct Matrix(RawMatrix);

impl Matrix {

    pub fn new() -> Matrix {
        Matrix::unit()
    }

    pub fn unit() -> Matrix {
        unit().into()
    }

    pub fn translate(&self, tx: f32, ty: f32, tz: f32) -> Matrix {
        multiply(self.0, translation(tx, ty, tz)).into()
    }

    pub fn xRotate(&self, angle_in_radians: f32) -> Matrix {
        multiply(self.0, xRotation(angle_in_radians)).into()
    }
 
    pub fn yRotate(&self, angle_in_radians: f32) -> Matrix {
        multiply(self.0, yRotation(angle_in_radians)).into()
    }
 
    pub fn zRotate(&self, angle_in_radians: f32) -> Matrix {
        multiply(self.0, zRotation(angle_in_radians)).into()
    }
 
    pub fn scale(&self, sx: f32, sy: f32, sz: f32) -> Matrix {
        multiply(self.0, scaling(sx, sy, sz)).into()
    }
}

impl From<RawMatrix> for Matrix {

    fn from(mat: RawMatrix) -> Self {
        Matrix(mat)
    } 
}

// All the fowlowing functions were adapted from:
// https://webgl2fundamentals.org/webgl/lessons/webgl-3d-orthographic.html

fn unit() -> RawMatrix {
    [
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ]
}

fn translation(tx: f32, ty: f32, tz: f32) -> RawMatrix {
    [
       1.0,  0.0,  0.0,  0.0,
       0.0,  1.0,  0.0,  0.0,
       0.0,  0.0,  1.0,  0.0,
       tx,   ty,   tz,   1.0,
    ]
}
 
fn xRotation(angle_in_radians: f32) -> RawMatrix {

    let c = angle_in_radians.cos();
    let s = angle_in_radians.sin();
 
    [
      1.0, 0.0, 0.0, 0.0,
      0.0, c,   s,   0.0,
      0.0, -s,  c,   0.0,
      0.0, 0.0, 0.0, 1.0,
    ]
}
 
fn yRotation(angle_in_radians: f32) -> RawMatrix {

    let c = angle_in_radians.cos();
    let s = angle_in_radians.sin();
 
    [
      c,   0.0, -s,  0.0,
      0.0, 1.0, 0.0, 0.0,
      s,   0.0, c,   0.0,
      0.0, 0.0, 0.0, 1.0,
    ]
}
 
fn zRotation(angle_in_radians: f32) -> RawMatrix {

    let c = angle_in_radians.cos();
    let s = angle_in_radians.sin();
 
    [
       c,   s,   0.0, 0.0,
      -s,   c,   0.0, 0.0,
       0.0, 0.0, 1.0, 0.0,
       0.0, 0.0, 0.0, 1.0,
    ]
}

fn scaling(sx: f32, sy: f32, sz: f32) -> RawMatrix {
    [
      sx,   0.0,  0.0,  0.0,
      0.0,  sy,   0.0,  0.0,
      0.0,  0.0,  sz,   0.0,
      0.0,  0.0,  0.0,  1.0,
    ]
}

fn multiply(a: RawMatrix, b: RawMatrix) -> RawMatrix {

    let b00 = b[0 * 4 + 0];
    let b01 = b[0 * 4 + 1];
    let b02 = b[0 * 4 + 2];
    let b03 = b[0 * 4 + 3];
    let b10 = b[1 * 4 + 0];
    let b11 = b[1 * 4 + 1];
    let b12 = b[1 * 4 + 2];
    let b13 = b[1 * 4 + 3];
    let b20 = b[2 * 4 + 0];
    let b21 = b[2 * 4 + 1];
    let b22 = b[2 * 4 + 2];
    let b23 = b[2 * 4 + 3];
    let b30 = b[3 * 4 + 0];
    let b31 = b[3 * 4 + 1];
    let b32 = b[3 * 4 + 2];
    let b33 = b[3 * 4 + 3];
    let a00 = a[0 * 4 + 0];
    let a01 = a[0 * 4 + 1];
    let a02 = a[0 * 4 + 2];
    let a03 = a[0 * 4 + 3];
    let a10 = a[1 * 4 + 0];
    let a11 = a[1 * 4 + 1];
    let a12 = a[1 * 4 + 2];
    let a13 = a[1 * 4 + 3];
    let a20 = a[2 * 4 + 0];
    let a21 = a[2 * 4 + 1];
    let a22 = a[2 * 4 + 2];
    let a23 = a[2 * 4 + 3];
    let a30 = a[3 * 4 + 0];
    let a31 = a[3 * 4 + 1];
    let a32 = a[3 * 4 + 2];
    let a33 = a[3 * 4 + 3];

    let mut dst: [f32; 16] = Matrix::new().0;

    dst[ 0] = b00 * a00 + b01 * a10 + b02 * a20 + b03 * a30;
    dst[ 1] = b00 * a01 + b01 * a11 + b02 * a21 + b03 * a31;
    dst[ 2] = b00 * a02 + b01 * a12 + b02 * a22 + b03 * a32;
    dst[ 3] = b00 * a03 + b01 * a13 + b02 * a23 + b03 * a33;
    dst[ 4] = b10 * a00 + b11 * a10 + b12 * a20 + b13 * a30;
    dst[ 5] = b10 * a01 + b11 * a11 + b12 * a21 + b13 * a31;
    dst[ 6] = b10 * a02 + b11 * a12 + b12 * a22 + b13 * a32;
    dst[ 7] = b10 * a03 + b11 * a13 + b12 * a23 + b13 * a33;
    dst[ 8] = b20 * a00 + b21 * a10 + b22 * a20 + b23 * a30;
    dst[ 9] = b20 * a01 + b21 * a11 + b22 * a21 + b23 * a31;
    dst[10] = b20 * a02 + b21 * a12 + b22 * a22 + b23 * a32;
    dst[11] = b20 * a03 + b21 * a13 + b22 * a23 + b23 * a33;
    dst[12] = b30 * a00 + b31 * a10 + b32 * a20 + b33 * a30;
    dst[13] = b30 * a01 + b31 * a11 + b32 * a21 + b33 * a31;
    dst[14] = b30 * a02 + b31 * a12 + b32 * a22 + b33 * a32;
    dst[15] = b30 * a03 + b31 * a13 + b32 * a23 + b33 * a33;

    dst
}
