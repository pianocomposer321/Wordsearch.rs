use clap::Parser;
use serde_json;
use std::{fs::File, io::Read};
use wordsearch::{pdf::PdfOptions, *};

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

    /// Number of rows to include in the wordsearch grid
    #[arg(short, long, default_value = "15")]
    rows: usize,

    /// Number of columns to include in the wordsearch grid
    #[arg(short, long, default_value = "15")]
    cols: usize,

    /// Maximum iterations to try before aborting
    #[arg(long, default_value = "1000000")]
    max_iterations: usize,

    /// Font size to use for the wordsearch grid
    #[arg(long, default_value = "16")]
    grid_font_size: f32,

    /// Font size to use for the word bank
    #[arg(long, default_value = "12")]
    word_bank_font_size: f32,

    /// Width of generated PDF document (in points)
    #[arg(long, default_value = "612")]
    width: f32,

    /// Height of generated PDF document (in points)
    #[arg(long, default_value = "792")]
    height: f32,

    /// Margin of generated PDF document (in points)
    #[arg(short, long, default_value = "40")]
    margin: f32,
}

fn main() -> Result<(), MainError> {
    let args = Args::parse();

    let mut file = File::open(&args.filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let words = contents.lines().collect();

    let board = generate_board(args.rows, args.cols, &words, true, Some(args.max_iterations))?;

    if args.json {
        println!("{}", serde_json::to_string(&board).unwrap());
        return Ok(());
    }

    if args.print {
        print_board(&board);
        return Ok(());
    }

    let pdf_opts = PdfOptions {
        grid_font_size: args.grid_font_size,
        word_bank_font_size: args.word_bank_font_size,
        page_width: args.width,
        page_height: args.height,
        margin: args.margin,
    };

    pdf::generate_pdf(&args.output, &words, &board, &pdf_opts)?;

    Ok(())
}
