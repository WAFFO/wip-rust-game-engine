use wasm_bindgen;
use wasm_bindgen::prelude::*;
use specs::{World};

pub mod core;
pub mod input;
pub mod components;
pub mod entities;
pub mod resources;
pub mod systems;
pub mod mesh_manager;

use self::components::*;
use self::resources::*;
use self::mesh_manager::MeshManager;
use self::input::*;
use renderer::Renderer;
use timer::Timer;


// Engine
#[wasm_bindgen]
pub struct Engine {
    world: World,
    //entities: Vec<Entity>,
    renderer: Renderer,
    timer: Timer,
    mesh_manager: MeshManager,
}

impl Engine {
    pub fn new(canvas : String) -> Result<Engine, JsValue> {

        let mut world = World::new();

        world.register::<Transform>();
        world.register::<Velocity>();
        world.register::<StaticMesh>();
        world.register::<Camera>();
        world.register::<Solid>();
        world.register::<Light>();
        world.register::<PlayerController>();

        world.add_resource(DeltaTime(0.0));
        world.add_resource(Mouse::new());
        world.add_resource(KeyBoard::new());

        world.maintain();

        //let entities : Vec<Entity> = Vec::new();

        let renderer = Renderer::new(canvas)?;

        let timer = Timer::new();

        let mesh_manager = MeshManager::new();

        Ok(Engine {
            world,
            //entities,
            renderer,
            timer,
            mesh_manager,
        })
    }

    pub fn tick(&mut self) {
        {
            // first tick delta time
            let mut _delta = self.world.write_resource::<DeltaTime>();
            _delta.0 = self.timer.tick_delta() as f32;
        }

        // do engine stuff here?
    }

    pub fn render(&mut self) -> Result<(), JsValue> {
        self.renderer.draw(&self.world, &mut self.mesh_manager)
    }
}