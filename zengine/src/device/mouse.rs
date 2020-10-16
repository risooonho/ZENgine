#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MouseButton {
    Left = sdl2::mouse::MouseButton::Left as u8,
    Middle = sdl2::mouse::MouseButton::Middle as u8,
    Right = sdl2::mouse::MouseButton::Right as u8,
}
