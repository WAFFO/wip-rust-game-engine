
use specs::{World, Builder, Entity};

use engine::components::*;
use engine::mesh_manager::UUID;
use glm::{Vec3, Vec4, Quat};

pub fn test_solid(world: &mut World, mesh: UUID, position: Vec3, rotation: Quat, scale: f32) -> Entity {
    world.create_entity()
        .with(Transform  { position, rotation, scale: Vec3::new(scale, scale, scale) })
        .with(Velocity   { position: Vec3::new(0.0, 0.0, 0.0) })
        .with(StaticMesh { mesh_id: mesh } )
        .with(Solid)
        .build()
}

pub fn test_light(world: &mut World, mesh: UUID, position: Vec3, rotation: Quat, scale: f32) -> Entity {
    world.create_entity()
        .with(Transform  { position, rotation, scale: Vec3::new(scale, scale, scale) })
        .with(Velocity   { position: Vec3::new(0.0, 0.0, 0.0) })
        .with(StaticMesh { mesh_id: mesh } )
        .with(Light { color: Vec4::new(1.0, 1.0, 1.0, 1.0) })
        .build()
}

//pub fn camera(world: &mut World, pitch: f32, yaw: f32) -> Entity {
//    world.create_entity()
//        .with(Camera  { rotation: Vec3::new(0.0, 0.0, 0.0), pitch, yaw, pole_arm: 0.1 })
//        .build()
//}