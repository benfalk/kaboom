mod minefield;
mod gui;

use clap::Clap;
use minefield::board::Board;

#[derive(Clap)]
#[clap(version = "0.1.0", author = "Benjamin Falk <benjamin.falk@yahoo.com>")]
struct Opts {
    #[clap(short, long, default_value = "25")]
    width: i32,

    #[clap(short, long, default_value = "25")]
    height: i32,

    #[clap(short, long, default_value = "99")]
    count: i32,
}

fn main() {
    let opts: Opts = Opts::parse();
    let board = Board::new(opts.width, opts.height, opts.count).unwrap();
    gui::run(board).unwrap();
}
