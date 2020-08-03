use crate::system::Game;
use crate::system::render_system::RenderSystem;
use ggez::{event, Context, GameResult};
use specs::{RunNow, WorldExt};
use ggez::event::KeyCode;
use ggez::input::keyboard::KeyMods;
use crate::system::input::InputSystem;

#[derive(Default)]
pub struct InputQueue {
    pub keys_pressed: Vec<KeyCode>,
}

//为游戏实现事件
//event::EventHandler 必须拥有更新(update)和(render)的实现
impl event::EventHandler for Game {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        // Run input system
        {
            let mut is = InputSystem {};
            is.run_now(&self.world);
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        let mut rs = RenderSystem {
            context,
        };
        rs.run_now(&self.world);
        
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _context: &mut Context,
        keycode: KeyCode,
        _key_mod: KeyMods,
        _repeat: bool,
    ) {
        println!("Key pressed: {:?}", keycode);

        let mut input_queue = self.world.write_resource::<InputQueue>();
        input_queue.keys_pressed.push(keycode);
    }
    
    
}
