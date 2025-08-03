use abstract_factory::{
    abstract_factories::boxed_factories::{
        BoxedPersistenceFactory, JsonBoxedPersistenceFactory, PostgresBoxedPersistenceFactory,
    },
    apps::boxed_app::BoxedApp,
};

fn main() {
    let json_factory = JsonBoxedPersistenceFactory {};
    let postgres_factory = PostgresBoxedPersistenceFactory {};

    let json_app = BoxedApp {
        loader: json_factory.create_loader(),
        storer: json_factory.create_storer(),
    };

    let postgres_app = BoxedApp {
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
