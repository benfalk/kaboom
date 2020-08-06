use super::assets::Assets;
use super::component::Component;
use crate::minefield::board::Board;
use crate::minefield::location::{
    Location,
    Status::*,
};

use tetra::graphics::{
    self,
    Texture,
    DrawParams,
    Drawable,
    Rectangle,
};

use tetra::math::Vec2;
use tetra::Context;
use tetra::input::MouseButton;

pub struct Minefield {
    board: Board,
    covered_square: Texture,
    flag_square: Texture,
    width: f32,
    height: f32,
    uncovered_texts: [Texture; 9]
}

impl Minefield {
    pub fn new(ctx: &mut Context, board: Board) -> Self {
        Self {
            width: board.width() as f32 * 32.,
            height: board.height() as f32 * 32.,
            board,
            covered_square: Assets::get_texture(ctx, "element_grey_square.png"),
            flag_square: Assets::get_texture(ctx, "flag.png"),
            uncovered_texts: [
                Assets::get_texture(ctx, "blank_square.png"),
                Assets::get_texture(ctx, "one_square.png"),
                Assets::get_texture(ctx, "two_square.png"),
                Assets::get_texture(ctx, "three_square.png"),
                Assets::get_texture(ctx, "four_square.png"),
                Assets::get_texture(ctx, "five_square.png"),
                Assets::get_texture(ctx, "six_square.png"),
                Assets::get_texture(ctx, "seven_square.png"),
                Assets::get_texture(ctx, "eight_square.png"),
            ],
        }
    }
}

impl Component for Minefield {
    fn rectangle<P: Into<DrawParams>>(&self, params: P) -> Rectangle {
        let params: DrawParams = params.into();

        Rectangle::new(
            params.position.x,
            params.position.y,
            self.width,
            self.height,
        )
    }

    fn click(&mut self, pos: Vec2<f32>, button: MouseButton) {
        let x = ( pos.x / 32. ) as i32;
        let y = ( pos.y / 32. ) as i32;

        match button {
            MouseButton::Left => self.board.uncover_location_at(x, y),
            MouseButton::Right => self.board.toggle_flag(x, y),
            _ => (),
        }
    }
}

impl Drawable for Minefield {
    fn draw<P: Into<DrawParams>>(&self, ctx: &mut Context, params: P) {
        let params: DrawParams = params.into();

        for x in 0..self.board.width() {
            for y in 0..self.board.height() {
                let vec2 = params.position + Vec2::new(x as f32 * 32., y as f32 * 32.);

                match self.board.location_at(x, y) {
                    Location { status: Flagged, .. } =>
                        graphics::draw(ctx, &self.flag_square, vec2),

                    Location { status: Covered, .. } =>
                        graphics::draw(ctx, &self.covered_square, vec2),

                    Location { status: Uncovered, surrounding_bomb_count: count, .. } => {
                        graphics::draw(ctx, &self.uncovered_texts[*count as usize], vec2);
                    }
                }
            }
        }
    }
}
