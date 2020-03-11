use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Color {
    #[serde(default)]
    pub r: u8,
    #[serde(default)]
    pub g: u8,
    #[serde(default)]
    pub b: u8,
    #[serde(default = "default_alpha")]
    pub a: u8
}

fn default_alpha() -> u8 { 255 }

impl Default for Color {
    fn default() -> Self { Color::white() }
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }

    pub fn white() -> Color {
        Color::new(255, 255, 255, 255)
    }

    pub fn black() -> Color {
        Color::new(0, 0, 0, 255)
    }

    pub fn red() -> Color {
        Color::new(255, 0, 0, 255)
    }

    pub fn green() -> Color {
        Color::new(0, 255, 0, 255)
    }

    pub fn blue() -> Color {
        Color::new(0, 0, 255, 255)
    }
}