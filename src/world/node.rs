use serde::{Deserialize};

use crate::world::manager::Manager;
use crate::components::Component;
use crate::behaviors::Behavior;
use crate::math::matrix4x4::Matrix4x4;
use crate::math::transform::Transform;

#[derive(Deserialize)]
pub struct Node<'a> {
    pub name: String,

    #[serde(default)]
    pub children: Vec<Node<'a>>,

    #[serde(default)]
    pub visible: bool,

    #[serde(default)]
    pub transform: Transform,

    #[serde(skip_deserializing)]
    pub local_matrix: Matrix4x4,

    #[serde(skip_deserializing)]
    pub world_matrix: Matrix4x4,

    #[serde(skip_deserializing)]
    pub components: Vec<Box<dyn Component<'a> + 'a>>,
    #[serde(skip_deserializing)]
    pub behaviors: Vec<Box<dyn Behavior + 'a>>
}

impl<'a> Node<'a> {
    pub fn new(name: &str) -> Node {
        Node {
            name: String::from(name),

            children: Vec::new(),

            visible: true,

            transform: Transform::new(),
            local_matrix: Matrix4x4::identity(),
            world_matrix: Matrix4x4::identity(),

            components: Vec::new(),
            behaviors: Vec::new()
        }
    }

    pub fn add_node(&mut self, node: Node<'a>) {
        self.children.push(node);
    }

    pub fn add_component(&mut self, component: impl Component<'a> + 'a) {
        self.components.push(Box::new(component));
    }

    pub fn add_behavior(&mut self, behavior: impl Behavior + 'a) {
        self.behaviors.push(Box::new(behavior));
    }

    pub fn load(&mut self, manager: &'a Manager) {
        for c in self.components.iter_mut() {
            c.load(manager)
        }

        for n in self.children.iter_mut() {
            n.load(manager)
        }
    }

    pub fn update(&mut self, time: f32, parent_world: Option<&Matrix4x4>) {

        self.local_matrix = self.transform.get_transformation_matrix();
        match parent_world {
            Some(parent_world) => self.world_matrix = parent_world * self.local_matrix,
            None => self.world_matrix = self.local_matrix
        }

        for c in self.components.iter_mut() {
            c.update();
        }

        for b in self.behaviors.iter_mut() {
            b.update(time, &mut self.transform);
        }

        for n in self.children.iter_mut() {
            n.update(time, Some(&self.world_matrix));
        }
    }

    pub fn render(&self) {
        if self.visible {
            for c in self.components.iter() {
                c.render(&self.world_matrix);
            }

            for n in self.children.iter() {
                n.render();
            }
        }
    }
}