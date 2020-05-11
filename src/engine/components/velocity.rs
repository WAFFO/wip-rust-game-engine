
use specs::{Component, VecStorage};
use glm::Vec3;

pub struct Velocity {
    pub position: Vec3,
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

impl Default for Velocity {
    fn default() -> Velocity {
        Velocity {
            position: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}
