use crate::world::node::Node;

pub struct Scene<'a> {
    pub root: Node<'a>,
}

impl<'a> Scene<'a> {
    pub fn new() -> Scene<'a> {
        Scene {
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

    pub fn load(&mut self) {
        self.root.load();
    }

    pub fn update(&mut self) {
        self.root.update(None);
    }

    pub fn render(&self) {
        self.root.render();
    }

/*
    pub fn get_node(&self, name: &str ) -> &Node {
        
    }*/
}