mod game;
pub mod assets;
pub mod ball;

use std::{env, path};
use ggez::{ContextBuilder, conf};

const RESOURCE_DIR: &str = "assets";

fn main() {
    // Add the CARGO_MANIFEST_DIR/assets to the resource paths
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push(RESOURCE_DIR);
        path
    } else {
        path::PathBuf::from(format!("./{}", RESOURCE_DIR).as_str())
    };

    // build the context
    let cb = ContextBuilder::new("MicroGolf", "Sk3pz")
        .window_setup(conf::WindowSetup::default().title("MicroGolf"))
        .window_mode(conf::WindowMode::default().dimensions(600, 400))
        .add_resource_path(resource_dir);

    let (mut ctx, events_loop) = cb.build().expect("Failed to build game context");


}