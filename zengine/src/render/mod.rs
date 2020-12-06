use crate::core::Resource;
use crate::graphics::color::Color;

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
