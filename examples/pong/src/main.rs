extern crate zengine;

use zengine::core::Scene;
use zengine::core::Store;
use zengine::core::Trans;
use zengine::Engine;

fn main() {
    Engine::default().run(Game {
        execution_number: 10,
    });
}

pub struct Game {
    execution_number: u32,
}

impl Scene for Game {
    fn on_start(&mut self, store: &mut Store) {
        println!("Game scene on start");

        let e = store.build_entity();

        println!("Entity {:?}", e);
    }

    fn on_stop(&mut self, store: &mut Store) {
        println!("Game scene on stop");
    }

    fn update(&mut self, store: &mut Store) -> Trans {
        match self.execution_number {
            0 => Trans::Quit,
            _ => {
                println!("Store {:?}", store);
                self.execution_number -= 1;
                Trans::None
            }
        }
    }
}
