pub mod controller;
pub mod keyboard;
pub mod mouse;

use crate::input::controller::Which;
use crate::input::keyboard::Key;
use crate::input::controller::ControllerButton;
use crate::input::mouse::MouseButton;

use sdl2::event::Event;

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum Axis {
    X,
    Y
}

#[derive(Debug)]
pub enum ActionType {
    Pressed,
    Released
}

pub enum InputEvent {
    Action(String),
    Axis(String, f32)
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum Input {
    Keyboard { key: Key },
    MouseMotion { axis: Axis },
    MouseButton { button: MouseButton },
    ControllerStick { which: Which, axis: Axis },
    ControllerTrigger { which: Which },
    ControllerButton { button: ControllerButton }
}

pub struct Action {
    pub name: String,
    pub events: Vec<Input>
}

pub struct InputMapping {
    pub action_mapping: Vec<Action>,
    pub axis_mapping: Vec<Action>
}

impl InputMapping {
    pub fn new() -> InputMapping {
        InputMapping {
            action_mapping: Vec::new(),
            axis_mapping: Vec::new()
        }
    }
}

#[derive(Debug)]
pub struct InputData {
    pub input: Input,
    pub value: f32
}

#[derive(Debug)]
pub enum InputFromEvent {
    Single(InputData),
    Double(InputData, InputData),
    None
}

impl Input {
    pub fn input_from_event(event: &sdl2::event::Event) -> InputFromEvent {
        match event {
            Event::KeyDown { keycode: Some(keycode), .. } => 
                InputFromEvent::Single( 
                    InputData {
                        input: Input::Keyboard { key: Key::from_sdl2_key(keycode) },
                        value: 1.0
                    }
                ),
            Event::KeyUp { keycode: Some(keycode), .. } => 
                InputFromEvent::Single( 
                    InputData {
                        input: Input::Keyboard { key: Key::from_sdl2_key(keycode) },
                        value: 0.0
                    }
                ),
            Event::MouseMotion { x, y, .. } => 
                InputFromEvent::Double(
                    InputData {
                        input: Input::MouseMotion { axis: Axis::X },
                        value: *x as f32
                    },
                    InputData {
                        input: Input::MouseMotion { axis: Axis::Y },
                        value: *y as f32
                    }
                ),
            Event::MouseButtonDown { mouse_btn, .. } =>
                InputFromEvent::Single(
                    InputData {
                        input: Input::MouseButton { button: MouseButton::from_sdl_button(mouse_btn) },
                        value: 1.0
                    }
                ),
            Event::MouseButtonUp { mouse_btn, .. } =>
                InputFromEvent::Single( 
                    InputData {
                        input: Input::MouseButton { button: MouseButton::from_sdl_button(mouse_btn) },
                        value: 0.0
                    }
                ),

            Event::ControllerButtonDown { button, .. } =>
                InputFromEvent::Single( 
                    InputData {
                        input: Input::ControllerButton { button: ControllerButton::from_sdl_button(button) },
                        value: 1.0
                    }
                ),
            Event::ControllerButtonUp { button, .. } =>
                InputFromEvent::Single(
                    InputData {
                        input: Input::ControllerButton { button: ControllerButton::from_sdl_button(button) },
                        value: 0.0
                    }
                ),
            
            Event::ControllerAxisMotion { axis, value, .. } => 
                InputFromEvent::Single(
                    InputData {
                        input: match axis {
                            sdl2::controller::Axis::LeftX => Input::ControllerStick { which: Which::Left, axis: Axis::X },
                            sdl2::controller::Axis::LeftY => Input::ControllerStick { which: Which::Left, axis: Axis::Y },
                            sdl2::controller::Axis::RightX => Input::ControllerStick { which: Which::Right, axis: Axis::X },
                            sdl2::controller::Axis::RightY => Input::ControllerStick { which: Which::Right, axis: Axis::Y },
                            sdl2::controller::Axis::TriggerLeft => Input::ControllerTrigger { which: Which::Left },
                            sdl2::controller::Axis::TriggerRight => Input::ControllerTrigger { which: Which::Right },
                            _ => panic!("Cannot convert Controller Axis Motion")
                        },
                        value: {
                            if *value > -8000i16 && *value < 8000i16 {                                
                                0.0
                            } else { 
                                if (*value).is_positive() { 
                                    (*value) as f32 / std::i16::MAX as f32 
                                } else {
                                    ((*value) as f32).abs() / std::i16::MIN as f32 
                                } 
                            }
                        }
                    }
                ),
            _ => InputFromEvent::None
        }
    }
}