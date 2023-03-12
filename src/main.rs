use std::fs;

mod board;
use crate::board::Board;

fn load_boards(file_name: &str) -> Result<Vec<Board>, std::io::Error> {
    let file_contents = fs::read_to_string(file_name)?;
    let mut boards = Vec::new();
    let mut cells = [[0; 9]; 9];
    let mut pos = 9;
    for line in file_contents.lines() {
        if pos == 9 {
            pos = 0;
            cells = [[0; 9]; 9];
            continue;
        }

        for (i, val) in line.chars().enumerate() {
            let num = val.to_digit(10);
            if let Some(num) = num {
                cells[i][pos] = num as u8;
            }
        }

        pos += 1;
        if pos == 9 {
            boards.push(Board::from_cells(cells));
        }
    }
    Ok(boards)
}

fn main() -> Result<(), std::io::Error> {
    let boards = load_boards("sudoku_boards.txt")?;
    let board = &boards[0];
    board.display();
    println!("Valid: {}", board.check_valid());
    Ok(())
}
