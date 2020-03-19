use crate::world::hook::Hook;
use crate::world::node::State;
use serde::de::DeserializeOwned;
use serde::{Deserialize}; 

use crate::input::InputEvent;

#[macro_export]
macro_rules! create_hub {
    (
        $self:ident, 
        $event:ident,
        action: [$($action_hook:expr => $action_delegate:ident),*],
        axis: [$($axis_hook:expr => $axis_delegate:ident),*]
    ) => {
        match $event {
            InputEvent::Action(name) => {
                $(if name == $action_hook { $self.$action_delegate(); })*
            },
            InputEvent::Axis(name, value) => {
                $(if name == $axis_hook { $self.$axis_delegate(value); })*
            },
            _ => {}
        }
    };
}

pub trait Behavior {

    #[allow(unused_variables)]
    fn load(&mut self) {}

    #[allow(unused_variables)]
    fn update(&mut self, time: f32, state: &mut State) {}

    fn event_hub(&mut self, event: &InputEvent) {

    }
}

#[derive(Deserialize)]
pub struct BehaviorDeclaration {
    pub name: String,
    pub r#type: String,
    data: serde_json::Value
}

impl BehaviorDeclaration {
    pub fn decode_data<T>(&self) -> T where T: DeserializeOwned {
        match serde_json::from_value(self.data.clone()) {
            Ok(decoded) => return decoded,
            Err(why) => panic!("Cannot decode behavior {} with type {}: {}", self.name, self.r#type, why)
        }
    }
}