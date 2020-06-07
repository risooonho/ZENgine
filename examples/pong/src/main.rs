extern crate zengine;

use zengine::core::Component;
use zengine::core::Scene;
use zengine::core::Store;
use zengine::core::System;
use zengine::core::Trans;
use zengine::Engine;

fn main() {
    Engine::default()
        .with_system(System1 {})
        .with_system(System2 {})
        .run(Game {
            execution_number: 10,
        });
}

#[derive(Debug)]
pub struct System1 {}

impl System for System1 {
    fn init(&mut self, store: &mut Store) {
        println!("System 1 init");
    }

    fn run(&mut self, store: &mut Store) {
        let test = store.get_components_mut::<Test>().unwrap();

        for t in test.values_mut() {
            t.data += 1;
        }

        println!("System 1 data {:?}", test);
    }

    fn dispose(&mut self, store: &mut Store) {
        println!("System 1 dispose");
    }
}

#[derive(Debug)]
pub struct System2 {}

impl System for System2 {
    fn init(&mut self, store: &mut Store) {
        println!("System 2 init");
    }

    fn run(&mut self, store: &mut Store) {
        println!("System 2 run");
    }

    fn dispose(&mut self, store: &mut Store) {
        println!("System 2 dispose");
    }
}

#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct Test {
    pub data: u32,
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
        //println!("Entity {:?}", e);

        {
            let e2 = store.build_entity().with(Test { data: 5 }).build();

            //println!("Entity2 {:?}", e2);
        }

        //println!("Store {:?}", store);
    }

    fn on_stop(&mut self, store: &mut Store) {
        println!("Game scene on stop");
    }

    fn update(&mut self, store: &mut Store) -> Trans {
        match self.execution_number {
            0 => Trans::Quit,
            _ => {
                //println!("Store {:?}", store);
                self.execution_number -= 1;
                Trans::None
            }
        }
    }
}
