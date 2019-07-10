use specs::{Component, EntityBuilder, Entity, RunNow, Builder};
use specs::shred::{Resource};

use crate::engine::Engine;
use crate::engine::input::{KeyBoard, Mouse};
use engine::mesh_manager::UUID;

#[macro_export]
macro_rules! create_entity {
    ( $eng:ident, $( $x:expr ),* ) => {
        $eng.create_entity()
            $(.with($x)
            )*
            .build()
    }
}

impl Engine {
    pub fn delta(&self) -> f32 {
        self.timer.get_delta() as f32
    }
    pub fn register_component<T: Component>(&mut self)
    where T::Storage: Default {
        self.world.register::<T>();
    }
    pub fn register_resource<T: Resource>(&mut self, resource: T) {
        self.world.add_resource(resource);
    }
    pub fn register_commit(&mut self) {
        self.world.maintain();
    }
    pub fn create_entity(&mut self) -> EntityBuilder {
        self.world.create_entity()
    }
    pub fn run_system<'a, T: RunNow<'a>>(&'a mut self, system: &'a mut T) {
        system.run_now(&mut self.world.res)
    }

    pub fn get_mouse(&self) -> Mouse {
        self.world.read_resource::<Mouse>().clone()
    }
    pub fn get_key_board(&self) -> KeyBoard {
        self.world.read_resource::<KeyBoard>().clone()
    }

    pub fn load_mesh(&mut self, id: UUID) -> UUID {
        self.mesh_manager.load(id)
    }
}