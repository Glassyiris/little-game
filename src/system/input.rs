use specs::{System, Write, WriteStorage, ReadStorage, Join};
use crate::system::event::InputQueue;
use crate::ecs::position::Position;
use ggez::{Context, GameResult};
use ggez::event::KeyCode;
use crate::ecs::player::Player;

pub struct InputSystem {}

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Write<'a, InputQueue>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut input_queue, mut positions, players) = data;
        //Get the first key
        for (position, _player) in (&mut positions, &players).join() {
            if let Some(key) = input_queue.keys_pressed.pop() {
                match key {
                    KeyCode::Up => position.y -= 1,
                    KeyCode::Down => position.y += 1,
                    KeyCode::Left => position.x -= 1,
                    KeyCode::Right => position.x += 1,
                    _ => (),
                }
            }
        }
    }
}