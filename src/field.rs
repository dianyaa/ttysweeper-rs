use rand::rngs::ThreadRng;
use rand::Rng;

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
                row.push(Tile::new_non_mine(col_num, row_num, 0));
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
        let mut rng: ThreadRng = rand::thread_rng();
        let mut mine_count: usize = self.num_mines;
        while mine_count > 0 {
            let (x, y) = (rng.gen_range(0..self.width), rng.gen_range(0..self.height));
            if self.tile_at(x, y).has_mine() {
                continue;
            }
            self.mut_tile_at(x, y).place_mine_at();
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
                let is_target: bool = tle.x == target_tile.x && tle.y == target_tile.y;
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

    fn count_neighbor_mines(&self, tile_x: usize, tile_y: usize) -> u8 {
        let mut count = 0;
        for neighbor_x in (tile_x - 1)..(tile_y + 1) {
            for neighbor_y in (tile_y - 1)..=(tile_y + 1) {
                // don't count self
                if (neighbor_x, neighbor_y) == (tile_x, tile_y) {
                    continue;
                }
                if self
                    .try_tile_at(neighbor_x, neighbor_y)
                    .map(|neighbor| neighbor.has_mine())
                    .unwrap_or(false)
                {
                    count += 1;
                }
            }
        }
        count
    }
    fn update_tile_counts(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                // FIXME: the borrow checker doesn't like us calling count_neighbor_mines inside the if statement, so we inefficiently calculate it for all tiles.
                let count = self.count_neighbor_mines(x, y);
                if let Some(neighbor_mines) = self.minefield[y][x].try_mut_neighbor_mines() {
                    *neighbor_mines = count;
                }
            }
        }
    }

    fn try_tile_at(&self, x: usize, y: usize) -> Option<&Tile> {
        self.minefield.get(y).and_then(|row| row.get(x))
    }
    fn try_mut_tile_at(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        self.minefield.get_mut(y).and_then(|row| row.get_mut(x))
    }

    fn mut_tile_at(&mut self, x: usize, y: usize) -> &mut Tile {
        self.try_mut_tile_at(x, y).expect("Tile out of range")
    }
    fn tile_at(&self, x: usize, y: usize) -> &Tile {
        self.try_tile_at(x, y).expect("Tile out of range")
    }
}
