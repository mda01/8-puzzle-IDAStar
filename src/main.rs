pub mod board;
pub mod solver;

use crate::{board::Board, solver::Algo, solver::Solver};
use std::{fs, time};

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
    for path in fs::read_dir("test_puzzles").unwrap() {
        let path_str = path.unwrap().path();
        let puzzle_name = path_str.to_str().unwrap();
        if puzzle_name.len() != 25 {
            continue;
        }
        let time_start = time::SystemTime::now();
        let board = load_board(puzzle_name);
        let mut solver = Solver::new(board.get_n());
        solver.solve(board, Algo::ASTAR, board::Heuristics::LINCONFLICT);
        println!("elapsed time: {:?}\n", (time_start.elapsed().unwrap()));
    }
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
