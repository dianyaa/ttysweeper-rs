#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tile {
    pub is_showing: bool,
    pub kind: TileKind,
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileKind {
    Mine,
    NotMine { neighbor_mines: u8 },
}

impl Tile {
    pub fn new_mine(x: usize, y: usize) -> Self {
        Self {
            is_showing: false,
            kind: TileKind::Mine,
            x,
            y,
        }
    }
    pub fn new_non_mine(x: usize, y: usize, neighbor_mines: u8) -> Self {
        Self {
            is_showing: false,
            kind: TileKind::NotMine { neighbor_mines },
            x,
            y,
        }
    }

    pub fn has_mine(&self) -> bool {
        match self.kind {
            TileKind::Mine => true,
            TileKind::NotMine { .. } => false,
        }
    }
    pub fn place_mine_at(&mut self) {
        self.kind = TileKind::Mine;
    }

    pub fn try_mut_neighbor_mines(&mut self) -> Option<&mut u8> {
        match self.kind {
            TileKind::Mine => None,
            TileKind::NotMine {
                ref mut neighbor_mines,
            } => Some(neighbor_mines),
        }
    }
}

use std::fmt::{self, Display, Formatter};

impl Display for TileKind {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::Mine => write!(formatter, "*"),
            Self::NotMine { neighbor_mines } => write!(formatter, "{}", neighbor_mines),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        if self.is_showing {
            self.kind.fmt(formatter)
        } else {
            write!(formatter, " ")
        }
    }
}
