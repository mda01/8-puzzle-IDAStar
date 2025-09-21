pub mod board;

use crate::board::Board;
use std::fs;

fn main() {
    let puzzle_name = "test_puzzles/puzzle00.txt";
    let puzzle_str = fs::read_to_string(puzzle_name).expect("Could not read file");
    let mut puzzle_in = puzzle_str.splitn(2, '\n');
    let n: usize = puzzle_in
        .next()
        .expect("Error reading the first line, maybe the file is empty?")
        .parse()
        .expect("Impossible to parse this str to usize");
    println!("N = {n}");
    let puzzle = puzzle_in
        .next()
        .expect("File only contains the puzzle size");
    println!("{puzzle}");
    let board = Board::load_from_str(n, puzzle);
}
