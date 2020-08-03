mod state;

use tetra::ContextBuilder;
use super::minefield::board::Board;
use state::GameState;

pub fn run(board: Board) -> tetra::Result {
    ContextBuilder::new(
            "Kaboom!",
            board.width() * 32,
            board.height() * 32
        )
        .quit_on_escape(true)
        .show_mouse(true)
        .build()?
        .run(move |ctx| Ok(GameState::new(ctx, board)))
}
