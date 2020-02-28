use crate::graphics::texture::TextureManager;
use crate::math::matrix4x4::Matrix4x4;
use crate::components::Component;
use crate::math::vector3::Vector3;
use crate::gl_utilities::shader::Shader;
use crate::graphics::material::Material;
use crate::graphics::sprite::Sprite;

pub struct SpriteComponent<'a> {
    name: String,

    origin: Vector3,

    sprite: Sprite<'a>
}

impl<'a> Component<'a> for SpriteComponent<'a> {
    fn load(&mut self, texture_manager: &'a TextureManager) {
        self.sprite.load(texture_manager);
    }

    fn render(&self, owner_world_matrix: &Matrix4x4) {
        self.sprite.draw(owner_world_matrix);
    }
}

impl<'a> SpriteComponent<'a> {
    pub fn new(name: &str, width: f32, height: f32, origin: Vector3, shader: &'a Shader, material: Material<'a>) -> SpriteComponent<'a> {
        let mut c = SpriteComponent {
            name: String::from(name),

            origin: origin,

            sprite: Sprite::new(shader, material, Some(width), Some(height))
        };

        c.sprite.set_origin(c.origin);

        c
    }
}