use abstract_factory::{
    abstract_factories::generic_factories::{
        GenericPersistenceFactory, JsonGenericPersistenceFactory, PostgresGenericPersistenceFactory,
    },
    apps::generic_app::GenericApp,
};

fn main() {
    let json_factory = JsonGenericPersistenceFactory {};
    let postgres_factory = PostgresGenericPersistenceFactory {};

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

    // Can't get both factories into the same collection, as we would need to specify
    // the generic types in the trait object, but we can't (as they differ).
    // let factories: Vec<Box<dyn GenericPersistenceFactory<JsonStorer, JsonLoader>>> =
    //     vec![Box::new(json_factory), Box::new(postgres_factory)];
}
