use super::vector3::Vector3;

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

    pub fn distance(a: Vector2, b: &Vector2) -> f32 {
        let mut diff = a;
        diff.subtract( b );
        return f32::sqrt( diff.x * diff.x + diff.y * diff.y );
    }

    pub fn set(&mut self, x: Option<f32>, y: Option<f32>) {
        if let Some(estracted_x) = x {
            self.x = estracted_x;
        }

        if let Some(estracted_y) = y {
            self.y = estracted_y;
        }
    }

    pub fn equals(&self, v: &Vector2) -> bool {
        self.x == v.x && self.y == v.y
    }

    pub fn to_array(&self) -> [f32; 2] {
        [self.x, self.y]
    }

    pub fn copy_from(&mut self, v: &Vector2) {
        self.x = v.x;
        self.y = v.y;
    }

    pub fn add(&mut self, v: &Vector2) -> &Vector2 {
        self.x += v.x;
        self.y += v.y;

        self
    }

    pub fn subtract(&mut self, v: &Vector2) -> &Vector2 {
        self.x -= v.x;
        self.y -= v.y;

        self
    }

    pub fn multiply(&mut self, v: &Vector2) -> &Vector2 {
        self.x *= v.x;
        self.y *= v.y;

        self
    }

    pub fn divide(&mut self, v: &Vector2) -> &Vector2 {
        self.x /= v.x;
        self.y /= v.y;

        self
    }

    pub fn scale(&mut self, scale: f32) -> &Vector2 {
        self.x *= scale;
        self.y *= scale;

        self
    }

    pub fn to_vector3(&self) -> Vector3 {
        Vector3::new( self.x, self.y, 0.0 )
    }
}