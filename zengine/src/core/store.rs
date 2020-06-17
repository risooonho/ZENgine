use crate::core::component::Component;
use crate::core::component::Components;
use crate::core::component::Set;
use crate::core::entity::Entities;
use crate::core::entity::Entity;
use crate::core::entity::EntityBuilder;
use std::cell::Ref;
use std::cell::RefMut;

#[derive(Default, Debug)]
pub struct Store {
    entities: Entities,
    components: Components,
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
}
