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
    loop {
        println!(
            "Player {}, enter your move (row and column): ",
            current_player
        );
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let coordinates: Vec<usize> = input
            .trim()
            .split_whitespace()
            .flat_map(str::parse::<usize>)
            .collect();
        if coordinates.len() == 2 {
            let (row, col) = (coordinates[0], coordinates[1]);
            if row < BOARD_SIZE && col < BOARD_SIZE && board[row][col] == ' ' {
                return (row, col);
            }
        }

        println!("Invalid move. Try again.");
    }
}

fn check_winner(board: &Board, player: char) -> bool {
    for row in 0..BOARD_SIZE {
        if board[row].iter().all(|&cell| cell == player) {
            return true;
        }
    }
    for col in 0..BOARD_SIZE {
        if (0..BOARD_SIZE).all(|row| board[row][col] == player) {
            return true;
        }
    }
    if (0..BOARD_SIZE).all(|i| board[i][i] == player) {
        return true;
    }
    if (0..BOARD_SIZE).all(|i| board[i][BOARD_SIZE - 1 - i] == player) {
        return true;
    }
    false
}



fn play_game() {
    let mut board = initialize_board();
    let mut current_player = PLAYER_X;

    loop {
        println!("Current board:");
        print_board(&board);

        let (row, col) = get_player_move(current_player, &board);
        board[row][col] = current_player;

        if check_winner(&board, current_player) {
            println!("Player {} wins!", current_player);
            print_board(&board);
            break;
        }

        current_player = if current_player == PLAYER_X {
            PLAYER_O
        } else {
            PLAYER_X
        };
    }
}

fn main() {
    println!("Welcome to Tic-Tac-Toe Game!");
    play_game();

}
