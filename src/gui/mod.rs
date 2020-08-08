mod state;
mod assets;
mod minefield;
mod component;
mod reset_button;
mod bomb_count_panel;

use tetra::ContextBuilder;
use super::minefield::board::Board;
use state::GameState;

pub fn run(board: Board) -> tetra::Result {
    ContextBuilder::new(
            "Kaboom!",
            board.width() * 32,
            board.height() * 32 + 40
        )
        .quit_on_escape(true)
        .show_mouse(true)
        .build()?
        .run(move |ctx| Ok(GameState::new(ctx, board)))
}
