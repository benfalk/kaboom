use std::cell::RefCell;
use std::rc::Rc;

use super::minefield::Minefield;
use super::reset_button::ResetButton;
use super::super::minefield::board::Board;
use super::component::Component;

use tetra::graphics::{self, Color};
use tetra::input::{self, MouseButton};
use tetra::{Context, State};
use tetra::math::Vec2;

pub struct GameState {
    minefield: Minefield,
    reset_button: ResetButton,
    reset_button_origin: Vec2<f32>,
    minefield_origin: Vec2<f32>,
    background_color: Color,
}

impl GameState {
    pub fn new(ctx: &mut Context, board: Board) -> Self {
        let board = Rc::new(RefCell::new(board));

        Self {
            reset_button: ResetButton::new(ctx, Rc::clone(&board)),
            reset_button_origin: Vec2::new(0., 0.),
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
        graphics::draw(ctx, &self.reset_button, self.reset_button_origin);
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) ->  tetra::Result {

        if input::is_mouse_button_pressed(ctx, MouseButton::Left) {
            let pos = input::get_mouse_position(ctx);

            if self.minefield.contains_point(self.minefield_origin, pos) {
                self.minefield.click(pos - self.minefield_origin, MouseButton::Left);
            } else if self.reset_button.contains_point(self.reset_button_origin, pos) {
                self.reset_button.click(self.reset_button_origin, MouseButton::Left);
            }
        }

        if input::is_mouse_button_pressed(ctx, MouseButton::Right) {
            let pos = input::get_mouse_position(ctx);

            if self.minefield.contains_point(self.minefield_origin, pos) {
                self.minefield.click(pos - self.minefield_origin, MouseButton::Right);
            }
        }

        Ok(())
    }
}
