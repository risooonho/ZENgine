use crate::core::store::Store;
use crate::event::stream::EventStream;

pub enum Trans {
    None,
    Quit,
}

pub trait AnyScene {
    fn on_start(&mut self, store: &mut Store);

    fn on_stop(&mut self, store: &mut Store);

    fn update(&mut self, store: &mut Store) -> Trans;
}

impl<S> AnyScene for S
where
    S: Scene,
{
    fn on_start(&mut self, store: &mut Store) {
        self.on_start(store);
    }

    fn on_stop(&mut self, store: &mut Store) {
        self.on_stop(store);
    }

    fn update(&mut self, store: &mut Store) -> Trans {
        let mut received_trans = Trans::None;
        if let Some(stream) = store.get_resource::<EventStream<Trans>>() {
            if let Some(trans) = stream.read_last() {
                received_trans = match trans {
                    Trans::Quit => Trans::Quit,
                    _ => Trans::None,
                }
            }
        }

        match received_trans {
            Trans::Quit => Trans::Quit,
            _ => self.update(store),
        }
    }
}

pub trait Scene {
    fn on_start(&mut self, store: &mut Store) {}

    fn on_stop(&mut self, store: &mut Store) {}

    fn update(&mut self, store: &mut Store) -> Trans {
        Trans::None
    }
}
