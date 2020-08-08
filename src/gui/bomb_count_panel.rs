use std::cell::RefCell;
use std::rc::Rc;
use std::fmt::Write;

use super::assets::Assets;
use super::component::Component;
use crate::minefield::board::Board;

use tetra::Context;

use tetra::graphics::{self, DrawParams, Drawable, Rectangle, Texture, Color};
use tetra::graphics::text::Text;

pub struct BombCountPanel {
    width: f32,
    height: f32,
    board: Rc<RefCell<Board>>,
    background: Texture,
    count: RefCell<Text>,
    buffer: RefCell<String>,
}

impl BombCountPanel {
    pub fn new(ctx: &mut Context, board: Rc<RefCell<Board>>) -> Self {
        Self {
            width: 80.,
            height: 40.,
            board,
            background: Assets::get_texture(ctx, "40-by-80.png"),
            count: RefCell::new(Text::new(
                "",
                Assets::get_font(ctx, "DejaVuSansMono.ttf", 20.)
            )),
            buffer: RefCell::new(String::with_capacity(20)),
        }
    }
}

impl Component for BombCountPanel {
    fn rectangle<P: Into<DrawParams>>(&self, params: P) -> Rectangle {
        let params: DrawParams = params.into();

        Rectangle::new(
            params.position.x,
            params.position.y,
            self.width,
            self.height,
        )
    }
}

impl Drawable for BombCountPanel {

    fn draw<P: Into<DrawParams>>(&self, ctx: &mut Context, params: P) {
        let params: DrawParams = params.into();
        graphics::draw(ctx, &self.background, params.position);

        let text_parms = DrawParams::new()
            .position(params.position + (20., 10.))
            .color(Color::BLACK);

        // The cost of not allocating a string?
        let remaining = self.board.borrow().bombs_remaining();
        let mut buffer = self.buffer.borrow_mut();
        buffer.truncate(0);
        write!(&mut *buffer, "{}", remaining).unwrap();

        self.count.borrow_mut().set_content(&(*buffer));

        graphics::draw(ctx, &(*self.count.borrow()), text_parms);
    }
}
