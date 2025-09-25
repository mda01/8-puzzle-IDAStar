pub mod board;
pub mod solver;

use crate::{board::Board, solver::Algo, solver::Solver};
use std::fs;

pub fn load_board(puzzle_name: &str) -> Board {
    let puzzle_str = fs::read_to_string(puzzle_name).expect("Could not read file");
    let mut puzzle_in = puzzle_str.splitn(2, '\n');
    let n: usize = puzzle_in
        .next()
        .expect("Error reading the first line, maybe the file is empty?")
        .parse()
        .expect("Impossible to parse this str to usize");
    // println!("N = {n}");
    let puzzle = puzzle_in
        .next()
        .expect("File only contains the puzzle size");
    // println!("{puzzle}");
    let board = Board::load_from_str(n, puzzle);
    return board;
}

fn main() {
    let puzzle_name = "test_puzzles/puzzle25.txt";
    let board = load_board(puzzle_name);
    let mut solver = Solver::new(board.get_n());
    solver.solve(&board, Algo::IDDFS, board::Heuristics::MANHATTAN);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_board_test() {
        let board = load_board("test_puzzles/puzzle00.txt");
        assert_eq!(board.get_n(), 10);
    }
}
