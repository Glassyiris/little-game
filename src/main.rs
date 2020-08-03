use ggez::{conf, event, GameResult};
use specs::{
    Builder, Component, ReadStorage, RunNow, System, VecStorage, World, WorldExt,
};

use std::path;

use crate::ecs::register_component;
pub mod ecs;
pub mod system;
use system::Game;
use system::map::*;
use crate::system::register_resources;

pub fn main() -> GameResult {
    let mut world = World::new();
    register_component(&mut world);
    register_resources(&mut world);
    initialize_level(&mut world);

    // Create a game context and event loop
    let context_builder = ggez::ContextBuilder::new("rust_sokoban", "sokoban")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (context, event_loop) = &mut context_builder.build()?;

    // Create the game state
    let game = &mut Game { world };
    // Run the main event loop
    event::run(context, event_loop, game)
}