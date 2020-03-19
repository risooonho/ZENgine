use crate::behaviors::Behavior;

pub enum Hook {
    Action(String, fn(time: f32)),
    Axis(String, fn(time: f32, value: &f32))
}

impl Hook {
    pub fn new_action(name: &str, delegate: fn(time: f32)) -> Hook {
        Hook::Action(
            String::from(name),
            delegate
        )
    }

    pub fn new_axis(name: &str, delegate: fn(time: f32, value: &f32)) -> Hook {
        Hook::Axis(
            String::from(name),
            delegate
        )
    }
}