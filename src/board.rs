use std::collections::{HashMap, HashSet};

pub enum Direction {
    X,
    Y,
}

#[derive(PartialEq, Eq)]
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

    // Assumes that hints have been trimmed first.
    fn trim_pointing_hints(&mut self) {
        for i in 0..9 {
            'nums: for num in 1..10 {
                let box_cells = self.get_box(i, i % 3 * 3);
                let mut positions = Vec::new();
                for (cell, pos) in &box_cells {
                    match cell {
                        Cell::Hint(hints) => {
                            if hints.contains(&num) {
                                positions.push(*pos);
                            }
                        }
                        // If we have already placed this number, move on.
                        Cell::Val(val) => {
                            if val == &num {
                                continue 'nums;
                            }
                        }
                    }
                }
                if positions.len() < 2 {
                    continue;
                }
                let mut first_position: [Option<usize>; 2] =
                    [Some(positions[0][0]), Some(positions[0][1])];
                for pos in &positions {
                    for i in 0..2 {
                        if let Some(j) = first_position[i] {
                            if pos[i] != j {
                                first_position[i] = None;
                            }
                        }
                    }
                }
                let mut pointing_positions = Vec::new();
                for (i, dir) in [(0, Direction::X), (1, Direction::Y)] {
                    if let Some(c) = first_position[i] {
                        pointing_positions =
                            self.get_line(c, dir).iter().map(|(_, i)| *i).collect();
                    }
                }
                if pointing_positions.len() == 0 {
                    continue;
                }
                for pos in pointing_positions {
                    if positions.contains(&pos) {
                        continue;
                    }
                    match &mut self.cells[pos[0]][pos[1]] {
                        Cell::Hint(hints) => {
                            hints.remove(&num);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn trim_pairs(&mut self) {
        // let mut groups: Vec<(Vec<&Cell>, Vec<u8>)>;
        let mut group_map = HashMap::new();
        for i in 0..9 {
            for cells in [
                self.get_box(i, i % 3 * 3),
                self.get_line(i, Direction::X),
                self.get_line(i, Direction::Y),
            ] {
                let mut hint_map = HashMap::<u8, Vec<usize>>::new();
                for (i, (cell, _)) in cells.iter().enumerate() {
                    match cell {
                        Cell::Hint(hints) => {
                            for hint in hints {
                                match hint_map.get_mut(hint) {
                                    Some(positions) => {
                                        positions.push(i);
                                    }
                                    None => {
                                        hint_map.insert(*hint, vec![i]);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }

                let mut position_map = HashMap::<&Vec<usize>, Vec<u8>>::new();
                for (val, positions) in hint_map.iter() {
                    match position_map.get_mut(positions) {
                        Some(vals) => {
                            vals.push(*val);
                        }
                        None => {
                            position_map.insert(positions, vec![*val]);
                        }
                    }
                }

                for (positions, vals) in position_map.iter() {
                    if positions.len() != vals.len() {
                        continue;
                    }
                    for pos in *positions {
                        group_map.insert(cells[*pos].1, vals.clone());
                    }
                }
            }
        }
        for (pos, vals) in group_map {
            let cell = &mut self.cells[pos[0]][pos[1]];
            if let Cell::Hint(hints) = cell {
                hints.drain();
                hints.extend(vals)
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
            self.trim_pointing_hints();
            self.trim_pairs();
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
