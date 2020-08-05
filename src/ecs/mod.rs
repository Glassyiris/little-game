use specs::{World, WorldExt, Builder};

pub mod box_spot;
pub mod boxer;
pub mod player;
pub mod wall;
pub mod renderable;
pub mod position;
pub mod movable;

use position::Position;
use wall::Wall;
use player::Player;
use box_spot::BoxSpot;
use boxer::Boxer;
use renderable::Renderable;
use crate::ecs::movable::{Immovable, Movable};

pub fn register_component(world: &mut World) {
    world.register::<Position>();
    world.register::<Wall>();
    world.register::<Player>();
    world.register::<BoxSpot>();
    world.register::<Boxer>();
    world.register::<Renderable>();
    world.register::<Immovable>();
    world.register::<Movable>();
}

pub fn create_wall(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable {
            path: "/images/wall.png".to_string(),
        })
        .with(Wall {})
        .with(Immovable)
        .build();
}

pub fn create_floor(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 5, ..position })
        .with(Renderable {
            path: "/images/floor.png".to_string(),
        })
        .build();
}

pub fn create_box(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable {
            path: "/images/box.png".to_string(),
        })
        .with(Boxer {})
        .with(Movable)
        .build();
}

pub fn create_box_spot(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 9, ..position })
        .with(Renderable {
            path: "/images/box_spot.png".to_string(),
        })
        .with(BoxSpot {})
        .build();
}

pub fn create_player(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable {
            path: "/images/player.png".to_string(),
        })
        .with(Player {})
        .with(Movable)
        .build();
}