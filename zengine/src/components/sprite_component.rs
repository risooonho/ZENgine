use crate::components::ComponentDeclaration;
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

    #[serde(default)]
    origin: Vector3,

    #[serde(default = "basic_shader")]
    shader: String,

    #[serde(default)]
    material: Material
}

fn basic_shader() -> String { "basic".to_string() }

pub struct SpriteComponent {
    name: String,

    origin: Vector3,

    sprite: Sprite
}

impl Component for SpriteComponent {
    fn load(&mut self, manager: &Manager) {
        self.sprite.load(
            manager.shaders.get(&self.sprite.shader_name),
            manager.textures.get(&self.sprite.material.texture)
        );
    }

    fn render(&self, owner_world_matrix: &Matrix4x4) {
        self.sprite.draw(owner_world_matrix);
    }
}

impl SpriteComponent {
    pub fn new(name: &str, width: f32, height: f32, origin: Vector3, shader: &str, material: Material) -> SpriteComponent {
        let mut c = SpriteComponent {
            name: String::from(name),

            origin: origin,

            sprite: Sprite::new(shader, material, Some(width), Some(height))
        };

        c.sprite.set_origin(c.origin);

        c
    }

    pub fn json_builder(declaration: &ComponentDeclaration) -> Box<dyn Component> {
        let scd: SpriteComponentDeclaration = declaration.decode_data();

        let mut c = SpriteComponent {
            name: String::from(&declaration.name),

            origin: scd.origin,

            sprite: Sprite::new(&scd.shader, scd.material, Some(scd.width), Some(scd.height))
        };

        c.sprite.set_origin(c.origin);

        Box::new(c)
    }
}