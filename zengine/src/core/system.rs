use crate::core::store::Store;
use std::any::Any;
use std::fmt::Debug;

pub trait System: Any + Debug {
    fn init(&mut self, store: &mut Store) {}

    fn run(&mut self, store: &Store);

    fn dispose(&mut self, store: &mut Store) {}
}
