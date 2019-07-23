
use super::FSize;
use glm::{Mat4, Vec3, Mat3};

// note: layout is [ x, y, z, w]
//              or [ i, j, k, w]
#[derive(Debug, Copy, Clone, Default, PartialEq)]
struct Quat (pub(crate) [FSize; 4]);


impl Quat {
    pub fn new(w: FSize, x: FSize, y: FSize, z: FSize) -> Quat { Quat ( [x, y, z, w] ) }
    pub fn x(&self) -> FSize { self.0[0] }
    pub fn y(&self) -> FSize { self.0[1] }
    pub fn z(&self) -> FSize { self.0[2] }
    pub fn w(&self) -> FSize { self.0[3] }
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 ( [
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
        ] )
    }
    pub fn mag(&self) -> FSize { ( self[0].powi(2) + self[1].powi(2) + self[2].powi(2) + self[3].powi(2) ).sqrt() }
    pub fn length(&self) -> FSize { self.mag() }
    pub fn normalize(&self) -> Quat {
        let mag = self.mag();
        if mag != 0.0 {
            Quat ( [
                self[0] / mag,
                self[1] / mag,
                self[2] / mag,
                self[3] / mag,
            ] )
        }
        else {
            *self
        }
    }
    pub fn conjugate(&self) -> Quat {
        Quat ( [
            -self.x(), // x
            -self.y(), // y
            -self.z(), // z
             self.w(), // w
        ] )
    }
    pub fn euler_to_quat(roll: FSize, pitch: FSize, yaw: FSize) -> Quat {
        let cr = (roll/2.0).cos();
        let cp = (pitch/2.0).cos();
        let cy = (yaw/2.0).cos();
        let sr = (roll/2.0).sin();
        let sp = (pitch/2.0).sin();
        let sy = (yaw/2.0).sin();
        let cpcy = cp * cy;
        let spsy = sp * sy;
        Quat ([
            sr * cpcy - cr * spsy,
            cr * sp * cy + sr * cp * sy,
            cr * cp * sy - sr * sp * cy,
            cr * cpcy + sr * spsy,
        ])
    }
    pub fn mat3(&self) -> Mat3 {
        // calculate coefficients
        let x2 = self.x() + self.x();
        let y2 = self.y() + self.y();
        let z2 = self.z() + self.z();
        let xx = self.x() * x2;
        let xy = self.x() * y2;
        let xz = self.x() * z2;
        let yy = self.y() * y2;
        let yz = self.y() * z2;
        let zz = self.z() * z2;
        let wx = self.w() * x2;
        let wy = self.w() * y2;
        let wz = self.w() * z2;
        Mat3([
            1.0 - yy - zz,       xy + wz,       xz - wy,
                  xy - wz, 1.0 - xx - zz,       yz + wx,
                  xz + wy,       yz - wx, 1.0 - xx - yy,
        ])
    }
    pub fn mat4(&self) -> Mat4 {
        // calculate coefficients
        let x2 = self.x() + self.x();
        let y2 = self.y() + self.y();
        let z2 = self.z() + self.z();
        let xx = self.x() * x2;
        let xy = self.x() * y2;
        let xz = self.x() * z2;
        let yy = self.y() * y2;
        let yz = self.y() * z2;
        let zz = self.z() * z2;
        let wx = self.w() * x2;
        let wy = self.w() * y2;
        let wz = self.w() * z2;
        Mat4([
            1.0 - yy - zz,       xy + wz,       xz - wy, 0.0,
                  xy - wz, 1.0 - xx - zz,       yz + wx, 0.0,
                  xz + wy,       yz - wx, 1.0 - xx - yy, 0.0,
                      0.0,           0.0,           0.0, 1.0,
        ])
    }
}


impl std::ops::Mul<Quat> for Quat {
    type Output = Quat;

    fn mul(self, rhs: Quat) -> Quat {
        Quat ( [
            self.x() * rhs.w() + self.w() * rhs.x() + self.y() * rhs.z() - self.z() * rhs.y(), // x
            self.y() * rhs.w() + self.w() * rhs.y() + self.z() * rhs.x() - self.x() * rhs.z(), // y
            self.z() * rhs.w() + self.w() * rhs.z() + self.x() * rhs.y() - self.y() * rhs.x(), // z
            self.w() * rhs.w() - self.x() * rhs.x() - self.y() * rhs.y() - self.z() * rhs.z(), // w
        ] )
    }
}

impl std::ops::Mul<Vec3> for Quat {
    type Output = Quat;

    fn mul(self, rhs: Vec3) -> Quat {
        Quat ( [
            self.w() * rhs.x() + self.y() * rhs.z() - self.z() * rhs.y(), // x
            self.w() * rhs.y() + self.z() * rhs.x() - self.x() * rhs.z(), // y
            self.w() * rhs.z() + self.x() * rhs.y() - self.y() * rhs.x(), // z
           -self.x() * rhs.x() - self.y() * rhs.y() - self.z() * rhs.z(), // w
        ] )
    }
}

impl std::ops::Index<usize> for Quat {
    type Output = FSize;

    fn index(&self, index: usize) -> &FSize {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for Quat {
    fn index_mut(&mut self, index: usize) -> &mut FSize {
        &mut self.0[index]
    }
}


impl std::ops::Mul<FSize> for Quat {
    type Output = Quat;

    fn mul(self, rhs: FSize) -> Quat {
        Quat ( [
            self[0] * rhs,
            self[1] * rhs,
            self[2] * rhs,
            self[3] * rhs,
        ] )
    }
}

impl std::ops::Div<FSize> for Quat {
    type Output = Quat;

    fn div(self, rhs: FSize) -> Quat {
        if rhs == 0.0 { panic!("Cannot divide by zero. (Vec4 / 0.0)"); }
        Quat ( [
            self[0] / rhs,
            self[1] / rhs,
            self[2] / rhs,
            self[3] / rhs,
        ] )
    }
}