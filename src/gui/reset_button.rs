use std::cell::RefCell;
use std::rc::Rc;

use super::assets::Assets;
use super::component::Component;
use crate::minefield::board::Board;

use tetra::Context;
use tetra::input::MouseButton;
use tetra::math::Vec2;

use tetra::graphics::{
    self,
    Texture,
    DrawParams,
    Drawable,
    Rectangle,
};

pub struct ResetButton {
    width: f32,
    height: f32,
    board: Rc<RefCell<Board>>,
    texture: Texture,
}

impl ResetButton {
    pub fn new(ctx: &mut Context, board: Rc<RefCell<Board>>) -> Self {
        Self {
            width: 80.,
            height: 40.,
            texture: Assets::get_texture(ctx, "restart.png"),
            board,
        }
    }
}

impl Drawable for ResetButton {
    fn draw<P: Into<DrawParams>>(&self, ctx: &mut Context, params: P) {
        graphics::draw(ctx, &self.texture, params);
    }
}

impl Component for ResetButton {
    fn rectangle<P: Into<DrawParams>>(&self, params: P) -> Rectangle {
        let params: DrawParams = params.into();

        Rectangle::new(
            params.position.x,
            params.position.y,
            self.width,
            self.height,
        )
    }

    fn click(&mut self, _pos: Vec2<f32>, button: MouseButton) {
        match button {
            MouseButton::Left => self.board.borrow_mut().reset(),
            _ => (),
        }
    }
}
