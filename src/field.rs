mod tile;

use std::io::Write;
use rand::Rng;
use rand::rngs::ThreadRng;

pub struct Field {
    height: usize,
    width: usize,
    num_mines: usize,
    minefield: Vec<Vec<tile::Tile>>,
    is_initialized: bool,
}

impl Field {
    pub fn new(height: usize, width: usize, num_mines: usize) -> Self {
        Field { height, width, num_mines, minefield: Vec::new(), is_initialized: false }
    }

    pub fn initialize(&mut self) -> () {
        for i in 0..self.height {
            self.minefield.push(Vec::new());
            for j in 0..self.width {
                self.minefield[i].push(tile::Tile::new(false, j, i));
            }
        }

        self.reset();
    }

    pub fn reset(&mut self) -> () {
        let mut rng: ThreadRng = rand::thread_rng();
        let mut minecount: usize = self.num_mines;
        while minecount > 0 {
            let (x, y) = (rng.gen_range(0..self.width), rng.gen_range(0..self.height));
            if self.minefield[y][x].has_mine { continue; }
            self.minefield[y][x].has_mine = true;
            minecount -= 1;
        }

        self.calculate_tile_counts();
    }

    pub fn print_field(& self) -> () {
        for row in &self.minefield {
            for tle in row {
                print!("[");

                match tle.neighbor_mines {
                    -2 => print!("*"),
                    0 => print!(" "),
                    _ => print!("{}", tle.neighbor_mines)
                }

                print!("]");
            }
            println!();
        }
    }

    pub fn highlight_print_field(& self, target_tile: tile::Tile) -> () {
        for row in &self.minefield {
            for tle in row {
                let is_target: bool = tle.x == target_tile.x && tle.y == target_tile.y;
                if is_target { print!("{{"); }
                else { print!("["); }

                match tle.neighbor_mines {
                    -2 => print!("*"),
                    0 => print!(" "),
                    _ => print!("{}", tle.neighbor_mines)
                }

                if is_target { print!("}}"); }
                else { print!("]"); }
            }
            println!();
        }
    }

    pub fn reveal_tile(&mut self, y: usize, x: usize) -> () {
        if y >= self.height || x >= self.width { return; }
        todo!();
    }


    fn calculate_tile_counts(&mut self) {
        let width = self.width;
        let height = self.height;
        for i in 0..self.width {
            for j in 0..self.height {
                let tle = self.tile_at(j, i);
                if tle.has_mine { self.mut_tile_at(j, i).neighbor_mines = -2; continue; }
                for k in -1..=1 {
                    for l in -1..=1 {
                        if k == 0 && l == 0 { continue; }
                        let (mut x, mut y) = (tle.x as i32, tle.y as i32);
                        x += k;
                        y += l;

                        if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 { continue; }

                        if self.tile_at(x as usize, y as usize).has_mine { self.mut_tile_at(tle.x as usize, tle.y as usize).neighbor_mines += 1; }
                        self.highlight_print_field(tle);
                        println!();
                        std::io::stdout().flush();
                    }
                }
            }
        }
    }

    fn mut_tile_at(&mut self, x: usize, y: usize) -> &mut tile::Tile {
        &mut self.minefield[y][x]
    }

    fn tile_at(& self, x: usize, y: usize) -> tile::Tile {
        self.minefield[y][x]
    }
}