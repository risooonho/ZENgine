use std::sync::Arc;
use serde::{Deserialize};

use crate::graphics::texture::Texture;
use crate::graphics::color::Color;

#[derive(Deserialize)]
pub struct Material {
    
    #[serde(default)]
    pub tint: Color,

    #[serde(default)]
    pub texture_name: String,

    #[serde(skip_deserializing)]
    texture: Option<Arc<Texture>>
}

impl Default for Material {
    fn default() -> Self { Material::new(Color::default(), None) }
}

impl Material {
    pub fn new(tint: Color, texture_name: Option<&str>) -> Material {
        Material {
            tint: tint,
            texture_name: String::from(match texture_name { Some(texture_name) => texture_name, _ => "" }),
            texture: None
        }
    }

    pub fn set_texture(&mut self, texture: Arc<Texture>) {
        self.texture = Some(texture);
    }

    pub fn use_material(&self, color_location: i32, diffuse_location: i32) {
        unsafe {
            gl::Uniform4f(
                color_location,
                self.tint.r as f32 / 255.0,
                self.tint.g as f32 / 255.0,
                self.tint.b as f32 / 255.0,
                self.tint.a as f32 / 255.0
            );

            match &self.texture {
                Some(texture) => {
                    texture.activate();
                    gl::Uniform1i(diffuse_location, 0);
                },
                _ => {}
            }
        }
    }
}
