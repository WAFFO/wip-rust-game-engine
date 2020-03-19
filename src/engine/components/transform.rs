use cgmath::{Vector3, Quaternion, Matrix4};
use specs::{VecStorage, Component};

use engine::FS;

pub struct Transform {
    pub position: Vector3<FS>,
    pub rotation: Quaternion<FS>,
    pub scale: Vector3<FS>,
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}

impl Default for Transform {
    fn default() -> Transform {
        Transform {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Quaternion::new(1.0, 0.0, 0.0, 0.0),
            scale: Vector3::new(0.0, 0.0, 0.0),
        }
    }
}

impl Transform {
    pub fn model(&self) -> Matrix4<FS> {
        Matrix4::from_translation(self.position)
            * Matrix4::from(self.rotation)
            * Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z)
    }

    pub fn rotate(&mut self, rotation: Quaternion<FS>) {
        self.rotation = self.rotation * rotation;
    }
}
