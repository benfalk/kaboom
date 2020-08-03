use super::super::minefield::board::Board;

use tetra::graphics::{self, Color, Texture};
use tetra::{Context, State};
use tetra::math::Vec2;

impl State for Board {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        let rect: Texture = Texture::new(ctx, "resources/element_grey_square.png")?;
        let rec_w = rect.width() as f32;
        let rec_h = rect.height() as f32;

        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        for x in 0..self.width() {
            for y in 0..self.height() {
                let vec2 = Vec2::new(x as f32 * rec_w, y as f32 * rec_h);
                graphics::draw(ctx, &rect, vec2);
            }
        }

        Ok(())
    }
}
