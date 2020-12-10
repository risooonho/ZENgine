use crate::core::Component;
use crate::core::Resource;
use crate::graphics::color::Color;
use crate::graphics::texture::SpriteType;
use crate::math::vector3::Vector3;

pub mod render_system;

pub struct WindowSpecs {
    title: String,
    width: u32,
    height: u32,
    fullscreen: bool,
}

impl WindowSpecs {
    pub fn new(title: String, width: u32, height: u32, fullscreen: bool) -> Self {
        WindowSpecs {
            title,
            width,
            height,
            fullscreen,
        }
    }
}

impl Default for WindowSpecs {
    fn default() -> Self {
        WindowSpecs {
            title: String::from("zengine"),
            width: 800,
            height: 600,
            fullscreen: false,
        }
    }
}

pub struct Background {
    pub color: Color,
}
impl Resource for Background {}
impl Default for Background {
    fn default() -> Self {
        Background {
            color: Color::white(),
        }
    }
}

#[derive(Debug)]
pub struct Sprite<ST: SpriteType> {
    pub width: f32,
    pub height: f32,
    pub origin: Vector3,
    pub color: Color,
    pub sprite_type: ST,
}
impl<ST: SpriteType> Component for Sprite<ST> {}
