use glm;
use glm::Vec3;
use specs::{Component, VecStorage};

pub struct Velocity {
    pub position: Vec3,
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

impl Default for Velocity {
    fn default() -> Velocity {
        Velocity {
            position: glm::vec3(0.0, 0.0, 0.0),
        }
    }
}
