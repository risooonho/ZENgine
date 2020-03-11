use crate::graphics::color::Color;
use crate::engine::JsonBuilder;
use crate::world::node::NodeDeclaration;
use crate::engine::ResourceDeclaration;
use serde::{Deserialize};

use crate::world::manager::Manager;
use crate::world::node::Node;
use crate::assets::text_loader;

#[derive(Deserialize)]
pub struct SceneDeclaration {
    pub name: String,

    #[serde(default)]
    pub resources: ResourceDeclaration,

    #[serde(default)]
    pub background: Color,

    #[serde(default)]
    pub root: NodeDeclaration
}

pub struct Scene {
    pub name: String,

    pub resources: ResourceDeclaration,

    pub background: Color,

    pub root: Node
}

impl Scene {
    pub fn new(name: &str) -> Scene {
        Scene {
            name: String::from(name),
            resources: ResourceDeclaration::default(),

            background: Color::default(),
            root: Node::new("ROOT")
        }
    }

    pub fn get_root(&mut self) -> &mut Node {
        &mut self.root
    }

    pub fn get_node(&mut self, name: &str) -> Option<&mut Node> {
        for n in self.root.children.iter_mut() {
            if n.name == name {
                return Some(n);
            }
        }

        None
    }

    pub fn declare_resource(&self, manager: &mut Manager) {
        for s in self.resources.shaders.iter() {
            manager.shaders.register(&s.name, &s.file);
        }

        for t in self.resources.textures.iter() {
            manager.textures.register(&t.name, &t.file);
        }        
    }

    pub fn load(&mut self, manager: &Manager) {
        self.root.load(manager);
    }

    pub fn update(&mut self, time: f32) {
        self.root.update(time, None);
    }

    pub fn render(&self) {
        self.root.render();
    }

    pub fn declare_from_json(name: &str, json_file: &str, json_builder: &JsonBuilder) -> Scene {
        let mut scene = Scene::new(name);

        let scene_json = text_loader::load(json_file);
        let scene_declaration: SceneDeclaration = serde_json::from_str(&scene_json.data).unwrap();

        scene.resources = scene_declaration.resources;

        scene.background = scene_declaration.background;
        scene.root = scene_declaration.root.create_node(json_builder);

        scene
    }

/*
    pub fn get_node(&self, name: &str ) -> &Node {
        
    }*/
}