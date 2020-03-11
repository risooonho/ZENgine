use serde::{Deserialize};

use crate::world::manager::Manager;
use crate::math::matrix4x4::Matrix4x4;
use crate::components::Component;
use crate::math::vector3::Vector3;
use crate::graphics::material::Material;
use crate::graphics::sprite::Sprite;

#[derive(Deserialize)]
pub struct SpriteComponentDeclaration {
    width: f32,

    height: f32,

    origin: Vector3,

    shader_name: String,

    material: Material
}

pub struct SpriteComponent {
    name: String,

    origin: Vector3,

    sprite: Sprite
}

impl Component for SpriteComponent {
    /*fn json_builder(name: &str, data: serde_json::Value) -> Self {
        let scd: SpriteComponentDeclaration = serde_json::from_value(data).unwrap();

        let mut c = SpriteComponent {
            name: String::from(name),

            origin: scd.origin,

            sprite: Sprite::new(&scd.shader_name, scd.material, Some(scd.width), Some(scd.height))
        };

        c.sprite.set_origin(c.origin);

        c
    }*/

    fn load(&mut self, manager: &Manager) {
        self.sprite.load(
            manager.shaders.get(&self.sprite.shader_name),
            manager.textures.get(&self.sprite.material.texture_name)
        );
    }

    fn render(&self, owner_world_matrix: &Matrix4x4) {
        self.sprite.draw(owner_world_matrix);
    }
}

impl SpriteComponent {
    pub fn new(name: &str, width: f32, height: f32, origin: Vector3, shader_name: &str, material: Material) -> SpriteComponent {
        let mut c = SpriteComponent {
            name: String::from(name),

            origin: origin,

            sprite: Sprite::new(shader_name, material, Some(width), Some(height))
        };

        c.sprite.set_origin(c.origin);

        c
    }
}