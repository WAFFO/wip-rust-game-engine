use glm::{Vec3, Mat4, translate, rotate_x, rotate_y, rotate_z, scale};
use specs::{VecStorage, Component};

pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}

impl Default for Transform {
    fn default() -> Transform {
        Transform {
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

impl Transform {
    pub fn model(&self) -> Mat4 {
        translate(self.position)
            * rotate_x(self.rotation[0])
            * rotate_y(self.rotation[1])
            * rotate_z(self.rotation[2])
            * scale(self.scale)
    }
}
