use specs::{VecStorage, Component};
use glm;
use glm::{Vec3, Mat4, Quat, translation, quat_cast, scaling};

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
            position: glm::vec3(0.0, 0.0, 0.0),
            rotation: glm::quat(1.0, 0.0, 0.0, 0.0),
            scale: glm::vec3(0.0, 0.0, 0.0),
        }
    }
}

impl Transform {
    pub fn model(&self) -> Mat4 {
        translation(&self.position)
            * quat_cast(&self.rotation)
            * scaling(&self.scale)
    }

    pub fn rotate(&mut self, rotation: &Quat) {
        self.rotation = &self.rotation * rotation;
    }
}
