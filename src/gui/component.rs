use tetra::math::Vec2;
use tetra::input::MouseButton;
use tetra::graphics::{Drawable, DrawParams, Rectangle};

pub trait Component : Drawable {
    fn rectangle<P: Into<DrawParams>>(&self, params: P) -> Rectangle;
    fn click(&mut self, _pos: Vec2<f32>, _button: MouseButton) { }
}
