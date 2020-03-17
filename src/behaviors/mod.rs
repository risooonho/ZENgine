use crate::world::node::State;
use serde::de::DeserializeOwned;
use serde::{Deserialize}; 

pub trait Behavior {
    #[allow(unused_variables)]
    fn update(&self, time: f32, state: &mut State) {}
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