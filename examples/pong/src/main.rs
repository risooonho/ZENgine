extern crate zengine;

use zengine::core::Component;
use zengine::core::Scene;
use zengine::core::Store;
use zengine::core::Trans;
use zengine::Engine;

fn main() {
    Engine::default().run(Game {
        execution_number: 10,
    });
}

#[derive(Debug)]
pub struct Position {
    x: f32,
    y: f32,
}

#[derive(Debug)]
pub struct Test {
    data: u32,
}

impl Component for Position {}
impl Component for Test {}

pub struct Game {
    execution_number: u32,
}

impl Scene for Game {
    fn on_start(&mut self, store: &mut Store) {
        println!("Game scene on start");

        let e = store
            .build_entity()
            .with(Position { x: 43.0, y: 3.5 })
            .with(Test { data: 5 })
            .build();
        println!("Entity {:?}", e);

        {
            let e2 = store.build_entity().with(Test { data: 5 }).build();

            println!("Entity2 {:?}", e2);
        }

        println!("Store {:?}", store);
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
