
use specs::{Read, ReadStorage, WriteStorage, System};

use engine::components::*;
use engine::resources::*;
use glm::Quat;

// systems
pub struct UpdatePosition;

impl<'a> System<'a> for UpdatePosition {
    type SystemData = (Read<'a, DeltaTime>, WriteStorage<'a, Transform>, ReadStorage<'a, Velocity>);

    fn run(&mut self, (delta, mut pos, vel): Self::SystemData) {
        use specs::Join;

        // Read implements DeRef
        let delta = delta.0;

        for (pos, vel) in (&mut pos, &vel).join() {
            pos.position += vel.position * delta;
        }
    }
}

pub struct UpdateRotation;

impl<'a> System<'a> for UpdateRotation {
    type SystemData = (Read<'a, DeltaTime>, WriteStorage<'a, Transform>, ReadStorage<'a, AngularVelocity>);

    fn run(&mut self, (delta, mut pos, vel): Self::SystemData) {
        use specs::Join;

        // Read implements DeRef
        let delta = delta.0;

        for (pos, vel) in (&mut pos, &vel).join() {
            pos.rotation *= vel.get_quat(delta);
        }
    }
}