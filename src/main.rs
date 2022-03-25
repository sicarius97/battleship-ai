mod board;
mod ai;
mod ship;

use crate::board::Board;
use crate::ai::Ai;

fn main() {
    let board_vec = Board::generate_board(10, 10); 

    let mut new_game = Board::new(10, 10, board_vec);

    new_game.set_index(54, -1);
    new_game.set_index(43, -1);
    new_game.set_index(35, -1);

    let ai = Ai::new(10000);

    ai.simulate(&mut new_game)
}
