use abstract_factory::{
    abstract_factories::associated_type_factories::{
        AssociatedTypePersistenceFactory, JsonAssociatedTypePersistenceFactory,
        PostgresAssociatedTypePersistenceFactory,
    },
    apps::generic_app::GenericApp,
    persistence::Storer,
};

fn main() {
    let json_factory = JsonAssociatedTypePersistenceFactory {};
    let postgres_factory = PostgresAssociatedTypePersistenceFactory {};

    let json_app = GenericApp {
        loader: json_factory.create_loader(),
        storer: json_factory.create_storer(),
    };

    let postgres_app = GenericApp {
        loader: postgres_factory.create_loader(),
        storer: postgres_factory.create_storer(),
    };

    println!("--- Running json app ---");
    json_app.store();
    json_app.load();
    println!("");

    println!("--- Running postgres app ---");
    postgres_app.store();
    postgres_app.load();
    println!("");

    // Play around a bit.

    // If we want multiple storers in the same collection, we still need to box (obviously!).
    let _storers: Vec<Box<dyn Storer>> = vec![
        Box::new(json_factory.create_storer()),
        Box::new(postgres_factory.create_storer()),
    ];

    // Can't get both factories into the same collection, as we would need to specify
    // the associated types in the trait object, but we can't (as they differ).
    // let factories: Vec<
    //     Box<
    //         dyn AssociatedTypePersistenceFactory<
    //                 CreatedLoader = JsonLoader,
    //                 CreatedStorer = JsonStorer,
    //             >,
    //     >,
    // > = vec![Box::new(json_factory), Box::new(postgres_factory)];
}
