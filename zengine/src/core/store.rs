use crate::core::component::Component;
use crate::core::component::Components;
use crate::core::entity::Entities;
use crate::core::entity::Entity;
use crate::core::entity::EntityBuilder;

#[derive(Default, Debug)]
pub struct Store {
    entities: Entities,
    components: Components,
}

impl Store {
    pub fn build_entity(&mut self) -> EntityBuilder {
        EntityBuilder::new(self.entities.create_entity(), self)
    }

    pub fn delete_entity(&mut self, entity: &Entity) {
        self.components.delete_entity(entity);
    }

    pub fn insert_component<C: Component>(&mut self, entity: &Entity, component: C) {
        self.components.insert(entity, component);
    }
}
