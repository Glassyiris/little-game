use specs::World;
use crate::system::event::InputQueue;


pub mod event;
pub mod render_system;
pub mod map;
pub mod input;

pub const TILE_WIDTH: f32 = 32.0;
pub const MAP_WIDTH: u8 = 8;
pub const MAP_HEIGHT: u8 = 9;

pub struct Game {
    pub world: World
}

pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default())
}

