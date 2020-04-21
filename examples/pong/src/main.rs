extern crate zengine;

use zengine::Event;
use zengine::world::manager::Manager;
use serde::{Deserialize};

use zengine::world::scene::Scene;

use zengine::components::{Component, ComponentDeclaration};

use zengine::input::InputMapping;
use zengine::input::{Input, Action, Axis};
use zengine::input::keyboard::Key;
use zengine::input::controller::{Which};

use zengine::input::InputEvent;
use zengine::create_event_hub;

fn main() {

    let mut input = InputMapping::new();

    input.action_mapping.push(
        Action {
            name: String::from("test"),
            events: vec![Input::Keyboard { key: Key::A }]
        }
    );
    input.axis_mapping.push(
        Action {
            name: String::from("x-move"),
            events: vec![
                Input::ControllerStick { which: Which::Left, axis: Axis::X }
            ]
        }
    );
    input.axis_mapping.push(
        Action {
            name: String::from("y-move"),
            events: vec![
                Input::ControllerStick { which: Which::Left, axis: Axis::Y }
            ]
        }
    );

    zengine::engine::start(
        zengine::engine::option_from_json("option.json"),
        zengine::engine::resources_declaration_from_json("option.json"),
        vec![
            (String::from("test"), Some(String::from("scenes/test.json")), Some(declare_scene))
        ],
        vec![
            (String::from("move"), json_builder)
        ],
        input,
        "test"
    );
}

pub fn json_builder(declaration: &ComponentDeclaration) -> Box<dyn Component> {
    let mb: TranslatComponentDeclaration = declaration.decode_data();

    let mut b = TranslateComponent {
        velocity: mb.velocity,
        x_vel: 0.0,
        y_vel: 0.0,
    };

    Box::new(b)
}

#[derive(Deserialize)]
struct TranslatComponentDeclaration {
    pub velocity: f32
}

#[derive(Debug)]
struct TranslateComponent {
    pub x_vel: f32,
    pub y_vel: f32,

    pub velocity: f32,
}

impl Component for TranslateComponent {

    fn load(&mut self, manager: &Manager) {
    }

    fn tick(&mut self, delta: f32) {
        /*state.transform.position.x += self.x_vel * time as f32;
        state.transform.position.y += self.y_vel * time as f32;   */
    }

    fn event_hub(&mut self, delta: f32, event: &Event) {
        create_event_hub!(
            self,
            delta,
            event,
            action: [
                "test" => test
            ],
            axis: [
                "x-move" => x_move,
                "y-move" => y_move
            ]
        );
    }
}

impl TranslateComponent {
    pub fn test(&mut self, delta: f32) {
        println!("suca");
    }

    pub fn x_move(&mut self, delta: f32, value: f32) {
        self.x_vel = self.velocity * value;
    }

    pub fn y_move(&mut self, delta: f32, value: f32) {
        self.y_vel = self.velocity * value;
    }
}

fn declare_scene(scene: &mut Scene) {
    //scene.declare_from_json("scenes/test.json");

    /*let s_component = SpriteComponent::new("Test", 200.0, 200.0, Vector3::one(), "basic", Material::new(Color::white(), "duck"));
*/
    /*let b1 = TranslateBehavior {
        value: 100.0,
        axis: 1
    };
/*
    let b2 = TranslateBehavior {
        value: 100.0,
        axis: 2
    };*/

    /*let mut node = Node::new("prova");
    node.transform.position.y = 300.0;

    node.add_component(s_component);
    node.add_behavior(b1);*/

    //scene.get_root().add_component(s_component);
    scene.get_root().add_behavior(b1);
    //scene.get_root().add_node(node);*/
}
