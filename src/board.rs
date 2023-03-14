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

    fn get_box(&self, row: usize, col: usize) -> Vec<(&Cell, [usize; 2])> {
        // Get the top left cell of the box
        let top = (row as f32 / 3f32).floor() as usize * 3;
        let left = (col as f32 / 3f32).floor() as usize * 3;
        let mut cells = Vec::new();
        for i in 0..3 {
            for j in 0..3 {
                let row = j + top;
                let col = i + left;
                cells.push((&self.cells[row][col], [row, col]));
            }
        }
        cells
    }

    fn get_line(&self, pos: usize, dir: Direction) -> Vec<(&Cell, [usize; 2])> {
        let mut cells = Vec::new();
        for i in 0..9 {
            let row = if let Direction::X = dir { pos } else { i };
            let col = if let Direction::Y = dir { pos } else { i };
            cells.push((&self.cells[row][col], [row, col]));
        }
        cells
    }

    pub fn check_solved(&self) -> bool {
        for row in &self.cells {
            for cell in row {
                match cell {
                    Cell::Hint(_) => return false,
                    _ => {}
                }
            }
        }
        true
    }

    pub fn check_valid(&mut self) -> bool {
        self.trim_hints();
        // Check that there are no cells with no valid numbers
        for row in &self.cells {
            for cell in row {
                match cell {
                    Cell::Hint(hints) => {
                        if hints.is_empty() {
                            return false;
                        }
                    }
                    _ => {}
                }
            }
        }

        for i in 0..9 {
            for cells in [
                self.get_box(i, i % 3 * 3),
                self.get_line(i, Direction::X),
                self.get_line(i, Direction::Y),
            ] {
                // Check that there are no row / cols / boxes which cannot
                // contain a certain number
                let mut possible_vals = [false; 9];
                // Check that each row / col / box contains no duplicate numbers
                let mut found_vals = [false; 9];
                for (cell, _) in cells {
                    match cell {
                        Cell::Val(num) => {
                            possible_vals[*num as usize - 1] = true;
                            if found_vals[*num as usize - 1] == true {
                                return false;
                            }
                            found_vals[*num as usize - 1] = true
                        }
                        Cell::Hint(hints) => {
                            for num in hints {
                                possible_vals[*num as usize - 1] = true;
                            }
                        }
                    }
                }
                if possible_vals.contains(&false) {
                    return false;
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
                        .filter_map(|(cell, _)| match cell {
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

    // Strategy
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

    fn find_lonely_single(&self) -> Option<SolveStep> {
        for i in 0..9 {
            for cells in [
                self.get_box(i, i % 3 * 3),
                self.get_line(i, Direction::X),
                self.get_line(i, Direction::Y),
            ] {
                'nums: for num in 1..10 {
                    let mut found = false;
                    let mut found_pos = [0; 2];
                    for (cell, pos) in &cells {
                        match cell {
                            Cell::Hint(hints) => {
                                if hints.contains(&num) {
                                    // If already found, try the next number
                                    if found {
                                        continue 'nums;
                                    }
                                    found = true;
                                    found_pos = *pos;
                                }
                            }
                            _ => {}
                        }
                    }
                    if found {
                        return Some(SolveStep {
                            val: num,
                            pos: found_pos,
                        });
                    }
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
        let strategies = [Board::find_naked_single, Board::find_lonely_single];
        'outer: loop {
            self.trim_hints();
            for strategy in strategies {
                let step = strategy(self);
                if let Some(step) = step {
                    self.apply_step(step);
                    continue 'outer;
                }
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
