extern crate zengine;

use std::collections::HashMap;
use zengine::core::system::Read;
use zengine::core::system::ReadEntities;
use zengine::core::system::ReadSet;
use zengine::core::system::WriteSet;
use zengine::core::timing::{FrameLimiter, TimingSystem};
use zengine::core::Component;
use zengine::core::Scene;
use zengine::core::Store;
use zengine::core::System;
use zengine::core::Trans;
use zengine::device::controller::{ControllerButton, Which};
use zengine::device::keyboard::Key;
use zengine::device::mouse::MouseButton;
use zengine::event::input::{Axis, Input};
use zengine::event::input_system::InputSystem;
use zengine::event::Bindings;
use zengine::event::InputHandler;
use zengine::event::InputType;
use zengine::event::{ActionBind, AxisBind};
use zengine::platform::platform_system::PlatformSystem;
use zengine::Engine;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum UserInput {
    Jump,
    Move_x,
}
impl InputType for UserInput {}

fn main() {
    let mut bindings = Bindings::<UserInput> {
        action_mappings: HashMap::default(),
        axis_mappings: HashMap::default(),
    };

    bindings.action_mappings.insert(
        UserInput::Jump,
        vec![
            ActionBind {
                source: Input::Keyboard { key: Key::Space },
            },
            ActionBind {
                source: Input::ControllerButton {
                    device_id: 1,
                    button: ControllerButton::A,
                },
            },
            ActionBind {
                source: Input::MouseButton {
                    button: MouseButton::Left,
                },
            },
        ],
    );

    bindings.axis_mappings.insert(
        UserInput::Move_x,
        vec![
            AxisBind {
                source: Input::Keyboard { key: Key::A },
                scale: -1.0,
            },
            AxisBind {
                source: Input::Keyboard { key: Key::D },
                scale: 1.0,
            },
            AxisBind {
                source: Input::ControllerStick {
                    device_id: 1,
                    which: Which::Left,
                    axis: Axis::X,
                },
                scale: 1.0,
            },
        ],
    );

    Engine::default()
        .with_system(PlatformSystem::default())
        .with_system(InputSystem::new(bindings))
        .with_system(System1 {})
        .with_system(System2 {})
        .with_system(TimingSystem::default().with_limiter(FrameLimiter::new(60)))
        .run(Game {
            execution_number: 10,
        });
}

#[derive(Debug)]
pub struct System1 {}

impl<'a> System<'a> for System1 {
    type Data = (
        WriteSet<'a, Test>,
        WriteSet<'a, Position>,
        ReadSet<'a, Test2>,
        ReadEntities<'a>,
        Read<'a, InputHandler<UserInput>>,
    );

    fn init(&mut self, store: &mut Store) {
        println!("System 1 init");
    }

    fn run(&mut self, (mut test, mut position, test2, entities, input_handler): Self::Data) {
        for t in test.values_mut() {
            t.data += 1;
        }
        for t in position.values_mut() {
            t.x += 1.0;
        }

        //println!("data {:?}", test);
        //println!("data {:?}", position);
        //println!("test2 {:?}", test2);
        //println!("entities {:?}", entities);

        println!(
            "JUMP VALUE {:?}",
            input_handler.action_value(UserInput::Jump)
        );
        println!(
            "MOVE X VALUE {:?}",
            input_handler.axis_value(UserInput::Move_x)
        );

        /*let mut test = store.get_components_mut::<Test>().unwrap();

        let mut test2 = store.get_components_mut::<Position>().unwrap();

        for t in test.values_mut() {
            t.data += 1;
        }

        for t in test2.values_mut() {
            t.x += 1.0;
        }

        println!("System 1 data {:?}", test);
        println!("System 1 data2 {:?}", test2);*/
    }

    fn dispose(&mut self, store: &mut Store) {
        println!("System 1 dispose");
    }
}

#[derive(Debug)]
pub struct System2 {}

impl<'a> System<'a> for System2 {
    type Data = ();
    fn init(&mut self, store: &mut Store) {
        println!("System 2 init");
    }

    fn run(&mut self, data: Self::Data) {
        //println!("System 2 run");
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

#[derive(Debug)]
pub struct Test2 {
    pub data2: u32,
}

impl Component for Position {}
impl Component for Test {}
impl Component for Test2 {}

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
            0 => Trans::None,
            _ => {
                //println!("Store {:?}", store);
                self.execution_number -= 1;
                Trans::None
            }
        }
    }
}
