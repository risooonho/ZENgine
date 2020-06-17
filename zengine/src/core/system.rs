use crate::core::component::Component;
use crate::core::component::Set;
use crate::core::store::Store;
use std::any::Any;
use std::cell::Ref;
use std::fmt::Debug;

pub trait AnySystem {
    fn init(&mut self, store: &mut Store) {}

    fn run_now(&mut self, store: &Store);

    fn dispose(&mut self, store: &mut Store) {}
}

impl<S> AnySystem for S
where
    S: for<'a> System<'a>,
{
    fn init(&mut self, store: &mut Store) {
        self.init(store);
    }

    fn run_now(&mut self, store: &Store) {
        let data = S::Data::fetch(store);
        self.run(data);
    }

    fn dispose(&mut self, store: &mut Store) {
        self.dispose(store);
    }
}

pub trait System<'a>: Any + Debug {
    type Data: Data<'a>;

    fn init(&mut self, store: &mut Store) {}

    fn run(&mut self, data: Self::Data);

    fn dispose(&mut self, store: &mut Store) {}
}

pub trait Data<'a> {
    fn fetch(store: &'a Store) -> Self;
}

pub type ReadSet<'a, C> = Ref<'a, Set<C>>;

impl<'a, C: Component> Data<'a> for ReadSet<'a, C> {
    fn fetch(store: &'a Store) -> Self {
        store.get_components::<C>().unwrap()
    }
}
