use std::{env, path};

use ggez::{conf, ContextBuilder, event, GameResult};
use glam::Vec2;
use crate::game::MicroGolf;

mod game;
pub mod assets;
pub mod velocity;
pub mod barrier;

const RESOURCE_DIR: &str     = "assets";
pub const DESIRED_FPS: u32   = 60;
pub const WINDOW_WIDTH: f32  = 600.0;
pub const WINDOW_HEIGHT: f32 = 600.0;
pub const FRICTION: f32      = 5.0;
pub const MAX_POWER: f32     = 250.0;

// Create a unit vector representing the given angle in radians
pub fn vec_from_angle(angle: f32) -> Vec2 {
    let vx = angle.sin();
    let vy = angle.cos();
    Vec2::new(vx, vy)
}

// clamp power to a max
pub fn clamp_power(power: f32) -> f32 {
    return if power > MAX_POWER {
        MAX_POWER
    } else if power <= 0. {
        0.
    } else {
        power
    }
}

fn main() -> GameResult {
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
        .window_mode(conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .add_resource_path(resource_dir);

    let (mut ctx, events_loop) = cb.build().expect("Failed to build game context");

    let game = MicroGolf::new(&mut ctx).expect("Failed to initialize game object");
    event::run(ctx, events_loop, game)
}