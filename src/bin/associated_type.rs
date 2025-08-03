use abstract_factory::{
    abstract_factories::associated_type_factories::{
        AssociatedTypePersistenceFactory, JsonAssociatedTypePersistenceFactory,
        PostgresAssociatedTypePersistenceFactory,
    },
    apps::generic_app::GenericApp,
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
}
