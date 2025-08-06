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

    /// Filename of resulting pdf
    #[arg(short, long, default_value = "output.pdf")]
    output: String,

    /// Output in json instead of generating a PDF
    #[arg(short, long)]
    json: bool,

    /// Print the board to the console instead of generating a PDF
    #[arg(short, long)]
    print: bool,
}

fn main() -> Result<(), MainError> {
    let args = Args::parse();

    let mut file = File::open(&args.filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let words = contents.lines().collect();

    let board = generate_board(ROWS, COLS, &words, true, Some(DEFAULT_MAX_ITERATIONS))?;

    if args.json {
        println!("{}", serde_json::to_string(&board).unwrap());
        return Ok(());
    }

    if args.print {
        print_board(&board);
        return Ok(());
    }

    pdf::generate_pdf(&args.output, &words, &board)?;

    Ok(())
}
