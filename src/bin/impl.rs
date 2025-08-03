use abstract_factory::{
    abstract_factories::impl_factories::{
        ImplPersistenceFactory, JsonImplPersistenceFactory, PostgresImplPersistenceFactory,
    },
    apps::generic_app::GenericApp,
};

fn main() {
    // Same as generic factory, impl is only a bit of syntax sugar here.
    let json_factory = JsonImplPersistenceFactory {};
    let postgres_factory = PostgresImplPersistenceFactory {};

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
