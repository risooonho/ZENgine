extern crate zengine;

use serde::{Serialize, Deserialize};

use zengine::assets::text_loader;
use zengine::world::scene::Scene;
use zengine::world::node::Node;
use zengine::components::sprite_component::SpriteComponent;
use zengine::math::transform::Transform;
use zengine::graphics::material::Material;
use zengine::graphics::texture::TextureDeclaration;
use zengine::math::vector3::Vector3;
use zengine::graphics::color::Color;

use zengine::components::{Component, ComponentDeclaration};
use zengine::behaviors::{Behavior, BehaviorDeclaration};

use std::collections::HashMap;

fn main() {

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
        value: mb.value,
        axis: mb.axis
    };

    Box::new(b)
}

#[derive(Deserialize)]
struct TranslateBehaviorDeclaration {
    pub value: f32,
    pub axis: u32
}

struct TranslateBehavior {
    pub value: f32,
    pub axis: u32
}

impl Behavior for TranslateBehavior {
    fn update(&self, time: f32, owner_transform: &mut Transform) {
        if self.axis == 1 {
            owner_transform.position.x += self.value * time as f32;
        }
        if self.axis == 2 {
            owner_transform.position.y += self.value * time as f32;
        }
        if self.axis == 3 {
            owner_transform.position.z += self.value * time as f32;
        }
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
