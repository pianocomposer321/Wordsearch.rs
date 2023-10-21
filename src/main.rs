use std::fs;
use rand::{Rng, SeedableRng, rngs::StdRng};
use thiserror::Error;

// Limit for how many times the generator will attempt to create a board.
const MAX_ITERATIONS: usize = 1_000_000;

// Alphabet characters for random placement on the board.
const ALPHABET: &[u8; 26] = b"abcdefghijklmnopqrstuvwxyz";

// Character used for overlining in board display.
const OVERLINE: char = '\u{203E}';

// Possible directions for word placement on the word search puzzle board.
#[derive(Clone, Copy)]
enum Direction {
    Horizontal,  
    Vertical,    
    Diagonal,    
}

// Represents the word search puzzle board as a 2D grid of characters.
type Board = Vec<Vec<char>>;

// Errors that might occur during word search puzzle generation.
#[derive(Error, Debug)]
enum GenerationError {
    #[error("No possible board with this configuration. Increase board size.")]
    NoPossibleBoard,
    #[error("Reached maximum iterations. Increase iteration limit or board size.")]
    MaxIterationsReached, 
}

fn print_board(board: &Board) {
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
    println!(" {}", OVERLINE.to_string().repeat(board[0].len() * 2));    
}

fn place_letter_board (
    board: &mut Board,
    letter: char,
    x: usize,
    y: usize, 
) -> bool {
    // Check if the coordinates are within the bounds of the board
    if x >= board.len() || y >= board[x].len() {
        return false;
    }

    // Check if there is already a letter 
    if board[x][y] != ' ' {
        return false;
    }

    board[x][y] = letter;
    true
}

fn place_word_board (
    word: &str,
    mut board: Board,
    direction: Direction,
    mut x: usize,
    mut y: usize,
) -> Option<Board> 
{
    let (x_offset, y_offset) = match direction {
        Direction::Horizontal => (1,0),
        Direction::Vertical => (0,1),
        Direction::Diagonal => (1,1),
    }; 
    for ch in word.chars() {
        if !place_letter_board(&mut board, ch, x, y){
            return None;
        }

        x += x_offset;
        y += y_offset;
    }
    Some(board)
}

fn fill_board (
    mut board: Board,
    rng_seed: [u8; 32],
) -> Board {
    let mut rng: StdRng = SeedableRng::from_seed(rng_seed);
    for row in board.iter_mut() {
        for position in row.iter_mut() {
            if *position == ' ' {
                let random_alphabet_index = rng.gen_range(0..ALPHABET.len());
                *position = ALPHABET[random_alphabet_index] as char;
            }
        }
    }
    board
}

fn place_words_board (
    mut board: Board,
    words: Vec<&str>,
    rng_seed: [u8; 32],
) -> Result<Board, GenerationError> {

    let rows = board.len();
    let cols = board.iter().map(|row| row.len()).max().unwrap_or(0);
    
    // Check if all words can fit within the board dimensions
    if !words.iter().all(|&word| word.len() <= rows && word.len() <= cols) {
        return Err(GenerationError::NoPossibleBoard);
    }

    let mut iterations: usize = 0;
    let mut rng: StdRng = SeedableRng::from_seed(rng_seed);
    let mut directions_counts = vec![
        (Direction::Horizontal , 0), 
        (Direction::Vertical , 0), 
        (Direction::Diagonal , 0)];
    
    for word in words {
        loop {
            if iterations >= MAX_ITERATIONS {
                return Err(GenerationError::MaxIterationsReached);
            }
            
            // Create a random x and y on the board
            let x = rng.gen_range(0..cols);
            let y = rng.gen_range(0..rows);
            
            // Order directions based on the least words in the puzzle with the direction
            directions_counts.sort_by(|a, b| a.1.cmp(&b.1));
            let mut put_word_success = false;

            // Try placing a word in all directions on a certain position
            for (direction, count) in directions_counts.iter_mut() {
                if let Some(result) = place_word_board(word, board.clone(), *direction, x, y) {
                    board = result;
                    // Increase the count of words on the board with this direction
                    *count += 1;
                    put_word_success = true;
                    break;
                }
            }
            iterations += 1;
            if put_word_success {
                break;
            }
        }
    }

    Ok(board)
}

fn main() -> Result<(), GenerationError> {
    let content = fs::read_to_string("words.txt")
        .expect("Failed to read the file");

    // Dimensions for the word search puzzle board.
    let (cols, rows) = (15, 15); 

    let words: Vec<&str> = content.lines().collect();
    let board = vec![vec![' '; cols]; rows];
    let seed = [0; 32];

    println!("Placing words on the board!");
    let word_board = place_words_board(board, words, seed)?;
    print_board(&word_board);

    println!("Filling the empty spaces on the board!");
    let complete_board = fill_board(word_board, seed);
    print_board(&complete_board);

    Ok(())
}
