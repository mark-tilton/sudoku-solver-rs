use std::collections::HashSet;

pub enum Direction {
    X,
    Y,
}

pub struct Board {
    cells: [[u8; 9]; 9],
}

impl Board {
    pub fn from_cells(cells: [[u8; 9]; 9]) -> Self {
        Board { cells }
    }

    // This may only be used to verify if the box is valid.
    // If so, this should probably be refactored to just to that here.
    fn get_box(&self, x: usize, y: usize) -> [u8; 9] {
        // Get the top left cell
        let top = (y as f32 / 3f32).floor() as usize * 3;
        let left = (x as f32 / 3f32).floor() as usize * 3;
        let mut vals = [0; 9];
        for i in 0..3 {
            for j in 0..3 {
                vals[i * 3 + j] = self.cells[i + top][j + left];
            }
        }
        vals
    }

    fn get_line(&self, pos: usize, dir: Direction) -> [u8; 9] {
        let mut vals = [0; 9];
        for i in 0..9 {
            vals[i] = self.cells[if let Direction::X = dir { i } else { pos }]
                [if let Direction::Y = dir { i } else { pos }];
        }
        vals
    }

    pub fn check_valid(&self) -> bool {
        fn check_vals(vals: [u8; 9]) -> bool {
            let mut unique = HashSet::new();
            for val in vals {
                if val != 0 && unique.contains(&val) {
                    return false;
                }
                unique.insert(val);
            }
            true
        }
        // Check all the boxes
        for i in 0..3 {
            for j in 0..3 {
                let vals = self.get_box(i * 3, j * 3);
                if !check_vals(vals) {
                    return false;
                }
            }
        }
        for i in 0..9 {
            // Check all the rows
            if !check_vals(self.get_line(i, Direction::X)) {
                return false;
            }
            // Check all the columns
            if !check_vals(self.get_line(i, Direction::Y)) {
                return false;
            }
        }
        true
    }

    pub fn display(&self) {
        for i in 0..9 {
            for j in 0..9 {
                let val = self.cells[j][i];
                if val == 0 {
                    print!(" ");
                } else {
                    print!("{}", val);
                }
                if j % 3 == 2 && j != 8 {
                    print!("|");
                }
            }
            if i % 3 == 2 && i != 8 {
                println!("");
                print!("-----------");
            }
            println!("");
        }
    }
}
