use crate::core::component::Component;
use crate::core::component::Components;
use crate::core::component::Set;
use crate::core::entity::Entities;
use crate::core::entity::Entity;
use crate::core::entity::EntityBuilder;
use downcast_rs::Downcast;
use std::any::TypeId;
use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
use std::collections::hash_map::HashMap;

pub trait Resource: Downcast + 'static {}
downcast_rs::impl_downcast!(Resource);

#[derive(Default)]
pub struct Store {
    entities: Entities,
    components: Components,
    resources: HashMap<TypeId, RefCell<Box<dyn Resource>>>,
}

impl Store {
    pub fn build_entity(&mut self) -> EntityBuilder {
        EntityBuilder::new(self.entities.create_entity(), self)
    }

    pub fn remove_entity(&self, entity: &Entity) {
        self.components.remove_entity(entity);
    }

    pub fn get_entities(&self) -> &Entities {
        &self.entities
    }

    pub fn get_components<C: Component>(&self) -> Option<Ref<Set<C>>> {
        self.components.get::<C>()
    }

    pub fn get_components_mut<C: Component>(&self) -> Option<RefMut<Set<C>>> {
        self.components.get_mut::<C>()
    }

    pub fn insert_component<C: Component>(&mut self, entity: &Entity, component: C) {
        self.components.insert(entity, component);
    }

    pub fn register_component<C: Component>(&mut self) {
        self.components.register_component::<C>();
    }

    pub fn get_resource<R: Resource>(&self) -> Option<Ref<R>> {
        let type_id = TypeId::of::<R>();

        match self.resources.get(&type_id) {
            Some(resource) => Some(Ref::map(resource.borrow(), |b| {
                b.downcast_ref::<R>().expect("downcast resource error")
            })),
            None => None,
        }
    }

    pub fn get_resource_mut<R: Resource>(&self) -> Option<RefMut<R>> {
        let type_id = TypeId::of::<R>();

        match self.resources.get(&type_id) {
            Some(resource) => Some(RefMut::map(resource.borrow_mut(), |b| {
                b.downcast_mut::<R>().expect("downcast resource error")
            })),
            None => None,
        }
    }

    pub fn insert_resource<R: Resource>(&mut self, resource: R) {
        let type_id = TypeId::of::<R>();
        self.resources
            .insert(type_id, RefCell::new(Box::new(resource)));
    }
}
