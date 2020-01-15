
use super::FSize;

use super::Vec3;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Vec4 ( pub(crate) [FSize; 4] );

impl Vec4 {
    pub fn new(x: FSize, y: FSize, z: FSize, w: FSize) -> Vec4 { Vec4 ( [x, y, z, w] ) }
    pub fn zero() -> Vec4 { Vec4 ( [0.0, 0.0, 0.0, 0.0] ) }
    pub fn one()  -> Vec4 { Vec4 ( [1.0, 1.0, 1.0, 1.0] ) }
    pub fn all(f: FSize) -> Vec4 { Vec4 ( [f, f, f, f] ) }
    pub fn vec3_w(v: Vec3, w: FSize) -> Vec4 { Vec4 ( [v[0], v[1], v[2], w] ) }
    pub fn vec4(vec: [FSize;4]) -> Vec4 { Vec4(vec) }
    pub fn data(&self) -> [FSize;4] { self.0 }
    pub fn data_ref(&self) -> &[FSize;4] { &self.0 }
    pub fn data_ref_mut(&mut self) -> &mut [FSize;4] { &mut self.0 }
    pub fn x(&self) -> FSize { self.0[0] }
    pub fn y(&self) -> FSize { self.0[1] }
    pub fn z(&self) -> FSize { self.0[2] }
    pub fn w(&self) -> FSize { self.0[3] }
    pub fn xyz(&self) -> Vec3 { Vec3([self[0], self[1], self[2]]) }
    pub fn dot(&self, other: &Vec4) -> FSize {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2] + self[3] * other[3]
    }
    /// note! cross in 4 dimensions doesn't work, 4th component is set to parameter w!
    pub fn cross(&self, other: &Vec4, w: FSize) -> Vec4 {
        Vec4 ( [
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
            w,
        ] )
    }
    pub fn mag(&self) -> FSize { ( self[0].powi(2) + self[1].powi(2) + self[2].powi(2) + self[3].powi(2) ).sqrt() }
    pub fn length(&self) -> FSize { self.mag() }
    pub fn normalize(&self) -> Vec4 {
        let mag = self.mag();
        if mag != 0.0 {
            *self / mag
        }
        else {
            *self
        }
    }
    pub fn bound(&self, bound: FSize) -> Vec4 {
        Vec4 ( [
            self[0] % bound,
            self[1] % bound,
            self[2] % bound,
            self[3] % bound,
        ] )
    }
}

impl std::ops::Index<usize> for Vec4 {
    type Output = FSize;

    fn index(&self, index: usize) -> &FSize {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for Vec4 {
    fn index_mut(&mut self, index: usize) -> &mut FSize {
        &mut self.0[index]
    }
}

impl From<Vec3> for Vec4 {
    fn from(f: Vec3) -> Self {
        Vec4 ( [f[0], f[1], f[2], 0.0] )
    }
}

impl std::ops::Add for Vec4 {
    type Output = Vec4;

    fn add(self, other: Vec4) -> Vec4 {
        Vec4 ( [
            self[0] + other[0],
            self[1] + other[1],
            self[2] + other[2],
            self[3] + other[3],
        ] )
    }
}

impl std::ops::AddAssign for Vec4 {
    fn add_assign(&mut self, other: Vec4) {
        *self = Vec4 ( [
            self[0] + other[0],
            self[1] + other[1],
            self[2] + other[2],
            self[3] + other[3],
        ] )
    }
}

impl std::ops::Sub for Vec4 {
    type Output = Vec4;

    fn sub(self, other: Vec4) -> Vec4 {
        Vec4 ( [
            self[0] - other[0],
            self[1] - other[1],
            self[2] - other[2],
            self[3] - other[3],
        ] )
    }
}

impl std::ops::SubAssign for Vec4 {
    fn sub_assign(&mut self, other: Vec4) {
        *self = Vec4 ( [
            self[0] - other[0],
            self[1] - other[1],
            self[2] - other[2],
            self[3] - other[3],
        ] )
    }
}

impl std::ops::Mul<FSize> for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: FSize) -> Vec4 {
        Vec4 ( [
            self[0] * rhs,
            self[1] * rhs,
            self[2] * rhs,
            self[3] * rhs,
        ] )
    }
}

impl std::ops::Div<FSize> for Vec4 {
    type Output = Vec4;

    fn div(self, rhs: FSize) -> Vec4 {
        if rhs == 0.0 { panic!("Cannot divide by zero. (Vec4 / 0.0)"); }
        Vec4 ( [
            self[0] / rhs,
            self[1] / rhs,
            self[2] / rhs,
            self[3] / rhs,
        ] )
    }
}

impl std::ops::Rem<FSize> for Vec4 {
    type Output = Vec4;

    fn rem(self, rhs: FSize) -> Vec4 {
        if rhs == 0.0 { panic!("Cannot divide by zero. (Vec4 % 0.0)"); }
        Vec4 ( [
            self[0] % rhs,
            self[1] % rhs,
            self[2] % rhs,
            self[3] % rhs,
        ] )
    }
}

impl std::ops::RemAssign<FSize> for Vec4 {

    fn rem_assign(&mut self, rhs: FSize) {
        if rhs == 0.0 { panic!("Cannot divide by zero. (Vec4 % 0.0)"); }
        *self = Vec4 ( [
            self[0] % rhs,
            self[1] % rhs,
            self[2] % rhs,
            self[3] % rhs,
        ] )
    }
}