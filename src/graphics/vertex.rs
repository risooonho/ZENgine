use crate::math::vector3::Vector3;
use crate::math::vector2::Vector2;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: Vector3,
    pub tex_coord: Vector2
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32, tu: f32, tv: f32) -> Vertex {
        Vertex {
            position: Vector3::new(x, y, z),
            tex_coord: Vector2::new(tu, tv)
        }
    }
}