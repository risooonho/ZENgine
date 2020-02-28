use crate::graphics::texture::TextureManager;
use crate::math::matrix4x4::Matrix4x4;

pub mod sprite_component;

pub trait Component<'a> {
    fn load(&mut self, texture_manager: &'a TextureManager);

    fn update(&self) {}

    fn render(&self, owner_world_matrix: &Matrix4x4);
}
