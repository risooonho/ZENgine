use std::borrow::Borrow;

use crate::input::InputEvent;
use crate::world::hook::Hook;
use crate::behaviors::BehaviorDeclaration;
use crate::engine::JsonBuilder;
use crate::components::ComponentDeclaration;
use serde::{Deserialize};

use crate::world::manager::Manager;
use crate::components::Component;
use crate::behaviors::Behavior;
use crate::math::matrix4x4::Matrix4x4;
use crate::math::transform::Transform;

#[derive(Deserialize, Default)]
pub struct NodeDeclaration {
    pub name: String,

    #[serde(default)]
    pub children: Vec<NodeDeclaration>,

    #[serde(default)]
    pub visible: bool,

    #[serde(default)]
    pub transform: Transform,

    #[serde(default)]
    pub components: Vec<ComponentDeclaration>,

    #[serde(default)]
    pub behaviors: Vec<BehaviorDeclaration>,
}

impl NodeDeclaration {
    pub fn create_node(&self, json_builder: &JsonBuilder) -> Node {
        let mut node = Node::new(&self.name);

        node.state.visible = self.visible;
        node.state.transform = self.transform;

        for c in &self.components {
            node.components.push( 
                match json_builder.components.get(&c.r#type) {
                    Some(builder) => builder(c),
                    None => panic!("No builder for component with type {}", c.r#type)
                }
            );
        }

        for b in &self.behaviors {
            node.behaviors.push(
                match json_builder.behaviors.get(&b.r#type) {
                    Some(builder) => builder(b),
                    None => panic!("No builder for behavior with type {}", b.r#type)
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

    pub components: Vec<Box<dyn Component>>,
    pub behaviors: Vec<Box<dyn Behavior>>
}

pub struct State {
    pub visible: bool,
    pub transform: Transform
}

impl Node {
    pub fn new(name: &str) -> Node {
        Node {
            name: String::from(name),

            children: Vec::new(),

            state: State {
                visible: true,
                transform: Transform::new()
            },

            local_matrix: Matrix4x4::identity(),
            world_matrix: Matrix4x4::identity(),

            components: Vec::new(),
            behaviors: Vec::new()
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.children.push(node);
    }

    pub fn add_component(&mut self, component: impl Component + 'static) {
        self.components.push(Box::new(component));
    }

    pub fn add_behavior(&mut self, behavior: impl Behavior + 'static) {
        self.behaviors.push(Box::new(behavior));
    }

    pub fn load(&mut self, manager: &Manager) {
        for c in self.components.iter_mut() {
            c.load(manager)
        }

        for b in self.behaviors.iter_mut() {
            b.load();
        }

        for n in self.children.iter_mut() {
            n.load(manager)
        }
    }

    pub fn propagate_input_event(&mut self, time: f32, event: &InputEvent) {   
        for b in self.behaviors.iter_mut() {
            b.event_hub(event);
        }

        for n in self.children.iter_mut() {
            n.propagate_input_event(time, event)
        }
    }

    pub fn update(&mut self, time: f32, parent_world: Option<&Matrix4x4>) {

        self.local_matrix = self.state.transform.get_transformation_matrix();
        match parent_world {
            Some(parent_world) => self.world_matrix = parent_world * self.local_matrix,
            None => self.world_matrix = self.local_matrix
        }

        for c in self.components.iter_mut() {
            c.update();
        }

        for b in self.behaviors.iter_mut() {
            b.update(time, &mut self.state);
        }

        for n in self.children.iter_mut() {
            n.update(time, Some(&self.world_matrix));
        }
    }

    pub fn render(&self) {
        if self.state.visible {
            for c in self.components.iter() {
                c.render(&self.world_matrix);
            }

            for n in self.children.iter() {
                n.render();
            }
        }
    }
}