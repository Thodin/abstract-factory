use abstract_factory::{
    abstract_factories::impl_factories::{JsonImplPersistenceFactory, SqlImplPersistenceFactory},
    apps::boxed_app::BoxedApp,
};

fn main() {
    // Same as generic factory, impl is only a bit of syntax sugar here.
    let json_factory = JsonImplPersistenceFactory {};
    let sql_factory = SqlImplPersistenceFactory {};

    let json_app = BoxedApp::from_impl_factory(&json_factory);

    let sql_app = BoxedApp::from_impl_factory(&sql_factory);

    println!("--- Running json app ---");
    json_app.store();
    json_app.load();
    println!("");

    println!("--- Running sql app ---");
    sql_app.store();
    sql_app.load();
    println!("");
}
