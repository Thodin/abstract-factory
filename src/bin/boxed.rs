use abstract_factory::{
    abstract_factories::boxed_factories::{
        BoxedPersistenceFactory, JsonBoxedPersistenceFactory, PostgresBoxedPersistenceFactory,
    },
    apps::boxed_app::BoxedApp,
};

fn main() {
    let json_factory = JsonBoxedPersistenceFactory {};
    let postgres_factory = PostgresBoxedPersistenceFactory {};

    let json_app = BoxedApp::from_boxed_factory(&json_factory);

    let postgres_app = BoxedApp::from_boxed_factory(&postgres_factory);

    println!("--- Running json app ---");
    json_app.store();
    json_app.load();
    println!("");

    println!("--- Running postgres app ---");
    postgres_app.store();
    postgres_app.load();
    println!("");

    // Can even move both factories in a common collection, because they exhibit the same type.
    let _factories: Vec<Box<dyn BoxedPersistenceFactory>> =
        vec![Box::new(json_factory), Box::new(postgres_factory)];
}
