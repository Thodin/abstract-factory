use abstract_factory::{
    abstract_factories::boxed_factories::{
        BoxedPersistenceFactory, JsonBoxedPersistenceFactory, SqlBoxedPersistenceFactory,
    },
    apps::boxed_app::BoxedApp,
};

fn main() {
    let json_factory = JsonBoxedPersistenceFactory {};
    let sql_factory = SqlBoxedPersistenceFactory {};

    let json_app = BoxedApp::from_boxed_factory(&json_factory);

    let sql_app = BoxedApp::from_boxed_factory(&sql_factory);

    println!("--- Running json app ---");
    json_app.store();
    json_app.load();
    println!("");

    println!("--- Running sql app ---");
    sql_app.store();
    sql_app.load();
    println!("");

    // Can even move both factories in a common collection, because they exhibit the same type.
    let _factories: Vec<Box<dyn BoxedPersistenceFactory>> =
        vec![Box::new(json_factory), Box::new(sql_factory)];
}
