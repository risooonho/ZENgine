use crate::math::transform::Transform;

pub trait Behavior {
    fn update(&self, owner_transform: &mut Transform) {}
}