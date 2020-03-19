use specs::{Component, VecStorage};
use cgmath::Vector4;

use engine::FS;
use engine::mesh_manager::UUID;

// components not in this file
mod camera;
mod transform;
mod velocity;
mod angular_velocity;

// raise to this level
pub use self::camera::Camera;
pub use self::transform::Transform;
pub use self::velocity::Velocity;
pub use self::angular_velocity::AngularVelocity;

// components

// TODO: impl std::ops::Add for Transform + Velocity

#[derive(Default)]
pub struct StaticMesh {
    pub mesh_id: UUID,
}

impl Component for StaticMesh {
    type Storage = VecStorage<Self>;
}

// entity types
#[derive(Default)]
pub struct Solid;
impl Component for Solid {
    type Storage = VecStorage<Self>;
}

pub struct Light{
    pub color: Vector4<FS>,
}

impl Component for Light {
    type Storage = VecStorage<Self>;
}

impl Default for Light {
    fn default() -> Light {
        Light {
            color: Vector4::new(1.0, 1.0, 1.0, 1.0),
        }
    }
}

#[derive(Default)]
pub struct PlayerController;

impl Component for PlayerController {
    type Storage = VecStorage<Self>;
}