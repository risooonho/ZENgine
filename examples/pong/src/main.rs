extern crate zengine;

use serde::{Serialize, Deserialize};

use zengine::assets::text_loader;
use zengine::world::scene::Scene;
use zengine::world::node::{Node, State};
use zengine::components::sprite_component::SpriteComponent;
use zengine::math::transform::Transform;
use zengine::graphics::material::Material;
use zengine::graphics::texture::TextureDeclaration;
use zengine::math::vector3::Vector3;
use zengine::graphics::color::Color;

use zengine::components::{Component, ComponentDeclaration};
use zengine::behaviors::{Behavior, BehaviorDeclaration};

use zengine::input::InputMapping;
use zengine::input::{Input, Action, Axis};
use zengine::input::keyboard::Key;
use zengine::input::controller::{Which};
use zengine::world::hook::Hook;

use std::collections::HashMap;

use zengine::input::InputEvent;
use zengine::create_hub;

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
            (String::from("test1"), json_comp_builder)
        ],
        vec![
            (String::from("move"), json_builder)
        ],
        input,
        "test"
    );
}


pub fn json_comp_builder(declaration: &ComponentDeclaration) -> Box<dyn Component> {
    /*let scd: SpriteComponentDeclaration = serde_json::from(data).unwrap();

    let mut c = SpriteComponent {
        name: String::from(name),

        origin: scd.origin,

        sprite: Sprite::new(&scd.shader_name, scd.material, Some(scd.width), Some(scd.height))
    };

    c.sprite.set_origin(c.origin);*/

    let mut c = SpriteComponent::new("ciao", 1.0, 1.0, Vector3::one(), "fdsa", Material::new(Color::red(), None));

    Box::new(c)
}

pub fn json_builder(declaration: &BehaviorDeclaration) -> Box<dyn Behavior> {
    let mb: TranslateBehaviorDeclaration = declaration.decode_data();

    let mut b = TranslateBehavior {
        velocity: mb.velocity,
        x_vel: 0.0,
        y_vel: 0.0,
    };

    Box::new(b)
}

#[derive(Deserialize)]
struct TranslateBehaviorDeclaration {
    pub velocity: f32
}

#[derive(Debug)]
struct TranslateBehavior {
    pub x_vel: f32,
    pub y_vel: f32,

    pub velocity: f32,
}

impl Behavior for TranslateBehavior {

    fn load(&mut self) {
    }

    fn update(&mut self, time: f32, state: &mut State) {
        state.transform.position.x += self.x_vel * time as f32;
        state.transform.position.y += self.y_vel * time as f32;   
    }

    fn event_hub(&mut self, event: &InputEvent) {
        create_hub!(
            self,
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

impl TranslateBehavior {
    pub fn test(&mut self) {
        println!("suca");
    }

    pub fn x_move(&mut self, value: &f32) {
        self.x_vel = self.velocity * value;
    }

    pub fn y_move(&mut self, value: &f32) {
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
