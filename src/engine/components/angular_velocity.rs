
use specs::{Component, VecStorage};
use cgmath::{Vector3, Quaternion, Rotation3, Rad};

use engine::FS;

pub struct AngularVelocity {
    pub axis: Vector3<FS>,
    pub angle: FS,
}

impl Component for AngularVelocity {
    type Storage = VecStorage<Self>;
}

impl Default for AngularVelocity {
    fn default() -> AngularVelocity {
        AngularVelocity {
            axis: Vector3::new(0.0, 0.0, 1.0),
            angle: 0.0,
        }
    }
}

impl AngularVelocity {
    pub fn get_quat(&self, delta: FS) -> Quaternion<FS> {
        Quaternion::from_axis_angle( self.axis, Rad(self.angle * delta) )
    }
}