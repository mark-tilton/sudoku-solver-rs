use std::collections::HashSet;

pub enum Direction {
    X,
    Y,
}

pub enum Cell {
    Val(u8),
    Hint(HashSet<u8>),
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Hint(HashSet::from_iter(1..10))
    }
}

pub struct Board {
    // Row major representation of all the board's values
    // (A list of rows, row first then column, dum dum)
    cells: [[Cell; 9]; 9],
    solve_steps: Vec<SolveStep>,
}

struct SolveStep {
    pos: [usize; 2],
    val: u8,
}

impl Board {
    pub fn from_vals(vals: [[u8; 9]; 9]) -> Self {
        let mut cells: [[Cell; 9]; 9] = Default::default();
        for row in 0..9 {
            for col in 0..9 {
                let val = vals[row][col];
                if val != 0 {
                    cells[row][col] = Cell::Val(val);
                }
            }
        }
        Board {
            cells,
            solve_steps: Vec::new(),
        }
    }

    // This may only be used to verify if the box is valid.
    // If so, this should probably be refactored to just to that here.
    fn get_box(&self, row: usize, col: usize) -> Vec<&Cell> {
        // Get the top left cell of the box
        let top = (row as f32 / 3f32).floor() as usize * 3;
        let left = (col as f32 / 3f32).floor() as usize * 3;
        let mut cells = Vec::new();
        for i in 0..3 {
            for j in 0..3 {
                cells.push(&self.cells[j + top][i + left]);
            }
        }
        cells
    }

    fn get_line(&self, pos: usize, dir: Direction) -> Vec<&Cell> {
        let mut cells = Vec::new();
        for i in 0..9 {
            cells.push(
                &self.cells[if let Direction::X = dir { pos } else { i }]
                    [if let Direction::Y = dir { pos } else { i }],
            );
        }
        cells
    }

    pub fn check_valid(&mut self) -> bool {
        self.trim_hints();
        for row in &self.cells {
            for val in row {
                match val {
                    Cell::Hint(hints) => {
                        if hints.is_empty() {
                            return false;
                        }
                    }
                    _ => {}
                }
            }
        }
        true
    }

    fn trim_hints(&mut self) {
        for row in 0..9 {
            for col in 0..9 {
                let box_vals = self.get_box(row, col);
                let row_vals = self.get_line(row, Direction::X);
                let col_vals = self.get_line(col, Direction::Y);
                let adjacent_vals = HashSet::<_>::from_iter(
                    box_vals
                        .iter()
                        .chain(row_vals.iter())
                        .chain(col_vals.iter())
                        .filter_map(|cell| match cell {
                            Cell::Val(num) => Some(*num),
                            _ => None,
                        }),
                );
                if let Cell::Hint(hints) = &mut self.cells[row][col] {
                    for num in adjacent_vals {
                        hints.remove(&num);
                    }
                }
            }
        }
    }

    fn find_naked_single(&self) -> Option<SolveStep> {
        for row in 0..9 {
            for col in 0..9 {
                match &self.cells[row][col] {
                    Cell::Hint(hints) => {
                        if let Some(val) = hints.iter().next() {
                            if hints.len() == 1 {
                                return Some(SolveStep {
                                    pos: [row, col],
                                    val: *val,
                                });
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        None
    }

    fn apply_step(&mut self, step: SolveStep) {
        self.cells[step.pos[0]][step.pos[1]] = Cell::Val(step.val);
        self.solve_steps.push(step);
    }

    pub fn solve(&mut self) {
        loop {
            self.trim_hints();
            let step = self.find_naked_single();
            if let Some(step) = step {
                self.apply_step(step);
                continue;
            }
            break;
        }
    }

    pub fn display(&self) {
        for row in 0..9 {
            for col in 0..9 {
                match self.cells[row][col] {
                    Cell::Val(val) => {
                        print!("{}", val)
                    }
                    Cell::Hint(_) => print!(" "),
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
