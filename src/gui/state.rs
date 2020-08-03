use super::super::minefield::board::Board;

use tetra::graphics::{self, Color, Texture};
use tetra::{Context, State};
use tetra::math::Vec2;

pub struct GameState {
    board: Board,
    covered_square: Texture,
}

impl GameState {
    pub fn new(ctx: &mut Context, board: Board) -> Self {
        Self {
            board,
            covered_square: Texture::new(ctx, "resources/element_grey_square.png").unwrap()
        }
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        let rec_w = self.covered_square.width() as f32;
        let rec_h = self.covered_square.height() as f32;


        for x in 0..self.board.width() {
            for y in 0..self.board.height() {
                let vec2 = Vec2::new(x as f32 * rec_w, y as f32 * rec_h);
                graphics::draw(ctx, &self.covered_square, vec2);
            }
        }

        Ok(())
    }
}
