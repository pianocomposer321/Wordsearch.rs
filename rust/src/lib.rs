use rand::{rngs::ThreadRng, seq::IteratorRandom, Rng};
use std::io;
use thiserror::Error;

pub mod pdf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Horizontal,
    Vertical,
    Diagonal,
}

type Board = Vec<Vec<char>>;

pub fn print_board(board: &Board) {
    println!(" {}", "_".repeat(board[0].len() * 2));
    for row in board.iter() {
        println!(
            "|{}|",
            row.iter()
                .map(|c| {
                    let mut s = c.to_string();
                    s.push(' ');
                    s
                })
                .collect::<String>()
        );
    }
    println!("|{}|", "_".repeat(board[0].len() * 2));
}

#[derive(Error, Debug)]
pub enum GenerationError {
    #[error("No possible board with this configuration. Increase board size.")]
    NoPossibleBoard,
    #[error("Reached maximum iterations. Increase iteration limit or board size.")]
    MaxIterationsReached,
}

#[derive(Error, Debug)]
pub enum MainError {
    #[error(transparent)]
    GenerateionError(#[from] GenerationError),
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error(transparent)]
    PdfError(#[from] pdf::PdfError),
}

pub fn generate_board(
    rows: usize,
    cols: usize,
    words: &Vec<&str>,
    fill_random: bool,
    max_iterations: Option<usize>,
) -> Result<Board, GenerationError> {
    let mut iterations: usize = 0;
    let mut rng = rand::thread_rng();

    let mut directions_count: Vec<u8> = vec![0, 0, 0];

    fn generate_board_impl(
        rows_count: usize,
        cols_count: usize,
        words: &Vec<&str>,
        word_ind: usize,
        board: Board,
        iterations: &mut usize,
        max_iterations: Option<usize>,
        directions_count: &mut Vec<u8>,
        rng: &mut ThreadRng,
    ) -> Result<Board, GenerationError> {
        let word = words[word_ind];
        let mut directions = [
            Direction::Horizontal,
            Direction::Diagonal,
            Direction::Vertical,
        ];
        directions.sort_by_key(|direction| directions_count[*direction as usize]);
        for direction in directions.iter() {
            // This works because (*direction as usize) will be 0, 1, or 2. Adding one gives 1, 2,
            // or 3, or 0b01, 0b10, or 0b11. & 1 gives the last bit, and >> 1 gives the second to
            // last.
            let dir_col_offset = (*direction as usize + 1) & 1;
            let dir_row_offset = (*direction as usize + 1) >> 1;

            let first_invalid_row;
            if dir_row_offset == 1 {
                if word.len() > rows_count {
                    continue;
                }
                first_invalid_row = rows_count - word.len();
            } else {
                first_invalid_row = rows_count;
            }
            let first_invalid_col;
            if dir_col_offset == 1 {
                if word.len() > rows_count {
                    continue;
                }
                first_invalid_col = cols_count - word.len();
            } else {
                first_invalid_col = cols_count;
            }

            let rand_row_offset = rng.gen_range(0..first_invalid_row);
            for row_ind in 0..rows_count {
                let row_ind_for_first_letter = (row_ind + rand_row_offset) % first_invalid_row;

                let rand_col_offset = rng.gen_range(0..first_invalid_col);
                for col_ind in 0..cols_count {
                    *iterations += 1;
                    if let Some(max_iterations) = max_iterations {
                        if *iterations > max_iterations {
                            return Err(GenerationError::MaxIterationsReached);
                        }
                    }

                    let col_ind_for_first_letter = (col_ind + rand_col_offset) % first_invalid_col;

                    let mut board_copy = Vec::with_capacity(board.len());
                    for row in board.iter() {
                        board_copy.push(row.clone());
                    }

                    let mut row_ind_for_current_letter = row_ind_for_first_letter;
                    let mut col_ind_for_current_letter = col_ind_for_first_letter;
                    let mut succesful = true;
                    for letter in word.chars() {
                        if board[row_ind_for_current_letter][col_ind_for_current_letter] != ' '
                            && board[row_ind_for_current_letter][col_ind_for_current_letter]
                                != letter
                        {
                            succesful = false;
                            break;
                        }
                        board_copy[row_ind_for_current_letter][col_ind_for_current_letter] = letter;
                        row_ind_for_current_letter += dir_row_offset;
                        col_ind_for_current_letter += dir_col_offset;
                    }
                    if succesful {
                        if word_ind + 1 == words.len() {
                            return Ok(board);
                        }

                        directions_count[*direction as usize] += 1;

                        if let Ok(board) = generate_board_impl(
                            rows_count,
                            cols_count,
                            words,
                            word_ind + 1,
                            board_copy,
                            iterations,
                            max_iterations,
                            directions_count,
                            rng,
                        ) {
                            return Ok(board);
                        } else {
                            directions_count[*direction as usize] -= 1;
                        };
                    }
                }
            }
        }
        Err(GenerationError::NoPossibleBoard)
    }

    let mut generated = generate_board_impl(
        rows,
        cols,
        words,
        0,
        vec![vec![' '; cols]; rows],
        &mut iterations,
        max_iterations,
        &mut directions_count,
        &mut rng,
    )?;
    if fill_random {
        let alphabet = "abcdefghijklmnopqrstuvwxyz";
        for row in generated.iter_mut() {
            for (ind, letter) in row.clone().iter().enumerate() {
                if *letter == ' ' {
                    row[ind] = alphabet.chars().choose(&mut rng).unwrap();
                }
            }
        }
    }
    Ok(generated)
}
