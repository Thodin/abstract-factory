use crate::{
    abstract_factories::{
        associated_type_factories::AssociatedTypePersistenceFactory,
        generic_factories::GenericPersistenceFactory,
    },
    persistence::{Loader, Storer},
};

pub struct GenericApp<S: Storer, L: Loader> {
    storer: S,
    loader: L,
}

impl<S: Storer, L: Loader> GenericApp<S, L> {
    pub fn from_generic_factory<F: GenericPersistenceFactory<S, L>>(
        persistence_factory: &F,
    ) -> Self {
        Self {
            storer: persistence_factory.create_storer(),
            loader: persistence_factory.create_loader(),
        }
    }

    pub fn from_associated_types_factory<
        F: AssociatedTypePersistenceFactory<CreatedStorer = S, CreatedLoader = L>,
    >(
        persistence_factory: &F,
    ) -> Self {
        Self {
            storer: persistence_factory.create_storer(),
            loader: persistence_factory.create_loader(),
        }
    }

    // Can't create an from the impl trait, because returning `impl Trait` returns an opaque type
    // which the Rust compiler can't directly link to the two generics S and L here. The only way
    // to use this is in the boxed app.

    pub fn store(&self) {
        self.storer.store();
    }

    pub fn load(&self) {
        self.loader.load();
    }
}
