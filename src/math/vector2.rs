#[derive(Copy, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 {
            x: x,
            y: y
        }
    }

    pub fn zero() -> Vector2 {
        Vector2 {
            x: 0.0,
            y: 0.0
        }
    }

    pub fn one() -> Vector2 {
        Vector2 {
            x: 1.0,
            y: 1.0
        }
    }
}