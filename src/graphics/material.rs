use crate::graphics::color::Color;
use crate::graphics::texture::Texture;

pub struct Material<'a> {
    pub texture: &'a Texture,
    pub tint: Color
}

impl<'a> Material<'a> {
    pub fn new(tint: Color, texture: &'a Texture) -> Material {
        Material {
            texture: texture,
            tint: tint
        }
    }
}