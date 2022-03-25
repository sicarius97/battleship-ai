#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Ship {
    Carrier,
    Battleship,
    Submarine,
    Cruiser,
    Destroyer,
}

impl Ship {
    pub fn size(&self) -> u32 {
        use Ship::*;
        match *self {
            Carrier => 5,
            Battleship => 4,
            Submarine => 3,
            Cruiser => 3,
            Destroyer => 2,
        }
    }
}