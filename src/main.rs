mod board;
mod ai;
mod ship;

use crate::board::Board;
use crate::ai::Ai;



fn main() {
    let board_vec = Board::generate_board(10, 10); 

    let mut new_game = Board::new(10, 10, board_vec);

    let mut running: bool = true;

    let mut move_num = 0;
    let mut ai = Ai::new(10000);

    fn override_index(board: &mut Board) {
        let mut input = String::new();
        println!("What is the overridden index number?");

        std::io::stdin().read_line(&mut input).expect("Non valid input entered!");

        let int: usize = input.trim().parse().unwrap();

        board.set_index(int, -1)
    }

    // main loop 
    while running {
        let mut selection = String::new();

        let index = ai.simulate(&mut new_game);

        println!("Move number: {}", move_num);
        println!("What is the result?");
        println!("M - Miss H - Hit S - Sunk O - Override Q - Quit: ");
        std::io::stdin().read_line(&mut selection).expect("error: unable to read user input");
        
        match selection.trim() {
            "M" => new_game.set_index(index, -1),
            "H" => new_game.set_index(index, -1),
            "S" => ai.sunk_ship(),
            "O" => override_index(&mut new_game),
            "Q" => running = false,
            _ => panic!("Invalid user input selected!: {}", selection),
        }

        move_num += 1;
    }
}
