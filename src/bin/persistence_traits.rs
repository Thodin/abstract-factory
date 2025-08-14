use abstract_factory::{
    apps::persistence_traits::PersistenceTraitsApp,
    persistence::{JsonStorer, SqlLoader},
};

fn main() {
    // Semantic error: app stores to json, but loads from sql!
    let app = PersistenceTraitsApp::new(JsonStorer {}, SqlLoader {});

    println!("--- Running persistence traits app ---");
    app.store();
    app.load();
}
