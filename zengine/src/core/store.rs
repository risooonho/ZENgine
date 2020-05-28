use crate::core::entity::Entities;
use crate::core::entity::Entity;

#[derive(Default, Debug)]
pub struct Store {
  entities: Entities,
}

impl Store {
  pub fn build_entity(&mut self) -> Entity {
    self.entities.create_entity()
  }
}
