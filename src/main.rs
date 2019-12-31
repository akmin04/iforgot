mod app;
mod config;
mod subcommands;

use log::error;
use std::process;

fn main() {
    let matches = app::init();
    app::init_logger(&matches);
    let mut commands = match config::read() {
        Ok(c) => c,
        Err(e) => {
            error!("{}", e);
            process::exit(1);
        }
    };

    app::handle_subcommands(&matches, &mut commands);

    if let Err(e) = config::write(&commands) {
        error!("{}", e);
    };
}
