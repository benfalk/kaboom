use super::super::minefield::board::Board;
use super::super::minefield::location::{
    Location,
    Status::*,
};

use tetra::graphics::{self, Color, Texture};
use tetra::graphics::text::{Text, Font, VectorFontBuilder};
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
    uncovered_texts: [Text; 8],
}

#[derive(RustEmbed)]
#[folder = "resources/"]
struct Asset;

impl Asset {
    fn get_texture(ctx: &mut Context, file: &str) -> Texture {
        let file = Self::get(file).unwrap();
        Texture::from_file_data(ctx, file.as_ref()).unwrap()
    }

    fn get_font(ctx: &mut Context, file: &str, size: f32) -> Font {
        let file = Self::get(file).unwrap();
        let file = Box::new(file);
        let file = Box::leak(file);
        let builder = VectorFontBuilder::from_file_data(file).unwrap();
        builder.with_size(ctx, size).unwrap()
    }
}

impl GameState {
    pub fn new(ctx: &mut Context, board: Board) -> Self {
        let font = Asset::get_font(ctx, "DejaVuSansMono.ttf", 16.0);

        Self {
            board,
            covered_square: Asset::get_texture(ctx, "element_grey_square.png"),
            blank_square: Asset::get_texture(ctx, "blank_square.png"),
            flag_square: Asset::get_texture(ctx, "flag.png"),
            background_color: Color::hex("#92969c"),
            uncovered_texts: [
                Text::new("1", font.clone()),
                Text::new("2", font.clone()),
                Text::new("3", font.clone()),
                Text::new("4", font.clone()),
                Text::new("5", font.clone()),
                Text::new("6", font.clone()),
                Text::new("7", font.clone()),
                Text::new("8", font.clone()),
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
                let mut vec2 = Vec2::new(x as f32 * rec_w, y as f32 * rec_h);

                match self.board.location_at(x, y) {
                    Location { status: Flagged, .. } =>
                        graphics::draw(ctx, &self.flag_square, vec2),

                    Location { status: Covered, .. } =>
                        graphics::draw(ctx, &self.covered_square, vec2),

                    Location { status: Uncovered, surrounding_bomb_count: 0, .. } =>
                        graphics::draw(ctx, &self.blank_square, vec2),

                    Location { status: Uncovered, surrounding_bomb_count: count, .. } => {
                        graphics::draw(ctx, &self.blank_square, vec2);
                        vec2 += Vec2::new(12., 8.);
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
