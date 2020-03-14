use glm::{Vec3, Mat4, Quat, translate, rotate, scale};
use specs::{VecStorage, Component};

pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}

impl Default for Transform {
    fn default() -> Transform {
        Transform {
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: Quat::new(1.0, 0.0, 0.0, 0.0),
            scale: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

impl Transform {
    pub fn model(&self) -> Mat4 {
        translate(self.position)
            * rotate(self.rotation)
            * scale(self.scale)
    }

    pub fn rotate(&mut self, rotation: Quat) {
        self.rotation = self.rotation * rotation;
    }
}
