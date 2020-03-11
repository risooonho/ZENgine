use std::sync::Arc;
use serde::{Deserialize};

use crate::graphics::texture::Texture;
use crate::graphics::color::Color;

#[derive(Deserialize)]
pub struct Material {
    pub tint: Color,
    pub texture_name: String,

    #[serde(skip_deserializing)]
    texture: Option<Arc<Texture>>
}

impl Material {
    pub fn new(tint: Color, texture_name: &str) -> Material {
        Material {
            tint: tint,
            texture_name: String::from(texture_name),
            texture: None
        }
    }

    pub fn load(&mut self, texture: Arc<Texture>) {
        self.texture = Some(texture);
    }

    pub fn use_material(&self, color_location: i32, diffuse_location: i32) {
        unsafe {
            gl::Uniform4f(
                color_location,
                self.tint.r,
                self.tint.g,
                self.tint.b,
                self.tint.a
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
