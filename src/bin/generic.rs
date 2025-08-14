use abstract_factory::{
    abstract_factories::generic_factories::{
        JsonGenericPersistenceFactory, SqlGenericPersistenceFactory,
    },
    apps::generic_app::GenericApp,
};

fn main() {
    let json_factory = JsonGenericPersistenceFactory {};
    let sql_factory = SqlGenericPersistenceFactory {};

    let json_app = GenericApp::from_generic_factory(&json_factory);

    let sql_app = GenericApp::from_generic_factory(&sql_factory);

    println!("--- Running json app ---");
    json_app.store();
    json_app.load();
    println!("");

    println!("--- Running sql app ---");
    sql_app.store();
    sql_app.load();
    println!("");

    // Play around a bit.

    // Can't get both factories into the same collection, as we would need to specify
    // the generic types in the trait object, but we can't (as they differ).
    // let factories: Vec<Box<dyn GenericPersistenceFactory<JsonStorer, JsonLoader>>> =
    //     vec![Box::new(json_factory), Box::new(sql_factory)];
}
