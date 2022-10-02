#[macro_use]
extern crate log;

use env_logger::{init_from_env, Env};

// modules
mod bootstrap;
mod entity;
mod game;
mod gfx;
mod overlay;
mod tile;
mod utils;
mod world;

fn main() {
    println!("Loading env_logger");
    // configure the logger
    let env = Env::default()
        .filter_or("LAMBDA_LOG_LEVEL", "trace")
        .write_style_or("LAMBDA_LOG_STYLE", "always");

    init_from_env(env);

    bootstrap::gamebootstrap::bootstrap();
}
