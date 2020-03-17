use std::sync::Arc;
use serde::{Deserialize};

use crate::graphics::texture::Texture;
use crate::graphics::color::Color;

#[derive(Deserialize)]
pub struct Material {
    
    #[serde(default)]
    pub tint: Color,

    #[serde(default)]
    pub texture: String,

    #[serde(skip_deserializing)]
    texture_ref: Option<Arc<Texture>>
}

impl Default for Material {
    fn default() -> Self { Material::new(Color::default(), None) }
}

impl Material {
    pub fn new(tint: Color, texture: Option<&str>) -> Material {
        Material {
            tint: tint,
            texture: String::from(match texture { Some(texture) => texture, _ => "" }),
            texture_ref: None
        }
    }

    pub fn set_texture(&mut self, texture: Arc<Texture>) {
        self.texture_ref = Some(texture);
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

            match &self.texture_ref {
                Some(texture) => {
                    texture.activate();
                    gl::Uniform1i(diffuse_location, 0);
                },
                _ => {}
            }
        }
    }
}
