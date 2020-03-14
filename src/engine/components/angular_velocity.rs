
use specs::{Component, VecStorage};
use glm::{Vec3, Quat, FSize};

pub struct AngularVelocity {
    pub axis: Vec3,
    pub angle: FSize,
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
    pub fn get_quat(&self, delta: FSize) -> Quat {
        Quat::from_euler_angle(
            self.axis.x(),
            self.axis.y(),
            self.axis.z(),
            self.angle * delta,
        )
    }
}