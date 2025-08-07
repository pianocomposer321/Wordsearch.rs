use pdf_canvas::{BuiltinFont, Canvas, FontSource, Pdf};

use std::io;
use thiserror::Error;

use crate::{Board, GenerationError};

const FONT: BuiltinFont = BuiltinFont::Helvetica;
const FONT_HEIGHT_MULTIPLIER: f32 = 0.9251;

#[derive(Error, Debug)]
pub enum PdfError {
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error(transparent)]
    GenerationError(#[from] GenerationError),
}

type PdfResult = Result<(), PdfError>;

pub struct PdfOptions {
    pub grid_font_size: f32,
    pub word_bank_font_size: f32,
    pub page_width: f32,
    pub page_height: f32,
    pub margin: f32,
    pub title: String,
    pub title_font_size: f32,
}

struct WordBankDimensions {
    box_width: f32,
    box_height: f32,
    num_cols: f32,
    words_per_col: f32,
}

fn calculate_word_bank_dimensions(words: &Vec<String>, opts: &PdfOptions) -> WordBankDimensions {
    let word_widths = words
        .iter()
        .map(|word| FONT.get_width(opts.word_bank_font_size, word) as i32);

    let word_bank_col_margin = opts.word_bank_font_size * 1.5;
    let word_bank_padding = word_bank_col_margin;
    let word_bank_font_height = opts.word_bank_font_size * FONT_HEIGHT_MULTIPLIER;
    let word_bank_row_margin = opts.word_bank_font_size * 0.3;

    let min_col_width = word_widths.max().unwrap() as f32 + word_bank_col_margin;
    let box_width = opts.page_width - opts.margin * 2.0;
    let num_cols = ((box_width - word_bank_padding * 2.0) / min_col_width).floor();
    let words_per_col = (words.len() as f32 / num_cols).ceil();
    let box_height =
        word_bank_padding * 2.0 + (word_bank_font_height + word_bank_row_margin) * words_per_col;

    WordBankDimensions {
        box_width,
        box_height,
        num_cols,
        words_per_col,
    }
}

fn create_word_bank(c: &mut Canvas, words: &Vec<String>, opts: &PdfOptions) -> io::Result<()> {
    let bank_dimensions = calculate_word_bank_dimensions(words, opts);

    let word_bank_col_margin = opts.word_bank_font_size * 1.5;
    let word_bank_padding = word_bank_col_margin;
    let word_bank_font_height = opts.word_bank_font_size * FONT_HEIGHT_MULTIPLIER;
    let word_bank_row_margin = opts.word_bank_font_size * 0.3;

    let actual_col_width =
        (bank_dimensions.box_width - word_bank_padding) / bank_dimensions.num_cols;
    c.rectangle(
        opts.margin,
        opts.margin,
        bank_dimensions.box_width,
        bank_dimensions.box_height,
    )?;
    c.stroke()?;

    let mut word_ind = 0;
    let mut quit = false;
    for column in 0..(bank_dimensions.num_cols as usize) {
        for row in 0..(bank_dimensions.words_per_col as usize) {
            if word_ind == words.len() {
                quit = true;
                break;
            }
            c.left_text(
                // opts.margin + word_bank_padding + actual_col_width * column as f32,
                opts.margin + word_bank_padding + bank_dimensions.box_width * 0.5 - actual_col_width * bank_dimensions.num_cols * 0.5 + actual_col_width * column as f32,
                opts.margin + bank_dimensions.box_height
                    - word_bank_padding
                    - word_bank_font_height
                    - word_bank_row_margin
                    - (word_bank_row_margin + word_bank_font_height) * row as f32,
                FONT,
                opts.word_bank_font_size,
                &words[word_ind],
            )?;
            word_ind += 1;
        }
        if quit {
            break;
        }
    }

    Ok(())
}

fn create_grid(
    c: &mut Canvas,
    board: &Vec<Vec<char>>,
    word_bank_dimensions: WordBankDimensions,
    opts: &PdfOptions,
) -> io::Result<()> {
    let grid_font_height = opts.grid_font_size * FONT_HEIGHT_MULTIPLIER;
    let cell_padding = opts.grid_font_size * 0.3;
    let grid_padding = opts.grid_font_size * 0.25;

    let cell_size = grid_font_height + cell_padding;
    let grid_width = grid_padding * 2.0 + cell_size * board[0].len() as f32;
    let grid_height = grid_padding * 2.0 + cell_size * board.len() as f32;

    let title_height = opts.title_font_size * FONT_HEIGHT_MULTIPLIER;
    let box_x = (opts.page_width / 2.0) - (grid_width / 2.0);
    let box_y = opts.margin
        + word_bank_dimensions.box_height
        + (opts.page_height - opts.margin - word_bank_dimensions.box_height) / 2.0
        - (grid_height + opts.margin + title_height) / 2.0;
    c.rectangle(box_x, box_y, grid_width, grid_height)?;
    c.stroke()?;

    for row in 0..board.len() {
        for col in 0..board[0].len() {
            c.center_text(
                box_x + grid_padding + cell_size / 2.0 + cell_size * col as f32,
                box_y + grid_padding + 1.0 + cell_padding / 2.0 + cell_size * row as f32,
                FONT,
                opts.grid_font_size,
                board[row][col].to_uppercase().to_string().as_str(),
            )?;
        }
    }

    c.center_text(opts.page_width / 2.0, box_y + grid_height + opts.margin - title_height / 2.0, FONT, opts.title_font_size, &opts.title)?;

    Ok(())
}

pub fn generate_pdf(filename: &str, words: &Vec<&str>, board: &Vec<Vec<char>>, opts: &PdfOptions) -> PdfResult {
    let words_upper: Vec<_> = words.iter().map(|word| word.to_uppercase()).collect();

    let mut document = Pdf::create(filename)?;
    document.render_page(opts.page_width, opts.page_height, |canvas| {
        create_word_bank(canvas, &words_upper, opts)?;
        create_grid(canvas, board, calculate_word_bank_dimensions(&words_upper, opts), opts)?;

        Ok(())
    })?;

    document.finish()?;

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::generate_board;

    use super::*;

    #[test]
    fn create_pdf() -> PdfResult {
        let mut document = Pdf::create("output.pdf")?;
        let font = BuiltinFont::Helvetica;

        document.render_page(612.0, 792.0, |canvas| {
            canvas.center_text(100f32, 100f32, font, 16f32, "Hello, world!")?;

            Ok(())
        })?;

        document.finish()?;

        Ok(())
    }

    #[test]
    fn generate_pdf() -> PdfResult {
        let words = vec![
            "cylinder", "denial", "boot", "fossil", "compact", "nuance", "hover", "ancestor",
            "asset", "disagree", "elapse", "have", "linen", "even", "section", "fantasy", "young",
            "gear", "open", "consumer",
        ];

        let opts = PdfOptions {
            grid_font_size: 20.0,
            word_bank_font_size: 16.0,
            page_width: 612.0,
            page_height: 792.0,
            margin: 40.0,
            title: "Wordsearch".to_string(),
            title_font_size: 20.0
        };

        super::generate_pdf(
            "output.pdf",
            &words,
            &generate_board(10, 10, &words, true, Some(1_000_000))?,
            &opts,
        )?;

        Ok(())
    }
}
