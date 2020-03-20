use specs::{Component, VecStorage};
use {glm, glm::{Vec3, Quat}};

use engine::FSize;

pub struct AngularVelocity {
    pub axis_normalized: Vec3,
    pub angle: FSize,
}

impl Component for AngularVelocity {
    type Storage = VecStorage<Self>;
}

impl Default for AngularVelocity {
    fn default() -> AngularVelocity {
        AngularVelocity {
            axis_normalized: glm::vec3(0.0, 0.0, 1.0),
            angle: 0.0,
        }
    }
}

impl AngularVelocity {
    pub fn get_quat(&self, delta: FSize) -> Quat {
        glm::quat_angle_axis(
            self.angle * delta,
            &self.axis_normalized,
        )
    }
}