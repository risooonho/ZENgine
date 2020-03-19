use serde::de::DeserializeOwned;
use serde::{Deserialize}; 
use crate::world::manager::Manager;
use crate::math::matrix4x4::Matrix4x4;

pub mod sprite_component;

pub trait Component {    
    fn load(&mut self, manager: &Manager);

    fn update(&self) {}

    fn render(&self, owner_world_matrix: &Matrix4x4);
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
