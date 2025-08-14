use abstract_factory::{
    abstract_factories::associated_type_factories::{
        AssociatedTypePersistenceFactory, JsonAssociatedTypePersistenceFactory,
        SqlAssociatedTypePersistenceFactory,
    },
    apps::generic_app::GenericApp,
    persistence::Storer,
};

fn main() {
    let json_factory = JsonAssociatedTypePersistenceFactory {};
    let sql_factory = SqlAssociatedTypePersistenceFactory {};

    let json_app = GenericApp::from_associated_types_factory(&json_factory);

    let sql_app = GenericApp::from_associated_types_factory(&sql_factory);

    println!("--- Running json app ---");
    json_app.store();
    json_app.load();
    println!("");

    println!("--- Running sql app ---");
    sql_app.store();
    sql_app.load();
    println!("");

    // Play around a bit.

    // If we want multiple storers in the same collection, we still need to box (obviously!).
    let _storers: Vec<Box<dyn Storer>> = vec![
        Box::new(json_factory.create_storer()),
        Box::new(sql_factory.create_storer()),
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
    // > = vec![Box::new(json_factory), Box::new(sql_factory)];
}
