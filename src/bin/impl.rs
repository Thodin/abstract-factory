use abstract_factory::{
    abstract_factories::impl_factories::{
        JsonImplPersistenceFactory, PostgresImplPersistenceFactory,
    },
    apps::boxed_app::BoxedApp,
};

fn main() {
    // Same as generic factory, impl is only a bit of syntax sugar here.
    let json_factory = JsonImplPersistenceFactory {};
    let postgres_factory = PostgresImplPersistenceFactory {};

    let json_app = BoxedApp::from_impl_factory(&json_factory);

    let postgres_app = BoxedApp::from_impl_factory(&postgres_factory);

    println!("--- Running json app ---");
    json_app.store();
    json_app.load();
    println!("");

    println!("--- Running postgres app ---");
    postgres_app.store();
    postgres_app.load();
    println!("");
}
