use clap::Parser;
use serde_json;
use std::{fs::File, io::Read};
use wordsearch::*;

/// Wordsearch generator CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Filename for word list
    filename: String,

    /// Output in json instead of printing the board
    #[arg(long)]
    json: bool,
}

fn main() -> Result<(), MainError> {
    let args = Args::parse();

    let mut file = File::open(args.filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let words = contents.lines().collect();

    if args.json {
        let board = generate_board(ROWS, COLS, &words, true, Some(DEFAULT_MAX_ITERATIONS))?;

        println!("{}", serde_json::to_string(&board).unwrap());
        return Ok(());
    }

    print_board(&generate_board(
        ROWS,
        COLS,
        &words,
        true,
        Some(DEFAULT_MAX_ITERATIONS),
    )?);

    Ok(())
}
