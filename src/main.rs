mod minefield;
use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.1.0", author = "Benjamin Falk <benjamin.falk@yahoo.com>")]
struct Opts {
    #[clap(short, long, default_value = "25")]
    width: i32,

    #[clap(short, long, default_value = "25")]
    height: i32,

    #[clap(short, long, default_value = "15")]
    count: i32,
}

fn main() {
    let opts: Opts = Opts::parse();

    match minefield::board::Board::new(opts.width, opts.height, opts.count) {
        Ok(board) =>
            println!("{}", board),

        Err(reason) => {
            eprintln!("{}", reason);
            std::process::exit(1)
        }
    };
}
