use crate::math::transform::Transform;

pub trait Behavior {
    #[allow(unused_variables)]
    fn update(&self, time: f32, owner_transform: &mut Transform) {}
}