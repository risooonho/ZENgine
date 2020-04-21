use std::cell::RefCell;
use std::rc::Rc;
use crate::math::transform::Transform;
use crate::Event;
use crate::engine::JsonBuilder;
use crate::components::ComponentDeclaration;
use serde::{Deserialize};

use crate::world::manager::Manager;
use crate::components::Component;
use crate::math::matrix4x4::Matrix4x4;

fn default_as_true() -> bool {
    true
}

#[derive(Deserialize, Default)]
pub struct NodeDeclaration {
    pub name: String,

    #[serde(default)]
    pub children: Vec<NodeDeclaration>,

    #[serde(default="default_as_true")]
    pub active: bool,

    #[serde(default)]
    pub transform: Transform,

    #[serde(default)]
    pub components: Vec<ComponentDeclaration>
}

impl NodeDeclaration {
    pub fn create_node(&self, json_builder: &JsonBuilder) -> Node {
        let mut node = Node::new(&self.name);

        node.state.active = self.active;
        node.state.transform = self.transform;

        for c in &self.components {
            node.components.push( 
                match json_builder.components.get(&c.r#type) {
                    Some(builder) => Rc::new(RefCell::new(*builder(c))),
                    None => panic!("No builder for component with type {}", c.r#type)
                }
            );
        }

        for nd in &self.children {
            node.children.push(nd.create_node(json_builder));
        }

        node
    }
}

pub struct Node {
    pub name: String,

    pub children: Vec<Node>,

    pub state: State,

    pub local_matrix: Matrix4x4,
    pub world_matrix: Matrix4x4,

    pub components: Vec<Rc<RefCell<dyn Component>>>
}

pub struct State {
    pub active: bool,
    pub transform: Transform,
}

impl Node {
    pub fn new(name: &str) -> Node {
        Node {
            name: String::from(name),

            children: Vec::new(),

            state: State {
                active: true,
                transform: Transform::new()
            },

            local_matrix: Matrix4x4::identity(),
            world_matrix: Matrix4x4::identity(),
            
            components: Vec::new()
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.children.push(node);
    }

    pub fn add_component(&mut self, component: impl Component + 'static) {
        self.components.push(Rc::new(component));
    }

    /*pub fn get_component_id(&self, component_name: &str) -> Option<usize> {
        self.components.iter().position(|c| c.is_my_name(component_name))
    }

    pub fn get_component<T>(&self, component_id: usize) -> &T where T: Component {
        self.components[component_id].as_ref().as_any().downcast_ref::<T>().unwrap()
    }*/

    pub fn load(&mut self, manager: &Manager) {
        for c in self.components.iter_mut() {
            c.load(self, manager)
        }

        for n in self.children.iter_mut() {
            n.load(manager)
        }
    }

    pub fn propagate_event(&mut self, delta: f32, event: &Event) {   
        for b in self.components.iter_mut() {
            b.event_hub(delta, event);
        }

        for n in self.children.iter_mut() {
            n.propagate_event(delta, event)
        }
    }

    pub fn tick(&mut self, delta: f32, parent_world: Option<&Matrix4x4>) {

        self.local_matrix = self.state.transform.get_transformation_matrix();
        match parent_world {
            Some(parent_world) => self.world_matrix = parent_world * self.local_matrix,
            None => self.world_matrix = self.local_matrix
        }

        for c in self.components.iter_mut() {
            c.tick(self, delta);
        }

        for n in self.children.iter_mut() {
            n.tick(delta, Some(&self.world_matrix));
        }
    }

    pub fn render(&self) {
        if self.state.active {
            for c in self.components.iter() {
                c.render(&self.world_matrix);
            }

            for n in self.children.iter() {
                n.render();
            }
        }
    }
}