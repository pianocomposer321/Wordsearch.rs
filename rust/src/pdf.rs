use pdf_canvas::{BuiltinFont, Canvas, FontSource, Pdf};

use std::io;
use thiserror::Error;

use crate::{Board, GenerationError};

#[derive(Error, Debug)]
pub enum PdfError {
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error(transparent)]
    GenerationError(#[from] GenerationError),
}

type PdfResult = Result<(), PdfError>;

const FONT: BuiltinFont = BuiltinFont::Helvetica;
const GRID_FONT_SIZE: f32 = 20.0;
const WORD_BANK_FONT_SIZE: f32 = 16.0;
const GRID_FONT_HEIGHT: f32 = GRID_FONT_SIZE / 1.081;
const WORD_BANK_FONT_HEIGHT: f32 = WORD_BANK_FONT_SIZE / 1.081;
const PAGE_WIDTH: f32 = 612.0;
const PAGE_HEIGHT: f32 = 792.0;
const MARGIN: f32 = 40.0;
const CELL_PADDING: f32 = GRID_FONT_SIZE / 3.0;
const GRID_PADDING: f32 = GRID_FONT_SIZE / 4.0;
const WORD_BANK_MAX_HEIGHT: f32 = 500.0;
const WORD_BANK_COL_MARGIN: f32 = 25.0;
const WORD_BANK_PADDING: f32 = 25.0;
const WORD_BANK_ROW_MARGIN: f32 = 5.0;
const WORD_BANK_BOX_RADIUS: f32 = 5.0;

struct WordBankDimensions {
    box_width: f32,
    box_height: f32,
    num_cols: f32,
    words_per_col: f32,
}

fn calculate_word_bank_dimensions(words: &Vec<String>) -> WordBankDimensions {
    let word_widths = words
        .iter()
        .map(|word| FONT.get_width(WORD_BANK_FONT_SIZE, word) as i32);
    let min_col_width = word_widths.max().unwrap() as f32 + WORD_BANK_COL_MARGIN;
    let box_width = PAGE_WIDTH - MARGIN * 2.0;
    let num_cols = ((box_width - WORD_BANK_PADDING * 2.0) / min_col_width + 0.5).floor();
    let words_per_col = (words.len() as f32 / num_cols).ceil();
    let box_height =
        WORD_BANK_PADDING * 2.0 + (WORD_BANK_FONT_HEIGHT + WORD_BANK_ROW_MARGIN) * words_per_col;

    WordBankDimensions {
        box_width,
        box_height,
        num_cols,
        words_per_col,
    }
}

fn create_word_bank(c: &mut Canvas, words: &Vec<String>) -> io::Result<()> {
    let bank_dimensions = calculate_word_bank_dimensions(words);
    let actual_col_width =
        (bank_dimensions.box_width - WORD_BANK_PADDING * 2.0) / bank_dimensions.num_cols;
    c.rectangle(
        MARGIN,
        MARGIN,
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
                MARGIN + WORD_BANK_PADDING + actual_col_width * column as f32,
                MARGIN + bank_dimensions.box_height
                    - WORD_BANK_PADDING
                    - WORD_BANK_FONT_HEIGHT
                    - WORD_BANK_ROW_MARGIN
                    - (WORD_BANK_ROW_MARGIN + WORD_BANK_FONT_HEIGHT) * row as f32,
                FONT,
                WORD_BANK_FONT_SIZE,
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
) -> io::Result<()> {
    let available_width = PAGE_WIDTH - MARGIN * 2.0;
    let available_height =
        PAGE_HEIGHT - MARGIN * 2.0 - word_bank_dimensions.box_height - WORD_BANK_PADDING;

    let cell_size = GRID_FONT_HEIGHT + CELL_PADDING;
    let grid_width = GRID_PADDING * 2.0 + cell_size * board[0].len() as f32;
    let grid_height = GRID_PADDING * 2.0 + cell_size * board.len() as f32;

    let box_x = (PAGE_WIDTH / 2.0) - (grid_width / 2.0);
    let box_y = MARGIN
        + word_bank_dimensions.box_height
        + (PAGE_HEIGHT - MARGIN - word_bank_dimensions.box_height) / 2.0
        - grid_height / 2.0;
    c.rectangle(box_x, box_y, grid_width, grid_height)?;
    c.stroke()?;

    for row in 0..board.len() {
        for col in 0..board[0].len() {
            c.center_text(
                box_x + GRID_PADDING + cell_size / 2.0 + cell_size * col as f32,
                box_y + GRID_PADDING + 1.0 + CELL_PADDING / 2.0 + cell_size * row as f32,
                FONT,
                GRID_FONT_SIZE,
                board[row][col].to_uppercase().to_string().as_str(),
            )?;
        }
    }

    Ok(())
}

pub fn generate_pdf(filename: &str, words: &Vec<&str>, board: &Vec<Vec<char>>) -> PdfResult {
    let words_upper: Vec<_> = words.iter().map(|word| word.to_uppercase()).collect();

    let mut document = Pdf::create(filename)?;
    document.render_page(PAGE_WIDTH, PAGE_HEIGHT, |canvas| {
        create_word_bank(canvas, &words_upper)?;
        create_grid(canvas, board, calculate_word_bank_dimensions(&words_upper))?;

        Ok(())
    })?;

    document.finish()?;

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{generate_board, COLS, DEFAULT_MAX_ITERATIONS, ROWS};

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

        super::generate_pdf(
            "output.pdf",
            &words,
            &generate_board(ROWS, COLS, &words, true, Some(DEFAULT_MAX_ITERATIONS))?,
        )?;

        Ok(())
    }
}
