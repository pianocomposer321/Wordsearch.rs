use clap::Parser;
use itertools::Itertools;
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
    #[arg(short='i', long, default_value = "1000000")]
    max_iterations: usize,

    /// Font size to use for the wordsearch grid
    #[arg(short, long, default_value = "16")]
    grid_font_size: f32,

    /// Font size to use for the word bank
    #[arg(short, long, default_value = "12")]
    word_bank_font_size: f32,

    /// Dimensions of generated PDF document
    #[arg(short, long, default_value = "letter")]
    size: String,

    /// Margin of generated PDF document (in points)
    #[arg(short, long, default_value = "36")]
    margin: f32,

    /// Title to put at the top of the PDF Document
    #[arg(short, long, default_value = "Wordsearch")]
    title: String,

    /// Font size for title
    #[arg(short='f', long, default_value = "24")]
    title_font_size: f32,
}

fn main() -> Result<(), MainError> {
    let args = Args::parse();

    let mut file = File::open(&args.filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let words = contents.lines().collect();

    let board = generate_board(args.rows, args.cols, &words, true, Some(args.max_iterations))?;

    if args.json {
        return Ok(());
    }

    if args.print {
        print_board(&board);
        return Ok(());
    }

    let (page_width, page_height): (f32, f32) = match args.size.to_lowercase().as_str() {
        "letter" => (612.0, 792.0),
        "a4" => (595.0, 842.0),
        other => {
            let dimensions = other.split_once(',').ok_or(MainError::ArgParseError)?;

            (dimensions.0.parse().map_err(|_| MainError::ArgParseError)?, dimensions.1.parse().map_err(|_| MainError::ArgParseError)?)
        }
    };

    let pdf_opts = PdfOptions {
        grid_font_size: args.grid_font_size,
        word_bank_font_size: args.word_bank_font_size,
        page_width,
        page_height,
        margin: args.margin,
        title: args.title,
        title_font_size: args.title_font_size,
    };

    pdf::generate_pdf(&args.output, &words, &board, &pdf_opts)?;

    Ok(())
}
