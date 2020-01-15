
use super::FSize;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Vec3 ( pub(crate) [FSize; 3] );

impl Vec3 {
    pub fn new(x: FSize, y: FSize, z: FSize) -> Vec3 { Vec3 ( [x, y, z] ) }
    pub fn zero() -> Vec3 { Vec3 ( [0.0, 0.0, 0.0] ) }
    pub fn one()  -> Vec3 { Vec3 ( [1.0, 1.0, 1.0] ) }
    pub fn all(f: FSize) -> Vec3 { Vec3 ( [f, f, f] ) }
    pub fn vec3(vec: [FSize;3]) -> Vec3 { Vec3(vec) }
    pub fn data(&self) -> [FSize;3] { self.0 }
    pub fn data_ref(&self) -> &[FSize;3] { &self.0 }
    pub fn data_ref_mut(&mut self) -> &mut [FSize;3] { &mut self.0 }
    pub fn x(&self) -> FSize { self.0[0] }
    pub fn y(&self) -> FSize { self.0[1] }
    pub fn z(&self) -> FSize { self.0[2] }
    pub fn dot(&self, other: &Vec3) -> FSize {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 ( [
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
        ] )
    }
    pub fn mag(&self) -> FSize { ( self[0].powi(2) + self[1].powi(2) + self[2].powi(2) ).sqrt() }
    pub fn length(&self) -> FSize { self.mag() }
    pub fn normalize(&self) -> Vec3 {
        let mag = self.mag();
        if mag != 0.0 {
            *self / mag
        }
        else {
            *self
        }
    }
    pub fn bound(&self, bound: FSize) -> Vec3 {
        Vec3 ( [
            self[0] % bound,
            self[1] % bound,
            self[2] % bound,
        ] )
    }
}

impl std::ops::Index<usize> for Vec3 {
    type Output = FSize;

    fn index(&self, index: usize) -> &FSize {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut FSize {
        &mut self.0[index]
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 ( [
            self[0] + other[0],
            self[1] + other[1],
            self[2] + other[2],
        ] )
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3 ( [
            self[0] + other[0],
            self[1] + other[1],
            self[2] + other[2],
        ] )
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 ( [
            self[0] - other[0],
            self[1] - other[1],
            self[2] - other[2],
        ] )
    }
}

impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        *self = Vec3 ( [
            self[0] - other[0],
            self[1] - other[1],
            self[2] - other[2],
        ] )
    }
}

impl std::ops::Mul<FSize> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: FSize) -> Vec3 {
        Vec3 ( [
            self[0] * rhs,
            self[1] * rhs,
            self[2] * rhs,
        ] )
    }
}

impl std::ops::Div<FSize> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: FSize) -> Vec3 {
        if rhs == 0.0 { panic!("Cannot divide by zero. (Vec3 / 0.0)"); }
        Vec3 ( [
            self[0] / rhs,
            self[1] / rhs,
            self[2] / rhs,
        ] )
    }
}

impl std::ops::Rem<FSize> for Vec3 {
    type Output = Vec3;

    fn rem(self, rhs: FSize) -> Vec3 {
        if rhs == 0.0 { panic!("Cannot divide by zero. (Vec3 % 0.0)"); }
        Vec3 ( [
            self[0] % rhs,
            self[1] % rhs,
            self[2] % rhs,
        ] )
    }
}

impl std::ops::RemAssign<FSize> for Vec3 {

    fn rem_assign(&mut self, rhs: FSize) {
        if rhs == 0.0 { panic!("Cannot divide by zero. (Vec3 % 0.0)"); }
        *self = Vec3 ( [
            self[0] % rhs,
            self[1] % rhs,
            self[2] % rhs,
        ] )
    }
}