use std::fs;

mod board;
use crate::board::Board;

fn load_boards(file_name: &str) -> Result<Vec<Board>, std::io::Error> {
    let file_contents = fs::read_to_string(file_name)?;
    let mut boards = Vec::new();
    let mut cells = [[0; 9]; 9];
    let mut row = 9;
    for line in file_contents.lines() {
        if row == 9 {
            row = 0;
            cells = [[0; 9]; 9];
            continue;
        }

        for (i, val) in line.chars().enumerate() {
            let num = val.to_digit(10);
            if let Some(num) = num {
                cells[row][i] = num as u8;
            }
        }

        row += 1;
        if row == 9 {
            boards.push(Board::from_vals(cells));
        }
    }
    Ok(boards)
}

fn main() -> Result<(), std::io::Error> {
    let mut boards = load_boards("sudoku_boards.txt")?;
    let board = &mut boards[0];
    board.display();
    board.solve();
    println!("");
    board.display();
    println!("Valid: {}", board.check_valid());
    Ok(())
}
