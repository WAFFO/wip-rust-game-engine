use cgmath::{Vector3, Quaternion};
use specs::{Component, VecStorage};

use engine::FS;

pub struct Velocity {
    pub position: Vector3<FS>,
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

impl Default for Velocity {
    fn default() -> Velocity {
        Velocity {
            position: Vector3::new(0.0, 0.0, 0.0),
        }
    }
}
