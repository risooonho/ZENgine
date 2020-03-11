use serde::{Deserialize};

use crate::math::matrix4x4::Matrix4x4;
use crate::math::vector3::Vector3;

#[derive(Deserialize, Default, Copy, Clone)]
pub struct Transform {
    #[serde(default)]
    pub position: Vector3,
    #[serde(default)]
    pub rotation: Vector3,
    #[serde(default = "Vector3::one")]
    pub scale: Vector3
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            position: Vector3::zero(),
            rotation: Vector3::zero(),
            scale: Vector3::one()
        }
    }

    pub fn get_transformation_matrix(&self) -> Matrix4x4 {
        let translation = Matrix4x4::translation(self.position);
        let rotation = Matrix4x4::rotation(self.rotation);
        let scale = Matrix4x4::scale(self.scale);

        translation * rotation * scale
    }
}