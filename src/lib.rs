// External stuff
extern crate wasm_bindgen;
extern crate js_sys;
extern crate web_sys;
extern crate specs;

use wasm_bindgen::prelude::*;

// Testing (tests.rs)
#[cfg(test)]
mod tests;

// My stuff
pub mod application;
pub mod engine;
pub mod game;
pub mod renderer;
pub mod timer;
pub mod javascript;
pub mod math;

// bring to the surface
//pub use application::Application;
pub use engine::Engine;
pub use game::Game;