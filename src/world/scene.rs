use crate::engine::ResourceDeclaration;
use serde::{Deserialize};

use crate::world::manager::Manager;
use crate::world::node::Node;
use crate::assets::text_loader;

#[derive(Deserialize)]
pub struct Scene<'a> {
    pub name: String,

    #[serde(default)]
    pub resources: ResourceDeclaration,

    pub root: Node<'a>
}

impl<'a> Scene<'a> {
    pub fn new(name: &str) -> Scene<'a> {
        Scene {
            name: String::from(name),
            resources: ResourceDeclaration::default(),
            root: Node::new("ROOT")
        }
    }

    pub fn get_root(&mut self) -> &mut Node<'a> {
        &mut self.root
    }

    pub fn get_node(&mut self, name: &str) -> Option<&mut Node<'a>> {
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

    pub fn load(&mut self, manager: &'a Manager) {
        self.root.load(manager);
    }

    pub fn update(&mut self, time: f32) {
        self.root.update(time, None);
    }

    pub fn render(&self) {
        self.root.render();
    }

    pub fn declare_from_json(&mut self, json_file: &str) {
        let scene_json = text_loader::load(json_file);
        let scene_deserialized: Scene = serde_json::from_str(&scene_json.data).unwrap();

        self.name = scene_deserialized.name;
        self.resources = scene_deserialized.resources;
        self.root = scene_deserialized.root;
    }

/*
    pub fn get_node(&self, name: &str ) -> &Node {
        
    }*/
}