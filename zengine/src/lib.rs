use crate::input::InputEvent;

pub mod engine;
pub mod math;
pub mod graphics;
pub mod gl_utilities;
pub mod assets;
pub mod world;
pub mod components;
pub mod input;

pub enum Event {
    Input(InputEvent)
}