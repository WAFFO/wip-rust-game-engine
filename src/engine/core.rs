
use specs::{Component, EntityBuilder, Entity, RunNow, Builder};
use specs::shred::{Resource};

use crate::engine::Engine;
use crate::engine::input::{KeyBoard, Mouse};

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
    pub fn create_entity(&mut self, factory: &Fn(EntityBuilder) -> EntityBuilder) -> Entity {
        factory(self.world.create_entity()).build()
    }
    pub fn run_system<'a, T: RunNow<'a>>(&'a mut self, system: &'a mut T) {
        system.run_now(&mut self.world.res)
    }
    pub fn get_mouse(&self) -> Mouse {
        self.mouse.clone()
    }
    pub fn get_key_board(&self) -> KeyBoard {
        self.key_board.clone()
    }

}