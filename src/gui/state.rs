use super::super::minefield::board::Board;
use super::super::minefield::location::{
    Location,
    Status::*,
};

use tetra::graphics::{self, Color, Texture};
use tetra::graphics::text::{Text, Font};
use tetra::input::{self, MouseButton};
use tetra::{Context, State};
use tetra::math::Vec2;

pub struct GameState {
    board: Board,
    covered_square: Texture,
    blank_square: Texture,
    font: Font,
    background_color: Color,
}

impl GameState {
    pub fn new(ctx: &mut Context, board: Board) -> Self {
        Self {
            board,
            covered_square: Texture::new(ctx, "resources/element_grey_square.png").unwrap(),
            blank_square: Texture::new(ctx, "resources/blank_square.png").unwrap(),
            font: Font::vector(ctx, "resources/DejaVuSansMono.ttf", 16.0).unwrap(),
            background_color: Color::hex("#92969c"),
        }
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        let rec_w = self.covered_square.width() as f32;
        let rec_h = self.covered_square.height() as f32;
        graphics::clear(ctx, self.background_color);

        for x in 0..self.board.width() {
            for y in 0..self.board.height() {
                let mut vec2 = Vec2::new(x as f32 * rec_w, y as f32 * rec_h);

                match self.board.location_at(x, y) {
                    Location { status: Covered, .. } =>
                        graphics::draw(ctx, &self.covered_square, vec2),

                    Location { status: Uncovered, surrounding_bomb_count: 0, .. } =>
                        graphics::draw(ctx, &self.blank_square, vec2),

                    Location { status: Uncovered, surrounding_bomb_count: count, .. } => {
                        graphics::draw(ctx, &self.blank_square, vec2);
                        vec2 += Vec2::new(12., 8.);
                        graphics::draw(ctx, &Text::new(count.to_string(), self.font.clone()), vec2);
                    },

                    _ =>
                        ()
                }
            }
        }

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) ->  tetra::Result {
        if input::is_mouse_button_pressed(&ctx, MouseButton::Left) {
            let loc = input::get_mouse_position(&ctx);
            let rec_w = self.covered_square.width() as f32;
            let rec_h = self.covered_square.height() as f32;
            let x = ( loc.x / rec_w ) as i32;
            let y = ( loc.y / rec_h ) as i32;
            self.board.location_at_mut(x, y).status = Uncovered;
            println!("{:?}", self.board.location_at(x, y));
        }
        Ok(())
    }
}
