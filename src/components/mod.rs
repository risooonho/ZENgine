use serde::{Deserialize}; 
use crate::world::manager::Manager;
use crate::math::matrix4x4::Matrix4x4;

pub mod sprite_component;

pub trait Component {
    //fn json_builder(data: serde_json::Value) -> Self;
    
    fn load(&mut self, manager: &Manager);

    fn update(&self) {}

    fn render(&self, owner_world_matrix: &Matrix4x4);
}

#[derive(Deserialize)]
pub struct ComponentDeclaration {
    name: String,
    r#type: String,
    data: serde_json::Value
}

pub trait Test {
    fn testiammo() -> Self;

    fn sigh(&self);
}

pub struct Prova {
    data: f32
}

impl Test for Prova {
    fn testiammo() -> Prova {
        Prova { data: 5.0 }
    }

    fn sigh(&self) {

    }
}
