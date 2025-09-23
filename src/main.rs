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

fn get_player_move(current_player: char, board: &Board) -> (usize, usize) {
    
}

fn play_game() {
    let mut board = initialize_board();
    let mut current_player = PLAYER_X;

    loop {
        println!("Current board:");
        print_board(&board);

        let (row, col) = get_player_move(current_player, &board);
        board[row][col] = current_player;

        current_player = if current_player == PLAYER_X {
            PLAYER_O
        } else {
            PLAYER_X
        };
    }
}

fn main() {
    println!("Welcome to Tic-Tac-Toe Game!");
}
