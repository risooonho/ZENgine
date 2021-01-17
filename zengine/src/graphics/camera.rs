use crate::core::entity::Entity;
use crate::core::Component;
use crate::core::Resource;
use crate::math::matrix4x4::Matrix4x4;

pub struct ActiveCamera {
    pub entity: Entity,
}
impl Resource for ActiveCamera {}

#[derive(Debug)]
pub enum CameraMode {
    Mode2D,
}

#[derive(Debug)]
pub struct Camera {
    pub width: u32,
    pub height: u32,
    pub mode: CameraMode,
}

impl Component for Camera {}

impl Camera {
    pub fn get_projection(&self) -> Matrix4x4 {
        match self.mode {
            CameraMode::Mode2D => Matrix4x4::orthographics(
                -(self.width as f32 / 2.0),
                self.width as f32 / 2.0,
                -(self.height as f32 / 2.0),
                self.height as f32 / 2.0,
                0.0,
                1000.0,
            ),
        }
    }
}
