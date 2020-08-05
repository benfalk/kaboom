use super::super::minefield::board::Board;
use super::super::minefield::location::{
    Location,
    Status::*,
};

use tetra::graphics::{self, Color, Texture};
use tetra::input::{self, MouseButton};
use tetra::{Context, State};
use tetra::math::Vec2;

use rust_embed::RustEmbed;

pub struct GameState {
    board: Board,
    covered_square: Texture,
    blank_square: Texture,
    flag_square: Texture,
    background_color: Color,
    uncovered_texts: [Texture; 8],
}

#[derive(RustEmbed)]
#[folder = "resources/"]
struct Asset;

impl Asset {
    fn get_texture(ctx: &mut Context, file: &str) -> Texture {
        let file = Self::get(file).unwrap();
        Texture::from_file_data(ctx, file.as_ref()).unwrap()
    }
}

impl GameState {
    pub fn new(ctx: &mut Context, board: Board) -> Self {
        Self {
            board,
            covered_square: Asset::get_texture(ctx, "element_grey_square.png"),
            blank_square: Asset::get_texture(ctx, "blank_square.png"),
            flag_square: Asset::get_texture(ctx, "flag.png"),
            background_color: Color::hex("#92969c"),
            uncovered_texts: [
                Asset::get_texture(ctx, "one_square.png"),
                Asset::get_texture(ctx, "two_square.png"),
                Asset::get_texture(ctx, "three_square.png"),
                Asset::get_texture(ctx, "four_square.png"),
                Asset::get_texture(ctx, "five_square.png"),
                Asset::get_texture(ctx, "six_square.png"),
                Asset::get_texture(ctx, "seven_square.png"),
                Asset::get_texture(ctx, "eight_square.png"),
            ],
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
                let vec2 = Vec2::new(x as f32 * rec_w, y as f32 * rec_h);

                match self.board.location_at(x, y) {
                    Location { status: Flagged, .. } =>
                        graphics::draw(ctx, &self.flag_square, vec2),

                    Location { status: Covered, .. } =>
                        graphics::draw(ctx, &self.covered_square, vec2),

                    Location { status: Uncovered, surrounding_bomb_count: 0, .. } =>
                        graphics::draw(ctx, &self.blank_square, vec2),

                    Location { status: Uncovered, surrounding_bomb_count: count, .. } => {
                        graphics::draw(ctx, &self.uncovered_texts[*count as usize - 1], vec2);
                    }
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
            self.board.uncover_location_at(x, y);
        }

        if input::is_mouse_button_pressed(&ctx, MouseButton::Right) {
            let loc = input::get_mouse_position(&ctx);
            let rec_w = self.covered_square.width() as f32;
            let rec_h = self.covered_square.height() as f32;
            let x = ( loc.x / rec_w ) as i32;
            let y = ( loc.y / rec_h ) as i32;
            self.board.toggle_flag(x, y);
        }

        Ok(())
    }
}
