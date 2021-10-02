extern crate structopt;

use structopt::StructOpt;

use std::path::PathBuf;
use std::process::exit;
use std::str;
use std::thread::sleep;
use std::time::Duration;

mod grid;

fn clear_screen() {
    let escape: u8 = 27;
    print!("{}[2J", escape as char);
    print!("{}[2J", escape as char);
}

fn game_loop(file: &str, update_interval: u64) {
    let mut grid = grid::create_grid(file);
    let delay = Duration::from_millis(update_interval);
    let mut generation = 0;
    clear_screen();
    grid.render();
    while grid.next_state() {
        generation += 1;
        sleep(delay);
        grid.render();
    }
    println!("Finished on generation {}.", generation);
}

#[derive(Debug, StructOpt)]
#[structopt(name = "Game of life")]
struct Cli {
    #[structopt(parse(from_os_str))]
    /// File containing a starting pattern for the game.
    /// Example patterns are in the "patterns" folder.
    pattern_file: PathBuf,

    #[structopt(default_value = "500", short)]
    /// The rendering update interval in milliseconds.
    update_interval: u64,
}

fn main() {
    let args = Cli::from_args();
    let file = args.pattern_file;
    let update_interval = args.update_interval;

    if file.exists() {
        game_loop(file.to_str().unwrap(), update_interval);
    } else {
        println!("File \"{}\" does not exist", file.to_str().unwrap());
        exit(1);
    }
}
