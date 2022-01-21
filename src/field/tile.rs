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
