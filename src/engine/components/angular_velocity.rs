
use specs::{Component, VecStorage};
use glm::{Vec3, Quat};

pub struct AngularVelocity {
    pub axis: Vec3,
    pub angle: f32,
}

impl Component for AngularVelocity {
    type Storage = VecStorage<Self>;
}

impl Default for AngularVelocity {
    fn default() -> AngularVelocity {
        AngularVelocity {
            axis: Vec3::new(0.0, 0.0, 1.0),
            angle: 0.0,
        }
    }
}

impl AngularVelocity {
    pub fn get_quat(&self, delta: f32) -> Quat {
        Quat::from_angle_axis(
            self.angle * delta,
            self.axis,
        )
    }
}