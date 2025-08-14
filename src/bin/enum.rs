use abstract_factory::{
    abstract_factories::enum_factories::{PersistenceFactoryEnum, StorerEnum},
    apps::enum_app::EnumApp,
};

fn main() {
    let json_factory = PersistenceFactoryEnum::Json {};
    let sql_factory = PersistenceFactoryEnum::Sql {};

    let json_app = EnumApp::from_enum_factory(&json_factory);
    let sql_app = EnumApp::from_enum_factory(&sql_factory);

    println!("--- Running json app ---");
    json_app.store();
    json_app.load();
    println!("");

    println!("--- Running sql app ---");
    sql_app.store();
    sql_app.load();
    println!("");

    // Can easily store storers in the same collection as they are variants of the same enum.
    let _storers: Vec<StorerEnum> = vec![json_factory.create_storer(), sql_factory.create_storer()];

    // Same for the factories
    let _factories: Vec<PersistenceFactoryEnum> = vec![json_factory, sql_factory];
}
