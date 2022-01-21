use crate::util::Position;

mod tile;
use tile::Tile;

pub struct Field {
    height: usize,
    width: usize,
    num_mines: usize,
    minefield: Vec<Vec<Tile>>,
}

impl Field {
    pub fn new(height: usize, width: usize, num_mines: usize) -> Self {
        let mut minefield = Vec::with_capacity(height);
        for row_num in 0..height {
            let mut row = Vec::with_capacity(width);
            for col_num in 0..width {
                row.push(Tile::new_non_mine(
                    Position::from_row_col(row_num, col_num),
                    0,
                ));
            }
            minefield.push(row);
        }

        let mut ret = Self {
            height,
            width,
            num_mines,
            minefield,
        };
        ret.reset();
        ret
    }

    pub fn reset(&mut self) {
        let mut mine_count: usize = self.num_mines;
        while mine_count > 0 {
            let pos = Position::random(self.width, self.height);
            if self.tile_at(pos).has_mine() {
                continue;
            }
            self.mut_tile_at(pos).place_mine_at();
            mine_count -= 1;
        }

        self.update_tile_counts();
    }

    pub fn print_field(&self) {
        for row in &self.minefield {
            for tle in row {
                print!("[{}]", tle);
            }
            println!();
        }
    }

    pub fn highlight_print_field(&self, target_tile: &Tile) {
        for row in &self.minefield {
            for tle in row {
                let is_target: bool = tle.position == target_tile.position;
                if is_target {
                    print!("{{");
                } else {
                    print!("[");
                }

                print!("{}", tle);

                if is_target {
                    print!("}}");
                } else {
                    print!("]");
                }
            }
            println!();
        }
    }

    pub fn reveal_tile(&mut self, y: usize, x: usize) {
        if y >= self.height || x >= self.width {
            return;
        }
        todo!();
    }

    fn count_neighbor_mines(&self, tile_pos: Position) -> u8 {
        let mut count = 0;
        for pos in tile_pos.iter_around(1) {
            if self
                .try_tile_at(pos)
                .map(|neighbor| neighbor.has_mine())
                .unwrap_or(false)
            {
                count += 1;
            }
        }
        count
    }
    fn update_tile_counts(&mut self) {
        for pos in Position::iter_2d(self.width, self.height) {
            // FIXME: the borrow checker doesn't like us calling count_neighbor_mines inside the if statement, so we inefficiently calculate it for all tiles.
            let count = self.count_neighbor_mines(pos);
            if let Some(neighbor_mines) = self.mut_tile_at(pos).try_mut_neighbor_mines() {
                *neighbor_mines = count;
            }
        }
    }

    fn try_tile_at(&self, Position { x, y }: Position) -> Option<&Tile> {
        self.minefield.get(y).and_then(|row| row.get(x))
    }
    fn try_mut_tile_at(&mut self, Position { x, y }: Position) -> Option<&mut Tile> {
        self.minefield.get_mut(y).and_then(|row| row.get_mut(x))
    }

    fn mut_tile_at(&mut self, pos: Position) -> &mut Tile {
        self.try_mut_tile_at(pos).expect("Tile out of range")
    }
    fn tile_at(&self, pos: Position) -> &Tile {
        self.try_tile_at(pos).expect("Tile out of range")
    }
}
