#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tile {
    pub has_mine: bool,
    pub display: i8,
    pub neighbor_mines: i8,
    pub x: usize,
    pub y: usize,
}

impl Tile {
    pub fn new(has_mine: bool, x: usize, y: usize) -> Self {
        Self {
            has_mine,
            display: -1,
            neighbor_mines: 0,
            x,
            y,
        }
    }
}

use std::fmt::{self, Display, Formatter};

impl Display for Tile {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self.neighbor_mines {
            -2 => write!(formatter, "*"),
            0 => write!(formatter, " "),
            neighbor_mines => write!(formatter, "{}", neighbor_mines),
        }
    }
}
