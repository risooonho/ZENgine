use core::any::Any;
use crate::world::node::Node;
use crate::Event;
use crate::input::InputEvent;
use serde::de::DeserializeOwned;
use serde::{Deserialize}; 
use crate::world::manager::Manager;
use crate::math::matrix4x4::Matrix4x4;

pub mod sprite_component;
pub mod transform_component;

#[macro_export]
macro_rules! create_event_hub {
    (
        $self:ident, 
        $delta:ident,
        $event:ident,
        action: [$($action_hook:expr => $action_delegate:ident),*],
        axis: [$($axis_hook:expr => $axis_delegate:ident),*]
    ) => {
        match $event {
            Event::Input(input_event) => match input_event {
                InputEvent::Action(name) => {
                    $(if name == $action_hook { $self.$action_delegate($delta); })*
                },
                InputEvent::Axis(name, value) => {
                    $(if name == $axis_hook { $self.$axis_delegate($delta, value.clone()); })*
                },
                _ => {}
            },
            _ => {}
        }
    };
}


pub trait Component {    
    fn is_my_name(&self, name: &str) -> bool;

    fn as_any(&self) -> &dyn Any;

    fn load(&mut self, node: &Node, manager: &Manager) {}

    fn tick(&mut self, node: &Node, delta: f32) {}

    fn render(&self, owner_world_matrix: &Matrix4x4) {}

    fn event_hub(&mut self, delta: f32, event: &Event) {}
}

#[derive(Deserialize)]
pub struct ComponentDeclaration {
    pub name: String,
    pub r#type: String,
    data: serde_json::Value
}

impl ComponentDeclaration {
    pub fn decode_data<T>(&self) -> T where T: DeserializeOwned {
        match serde_json::from_value(self.data.clone()) {
            Ok(decoded) => return decoded,
            Err(why) => panic!("Cannot decode component {} with type {}: {}", self.name, self.r#type, why)
        }
    }
}
