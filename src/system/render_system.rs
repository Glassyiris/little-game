use ggez::Context;
use ggez::graphics;
use specs::{System, ReadStorage, Join};
use crate::ecs::position::Position;
use crate::ecs::renderable::Renderable;
use ggez::graphics::{Image, DrawParam};
use ggez::nalgebra as na;

use crate::system::TILE_WIDTH;

pub struct RenderSystem<'a> {
    pub context: &'a mut Context,
}

impl<'a> System<'a> for RenderSystem<'a> {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a , Renderable>);

    fn run(&mut self, data: Self::SystemData) {
        let (positions, renderables) = data;

        //Clearing the Screen
        graphics::clear(self.context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));

        // Get all the renderables with their positions and sort by the position z
        // This will allow us to have entities layered visually.
        let mut rendering_date = (&positions, &renderables)
            .join().collect::<Vec<_>>();
        rendering_date.sort_by(|&a, &b| {
            a.0.z.partial_cmp(&b.0.z).expect("error cmp")
        });

        //rendering specified position
        for (position, renderable) in rendering_date {
            //load game resources
            let image = Image::new(self.context, renderable.path.clone()).expect(
                "error image"
            );
            let x = position.x as f32 * TILE_WIDTH;
            let y = position.y as f32 * TILE_WIDTH;

            // draw
            let draw_params = DrawParam::new().dest(na::Point2::new(x, y));
            graphics::draw(self.context, &image, draw_params).expect("expected render");
        }

        // Finally, present the context, this will actually display everything
        // on the screen.
        graphics::present(self.context).expect("expected to present");
    }
}
