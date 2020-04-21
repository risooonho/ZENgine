use core::any::Any;
use crate::world::node::Node;
use crate::math::matrix4x4::Matrix4x4;
use crate::components::Component;
use crate::math::vector3::Vector3;
use serde::{Deserialize};

#[derive(Deserialize, Default)]
pub struct TransformDeclaration {
    #[serde(default)]
    pub position: Vector3,
    #[serde(default)]
    pub rotation: Vector3,
    #[serde(default = "Vector3::one")]
    pub scale: Vector3
}

pub struct TransformComponent {
    name: String,

    pub position: Vector3,
    pub rotation: Vector3,
    pub scale: Vector3,

    pub local_matrix: Matrix4x4,
    pub world_matrix: Matrix4x4,
}

impl Component for TransformComponent {
    fn is_my_name(&self, name: &str) -> bool {
        if self.name == name {
            true
        } else {
            false
        }
    }

    fn as_any(&self) -> &dyn Any { self }

    fn tick(&mut self, node: &Node, delta: f32) {
        /*self.local_matrix = self.transform.get_transformation_matrix();
        match parent_world {
            Some(parent_world) => self.world_matrix = parent_world * self.local_matrix,
            None => self.world_matrix = self.local_matrix
        }*/
    }
}

impl TransformComponent {
    pub fn new(name: &str, position: Vector3, rotation: Vector3, scale: Vector3) -> TransformComponent {
        TransformComponent {
            name: String::from(name),

            position: position,
            rotation: rotation,
            scale: scale,

            local_matrix: Matrix4x4::identity(),
            world_matrix: Matrix4x4::identity(),
        }
    }

    pub fn get_transformation_matrix(&self) -> Matrix4x4 {
        let translation = Matrix4x4::translation(self.position);
        let rotation = Matrix4x4::rotation(self.rotation);
        let scale = Matrix4x4::scale(self.scale);

        translation * rotation * scale
    }
}