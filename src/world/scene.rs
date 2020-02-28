use crate::graphics::texture::TextureManager;
use crate::graphics::texture::Texture;
use crate::gl_utilities::shader::Shader;
use crate::world::node::Node;

pub struct Scene<'a> {
    pub root: Node<'a>,

    pub resources: Vec<String>
}

impl<'a> Scene<'a> {
    pub fn new() -> Scene<'a> {
        Scene {
            root: Node::new("ROOT"),
            resources: Vec::new()
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

    pub fn declare_resource(&self, texture_manager: &mut TextureManager) {
        for r in self.resources.iter() {
            texture_manager.register(r);
        }        
    }

    pub fn load(&mut self, texture_manager: &'a TextureManager) {
        self.root.load(texture_manager);
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