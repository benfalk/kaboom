mod state;

use tetra::ContextBuilder;
use super::minefield::board::Board;

pub fn run(gamestate: Board) -> tetra::Result {
    ContextBuilder::new(
            "Kaboom!",
            gamestate.width() * 32,
            gamestate.height() * 32
        )
        .quit_on_escape(true)
        .show_mouse(true)
        .build()?
        .run(move |_| Ok(gamestate))
}
