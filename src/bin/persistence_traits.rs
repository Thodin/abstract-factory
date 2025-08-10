use abstract_factory::{
    apps::persistence_traits::PersistenceTraitsApp,
    persistence::{JsonStorer, PostgresLoader},
};

fn main() {
    // Semantic error: app stores to json, but loads from postgres!
    let app = PersistenceTraitsApp::new(JsonStorer {}, PostgresLoader {});

    println!("--- Running persistence traits app ---");
    app.store();
    app.load();
}
