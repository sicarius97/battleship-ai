use crate::board::Board;
use crate::ship::Ship;
use rand::Rng;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Ai {
    pub move_num: u16,
    pub samples: u16,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Vertical,
    Horizontal,
}

impl Ai {
    pub fn new(samples: u16) -> Self {
        Self { samples, move_num: 0 }
    }

    pub fn simulate(&self, board: &mut Board) {
        let mut all_boards: Vec<Vec<i16>> = Vec::new();
        let mut board_vec = Board::generate_board(10, 10); 


        for x in 0..self.samples {
            let mut base_board = board.clone();
            println!("Board number: {}", x);

            self.place_all_random(&mut base_board);

            base_board.get_state();

            all_boards.push(base_board.board_state.to_vec());
        }

        for board in &all_boards {
            for x in 0..100 {
                board_vec[x] += board[x];
            }
        }

        let mut max_val = 0;
        let mut max_index = 0;

        for x in 0..100 {
            if &board_vec[x] > &max_val {
                max_val = board_vec[x];
                max_index = x
            }
        }

        println!("{:?}", &board_vec[0..10]);
        println!("{:?}", &board_vec[10..20]);
        println!("{:?}", &board_vec[20..30]);
        println!("{:?}", &board_vec[30..40]);
        println!("{:?}", &board_vec[40..50]);
        println!("{:?}", &board_vec[50..60]);
        println!("{:?}", &board_vec[60..70]);
        println!("{:?}", &board_vec[70..80]);
        println!("{:?}", &board_vec[80..90]);
        println!("{:?}", &board_vec[90..100]);
        println!("Length of array of boards: {}", all_boards.len());
        println!("Optimal move at index: {}", max_index)
    }

    pub fn place_all_random(&self, board: &mut Board) {
        let carrier = Ship::Carrier;
        let battleship = Ship::Battleship;
        let destroyer = Ship::Destroyer;
        let submarine = Ship::Submarine;
        let cruiser = Ship::Cruiser;

        self.place_ship(carrier, board);
        self.place_ship(battleship, board);
        self.place_ship(destroyer, board);
        self.place_ship(submarine, board);
        self.place_ship(cruiser, board);
    }

    fn place_ship(&self, ship: Ship, board: &mut Board) {
        let mut random_index = rand::thread_rng().gen_range(0..100);
        let mut is_occupied = true;
        let mut direction = Direction::Horizontal;
        let is_vert = rand::thread_rng().gen_range(0..10);

        if is_vert % 2 == 1 {
            direction = Direction::Vertical;
        }


        while is_occupied {
            let check = self.check_slice(random_index, ship.size().try_into().unwrap(), board, direction);

            if check {
                random_index = rand::thread_rng().gen_range(0..100);
            } else {
                is_occupied = false;
            }
        }

        let ship_size: usize = ship.size().try_into().unwrap();

        if direction == Direction::Horizontal {
            for x in random_index..random_index + ship_size {
                board.set_index(x, 1)
            }
        } else {
            let mut vertical_indices = vec![random_index];

            let i = 10;

            for x in 1..ship_size {
                vertical_indices.push(random_index + i * x);
            }

            for x in vertical_indices {
                board.set_index(x, 1)
            }
        }

    }

    fn check_slice(&self, index: usize, size: usize, board: &mut Board, direction: Direction) -> bool {
        let mut check = false;

        if direction == Direction::Horizontal {
            for x in index..index + size {
                if index % 10 > (index + size) % 10 {
                    check = true;
                } else if board.board_state[x] == 1 {
                    check = true;
                } else if board.board_state[x] == -1 {
                    check = true;
                }
            }
        } else {
            let mut vertical_indices = vec![index];

            let i = 10;

            for x in 1..size {
                vertical_indices.push(index + i * x);
            }

            for x in vertical_indices {
                if x > 99 {
                    check = true;
                } else if board.board_state[x] == 1 {
                    check = true;
                } else if board.board_state[x] == -1 {
                    check = true;
                }
            }
        }

        check
    }

}