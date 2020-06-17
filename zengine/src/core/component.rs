use crate::core::entity::Entity;
use downcast_rs::Downcast;
use std::any::Any;
use std::any::TypeId;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::fmt::Debug;

pub trait Component: Any + Debug {}

#[derive(Debug, Default)]
pub struct Components {
    storages: HashMap<TypeId, RefCell<Box<dyn AnySet>>>,
}

impl Components {
    pub fn insert<C: Component>(&mut self, entity: &Entity, component: C) {
        let type_id = TypeId::of::<C>();

        match self.storages.get_mut(&type_id) {
            Some(storage) => {
                RefMut::map(storage.borrow_mut(), |b| {
                    b.downcast_mut::<Set<C>>().expect("downcast set error")
                })
                .insert(entity.clone(), component);
            }
            None => {
                let mut storage = Set::<C>::default();
                storage.insert(entity.clone(), component);

                self.storages
                    .insert(type_id, RefCell::new(Box::new(storage)));
            }
        }
    }

    pub fn get<C: Component>(&self) -> Option<Ref<Set<C>>> {
        let type_id = TypeId::of::<C>();

        match self.storages.get(&type_id) {
            Some(storage) => Some(Ref::map(storage.borrow(), |b| {
                b.downcast_ref::<Set<C>>().expect("downcast set error")
            })),
            None => None,
        }
    }

    pub fn get_mut<C: Component>(&self) -> Option<RefMut<Set<C>>> {
        let type_id = TypeId::of::<C>();

        match self.storages.get(&type_id) {
            Some(storage) => Some(RefMut::map(storage.borrow_mut(), |b| {
                b.downcast_mut::<Set<C>>().expect("downcast set error")
            })),
            None => None,
        }
    }

    pub fn remove_entity(&self, entity: &Entity) {
        for s in self.storages.iter() {
            s.1.borrow_mut().remove(&entity);
        }
    }
}

pub type Set<C> = HashMap<Entity, C>;
impl<C: Component> AnySet for Set<C> {
    fn remove(&mut self, entity: &Entity) {
        self.remove(entity);
    }
}

pub trait AnySet: Downcast + Debug {
    fn remove(&mut self, entity: &Entity);
}
downcast_rs::impl_downcast!(AnySet);
