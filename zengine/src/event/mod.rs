use crate::core::Resource;
use crate::event::input::Input;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub mod input;
pub mod input_system;
pub mod stream;

pub trait InputType: Any + Eq + PartialEq + Hash + Clone + Debug {}

impl InputType for String {}

#[derive(Debug)]
pub struct ActionBind {
    pub source: Input,
}

#[derive(Debug)]
pub struct AxisBind {
    pub source: Input,
    pub scale: f32,
}

#[derive(Debug)]
pub struct Bindings<T: InputType> {
    pub action_mappings: HashMap<T, Vec<ActionBind>>,
    pub axis_mappings: HashMap<T, Vec<AxisBind>>,
}

pub struct InputHandler<T: InputType> {
    actions_value: HashMap<T, bool>,
    axes_value: HashMap<T, f32>,
}

impl<T: InputType> Default for InputHandler<T> {
    fn default() -> Self {
        InputHandler {
            actions_value: HashMap::default(),
            axes_value: HashMap::default(),
        }
    }
}

impl<T: InputType> Resource for InputHandler<T> {}

impl<T: InputType> InputHandler<T> {
    pub fn axis_value(&self, input_type: T) -> Option<f32> {
        self.axes_value.get(&input_type).copied()
    }

    pub fn action_value(&self, input_type: T) -> Option<bool> {
        self.actions_value.get(&input_type).copied()
    }
}
