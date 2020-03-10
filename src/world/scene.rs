use crate::world::manager::Manager;
use crate::world::node::Node;

pub struct Scene<'a> {
    pub name: String,

    pub root: Node<'a>,

    pub textures: Vec<(String, String)>
}

impl<'a> Scene<'a> {
    pub fn new(name: &str) -> Scene<'a> {
        Scene {
            name: String::from(name),
            root: Node::new("ROOT"),
            textures: Vec::new()
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
        for t in self.textures.iter() {
            manager.textures.register(&t.0, &t.1);
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

/*
    pub fn get_node(&self, name: &str ) -> &Node {
        
    }*/
}