use crate::world::manager::Manager;
use crate::math::matrix4x4::Matrix4x4;
use crate::components::Component;
use crate::math::vector3::Vector3;
use crate::graphics::material::Material;
use crate::graphics::sprite::Sprite;

pub struct SpriteComponent<'a> {
    name: String,

    origin: Vector3,

    sprite: Sprite<'a>
}

impl<'a> Component<'a> for SpriteComponent<'a> {
    fn load(&mut self, manager: &'a Manager) {
        self.sprite.load(
            manager.shaders.get(&self.sprite.shader_name),
            manager.textures.get(&self.sprite.material.texture_name)
        );
    }

    fn render(&self, owner_world_matrix: &Matrix4x4) {
        self.sprite.draw(owner_world_matrix);
    }
}

impl<'a> SpriteComponent<'a> {
    pub fn new(name: &str, width: f32, height: f32, origin: Vector3, shader_name: &str, material: Material<'a>) -> SpriteComponent<'a> {
        let mut c = SpriteComponent {
            name: String::from(name),

            origin: origin,

            sprite: Sprite::new(shader_name, material, Some(width), Some(height))
        };

        c.sprite.set_origin(c.origin);

        c
    }
}