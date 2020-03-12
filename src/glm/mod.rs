
pub mod vec3;
pub mod vec4;
pub mod mat3;
pub mod mat4;
pub mod quat;


pub type FSize = f32;

pub use self::vec3::Vec3;
pub use self::vec4::Vec4;
pub use self::mat3::Mat3;
pub use self::mat4::Mat4;

pub const NEAR_ZERO: FSize = 0.000001;

pub fn perspective(aspect: FSize, fov: FSize, near: FSize, far: FSize) -> Mat4 {
    let xy_max = near * fov.to_radians();

    let depth = far - near;
    let q = -(far + near) / depth;
    let qn = -2.0 * (far * near) / depth;

    let w = 2.0 * near / xy_max;
    let w = w / aspect;
    let h = 2.0 * near / xy_max;

    let mut m = Mat4::zero();

    m[0] =   w;
    m[5] =   h;
    m[10] =  q;
    m[11] = -1.0;
    m[14] = qn;

    m
}

pub fn look_at(pos: Vec3, target: Vec3, up: Vec3) -> Mat4 {
    let zaxis: Vec3 = (pos - target).normalize();
    let xaxis: Vec3 = up.cross(&target).normalize();
    let yaxis: Vec3 = zaxis.cross(&xaxis).normalize();

    Mat4::new(
        Vec4::new(xaxis[0], yaxis[0], zaxis[0], 0.0),
        Vec4::new(yaxis[1], yaxis[1], zaxis[1], 0.0),
        Vec4::new(zaxis[2], yaxis[2], zaxis[2], 0.0),
        Vec4::new(xaxis.dot(&pos) * -1.0, yaxis.dot(&pos) * -1.0, zaxis.dot(&pos) * -1.0, 1.0),
    )
}

pub fn translate(t: Vec3) -> Mat4 {
    Mat4([
        1.0,  0.0,  0.0, 0.0,
        0.0,  1.0,  0.0, 0.0,
        0.0,  0.0,  1.0, 0.0,
        t[0], t[1], t[2], 1.0,
    ])
}

pub fn rotate_x(f: FSize) -> Mat4 {
    Mat4([
        1.0, 0.0, 0.0, 0.0,
        0.0, f.cos(), -f.sin(), 0.0,
        0.0, f.sin(), f.cos(), 0.0,
        0.0, 0.0, 0.0, 1.0,
    ])
}

pub fn rotate_y(f: FSize) -> Mat4 {
    Mat4([
        f.cos(), 0.0, -f.sin(), 0.0,
        0.0, 1.0, 0.0, 0.0,
        f.sin(), 0.0, f.cos(), 0.0,
        0.0, 0.0, 0.0, 1.0,
    ])
}

pub fn rotate_z(f: FSize) -> Mat4 {
    Mat4([
        f.cos(), -f.sin(), 0.0, 0.0,
        f.sin(), f.cos(), 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ])
}

pub fn scale(s: Vec3) -> Mat4 {
    Mat4([
        s[0],  0.0,  0.0, 0.0,
        0.0, s[1],  0.0, 0.0,
        0.0,  0.0, s[2], 0.0,
        0.0,  0.0,  0.0, 1.0,
    ])
}