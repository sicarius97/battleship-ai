
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Board {
    pub length: u16,
    pub width: u16,
    pub board_state: Vec<i16>,
}

impl Board {
    pub fn new(length: u16, width: u16, board_state: Vec<i16>) -> Self {
        Self { length, width, board_state }
    }

    pub fn generate_board(length: u16, width: u16) -> Vec<i16> {
        let board_size: usize = usize::from(length * width);
        let vec = vec![0; board_size];

        vec
    }
    /* only used for debug purposes
    pub fn get_state(&self) {
        println!("{:?}", &self.board_state[0..10]);
        println!("{:?}", &self.board_state[10..20]);
        println!("{:?}", &self.board_state[20..30]);
        println!("{:?}", &self.board_state[30..40]);
        println!("{:?}", &self.board_state[40..50]);
        println!("{:?}", &self.board_state[50..60]);
        println!("{:?}", &self.board_state[60..70]);
        println!("{:?}", &self.board_state[70..80]);
        println!("{:?}", &self.board_state[80..90]);
        println!("{:?}", &self.board_state[90..100])
    }
    */

    pub fn set_index(&mut self, index: usize, val: i16) {
        self.board_state[index] = val;
    }
}