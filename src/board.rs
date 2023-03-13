use std::collections::HashSet;

pub enum Direction {
    X,
    Y,
}

pub struct Board {
    // Row major representation of all the board's values
    // (A list of rows, row first then column, dum dum)
    cells: [[u8; 9]; 9],
    solve_steps: Vec<SolveStep>,
}

struct SolveStep {
    pos: [usize; 2],
    val: u8,
}

impl Board {
    pub fn from_cells(cells: [[u8; 9]; 9]) -> Self {
        Board {
            cells,
            solve_steps: Vec::new(),
        }
    }

    // This may only be used to verify if the box is valid.
    // If so, this should probably be refactored to just to that here.
    fn get_box(&self, col: usize, row: usize) -> [u8; 9] {
        let cells = self.get_solved_cells();
        // Get the top left cell of the box
        let top = (row as f32 / 3f32).floor() as usize * 3;
        let left = (col as f32 / 3f32).floor() as usize * 3;
        let mut vals = [0; 9];
        for i in 0..3 {
            for j in 0..3 {
                vals[i * 3 + j] = cells[j + top][i + left];
            }
        }
        vals
    }

    fn get_line(&self, pos: usize, dir: Direction) -> [u8; 9] {
        let cells = self.get_solved_cells();
        let mut vals = [0; 9];
        for i in 0..9 {
            vals[i] = cells[if let Direction::X = dir { pos } else { i }]
                [if let Direction::Y = dir { pos } else { i }];
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

    fn find_naked_single(&self) -> Option<SolveStep> {
        let cells = self.get_solved_cells();
        for row in 0..9 {
            'nums: for col in 0..9 {
                if cells[row][col] != 0 {
                    continue;
                }
                let box_vals = self.get_box(col, row);
                let row_vals = self.get_line(row, Direction::X);
                let col_vals = self.get_line(col, Direction::Y);
                // println!("Box vals: {:?}", box_vals);
                // println!("Row vals: {:?}", row_vals);
                // println!("Col vals: {:?}", col_vals);
                // return None;
                let mut valid_num = 0;
                for num in 1..10 {
                    if !box_vals.contains(&num)
                        && !row_vals.contains(&num)
                        && !col_vals.contains(&num)
                    {
                        if valid_num != 0 {
                            continue 'nums;
                        }
                        valid_num = num;
                    }
                }
                if valid_num != 0 {
                    return Some(SolveStep {
                        pos: [row, col],
                        val: valid_num,
                    });
                }
            }
        }
        return None;
    }

    pub fn solve(&mut self) {
        loop {
            let step = self.find_naked_single();
            if let Some(step) = step {
                // println!("Found {} at ({}, {})", step.val, step.pos[0], step.pos[1]);
                // println!("Valid: {}", self.check_valid());
                // self.display(true);
                self.solve_steps.push(step);
                continue;
            }
            break;
        }
    }

    pub fn get_solved_cells(&self) -> [[u8; 9]; 9] {
        let mut cells = self.cells;
        for step in &self.solve_steps {
            cells[step.pos[0]][step.pos[1]] = step.val;
        }
        return cells;
    }

    pub fn display(&self, solved: bool) {
        let cells = if solved {
            self.get_solved_cells()
        } else {
            self.cells
        };
        for row in 0..9 {
            for col in 0..9 {
                let val = cells[row][col];
                if val == 0 {
                    print!(" ");
                } else {
                    print!("{}", val);
                }
                if col % 3 == 2 && col != 8 {
                    print!("|");
                }
            }
            if row % 3 == 2 && row != 8 {
                println!("");
                print!("-----------");
            }
            println!("");
        }
    }
}
