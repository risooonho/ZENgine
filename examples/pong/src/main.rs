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
use zengine::behaviors::Behavior;
use zengine::graphics::color::Color;

use std::collections::HashMap;

fn main() {

    zengine::engine::start(
        zengine::engine::option_from_json("option.json"),
        zengine::engine::resources_declaration_from_json("option.json"),
        vec![
            (String::from("test"), declare_scene)
        ],
        "test"
    );
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
    scene.declare_from_json("scenes/test.json");

    let s_component = SpriteComponent::new("Test", 200.0, 200.0, Vector3::one(), "basic", Material::new(Color::white(), "duck"));

    let b1 = TranslateBehavior {
        value: 100.0,
        axis: 1
    };

    let b2 = TranslateBehavior {
        value: 100.0,
        axis: 2
    };

    /*let mut node = Node::new("prova");
    node.transform.position.y = 300.0;

    node.add_component(s_component);
    node.add_behavior(b1);*/

    scene.get_root().add_component(s_component);
    scene.get_root().add_behavior(b1);
    //scene.get_root().add_node(node);
}
