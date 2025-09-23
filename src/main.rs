use std::io;

const PLAYER_X: char = 'X';
const PLAYER_O: char = 'O';

const BOARD_SIZE: usize = 3;

// type keyword to define a custom type alias
type Board = [[char; BOARD_SIZE]; BOARD_SIZE]; // 2D array to represent the board
// represents a 3x3 grid where each cell can hold a character

fn initialize_board() -> Board {
    [[' '; BOARD_SIZE]; BOARD_SIZE]
}

fn print_board(board: &Board) {
    for row in board {
        for &cell in row {
            print!("|{}", cell);
        }
        println!("|");
    }
}

fn main() {
    println!("Welcome to Tic-Tac-Toe Game!");
}