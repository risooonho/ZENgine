use crate::core::component::Component;
use crate::core::store::Store;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct Entity(u32);

#[derive(Default, Debug)]
pub struct Entities {
    current_id: u32,
}

impl Entities {
    pub fn create_entity(&mut self) -> Entity {
        let id = self.current_id;
        self.current_id += 1;

        Entity(id)
    }
}

#[derive(Debug)]
pub struct EntityBuilder<'a> {
    entity: Entity,
    store: &'a mut Store,
    is_build: bool,
}

impl<'a> EntityBuilder<'a> {
    pub fn new(entity: Entity, store: &'a mut Store) -> Self {
        EntityBuilder {
            entity: entity,
            store: store,
            is_build: false,
        }
    }

    pub fn with<C: Component>(self, component: C) -> Self {
        self.store.insert_component(&self.entity, component);

        self
    }

    pub fn build(mut self) -> Entity {
        self.is_build = true;

        self.entity
    }
}

impl<'a> Drop for EntityBuilder<'a> {
    fn drop(&mut self) {
        if !self.is_build {
            self.store.delete_entity(&self.entity);
        }
    }
}
