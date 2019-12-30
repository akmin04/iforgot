mod app;
mod config;
mod subcommands;

fn main() {
    let matches = app::init();
    app::init_logger(&matches);
    let mut commands = config::read().unwrap();
    app::handle_subcommands(&matches, &mut commands);
    config::write(&commands);
}
