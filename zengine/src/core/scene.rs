use crate::core::store::Store;

pub enum Trans {
    None,
    Quit,
}

pub trait Scene {
    fn on_start(&mut self, store: &mut Store) {}

    fn on_stop(&mut self, store: &mut Store) {}

    fn update(&mut self, store: &mut Store) -> Trans {
        Trans::None
    }
}
