use super::minefield::Minefield;
use super::super::minefield::board::Board;
use super::component::Component;

use tetra::graphics::{self, Color};
use tetra::input::{self, MouseButton};
use tetra::{Context, State};
use tetra::math::Vec2;

pub struct GameState {
    minefield: Minefield,
    minefield_origin: Vec2<f32>,
    background_color: Color,
}

impl GameState {
    pub fn new(ctx: &mut Context, board: Board) -> Self {
        Self {
            minefield: Minefield::new(ctx, board),
            minefield_origin: Vec2::new(0., 40.),
            background_color: Color::hex("#92969c"),
        }
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, self.background_color);
        graphics::draw(ctx, &self.minefield, self.minefield_origin);
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) ->  tetra::Result {

        if input::is_mouse_button_pressed(ctx, MouseButton::Left) {
            let pos = input::get_mouse_position(ctx);

            if self.minefield.rectangle(self.minefield_origin).contains_point(pos) {
                self.minefield.click(pos - self.minefield_origin, MouseButton::Left);
            }
        }

        if input::is_mouse_button_pressed(ctx, MouseButton::Right) {
            let pos = input::get_mouse_position(ctx);

            if self.minefield.rectangle(self.minefield_origin).contains_point(pos) {
                self.minefield.click(pos - self.minefield_origin, MouseButton::Right);
            }
        }

        Ok(())
    }
}
