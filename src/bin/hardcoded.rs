use abstract_factory::apps::hardcoded_app::HardcodedApp;

fn main() {
    let app = HardcodedApp::new();

    println!("--- Running hardcoded app ---");
    app.store();
    app.load();
}
