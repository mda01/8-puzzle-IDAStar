use grid::*;

use crate::board::{Board, Directions, Heuristics};

use std::collections::{HashMap, HashSet};
use std::time;

#[derive(PartialEq)]
pub enum Algo {
    ASTAR,
    IDASTAR,
}
pub struct Solver {
    target: Board,
    is_over: bool,
    solution_path: Vec<Directions>,
    visited_cache: HashMap<(Board, usize), bool>,
}

/*fn print_vec(v: &Vec<usize>) {
    for val in v {
        println!("{val}")
    }
}*/

impl Solver {
    pub fn new(n: usize) -> Self {
        let mut target_vec = Vec::from_iter(1..n * n);
        target_vec.push(0);
        let target_grid = Grid::from_vec(target_vec, n);
        return Solver {
            target: Board::new(target_grid, n, (n - 1, n - 1)),
            is_over: false,
            solution_path: vec![],
            visited_cache: HashMap::new(),
        };
    }

    pub fn solution_path_to_string(&self) -> String {
        if !self.is_over {
            return "No solution!".to_string();
        }
        if self.solution_path.is_empty() {
            return "Puzzle is already won!".to_string();
        }
        println!("Solution found! {} steps:", self.solution_path.len());

        let mut solution = "".to_string();
        for step in self.solution_path.clone() {
            match step {
                Directions::DOWN => solution += "DOWN ",
                Directions::UP => solution += "UP ",
                Directions::LEFT => solution += "LEFT ",
                Directions::RIGHT => solution += "RIGHT ",
            }
        }
        return solution.to_string();
    }

    pub fn solve(&mut self, init_board: Board, algo: Algo, heuristic: Heuristics) {
        match algo {
            Algo::ASTAR => (), //self.a_star(init_board, vec![], 0, &heuristic),
            Algo::IDASTAR => self.id_a_star(init_board, &heuristic),
        }
        println!("{}", self.solution_path_to_string());
    }

    fn is_target(&self, board: &Board) -> bool {
        return *board == self.target;
    }

    fn id_a_star(&mut self, init_board: Board, heuristic: &Heuristics) {
        for i in init_board.heuristic(*heuristic)..100 {
            let time_start = time::SystemTime::now();
            println!("---------------------------------\nTrying depth {i}");

            self.visited_cache.clear();
            let mut path_states = HashSet::new();
            path_states.insert(init_board.clone());

            self.dfs(init_board.clone(), vec![], path_states, 0, i, heuristic);
            println!("Time spend in depth: {:?}\n", time_start.elapsed().unwrap());
            if self.is_over {
                break;
            }
        }
    }

    fn dfs(
        &mut self,
        current_board: Board,
        path: Vec<Directions>,
        path_states: HashSet<Board>,
        depth: usize,
        max_depth: usize,
        heuristic: &Heuristics,
    ) {
        if self.is_over || depth + current_board.heuristic(*heuristic) > max_depth {
            return;
        }

        // Check if this state has already been explored at this depth or less
        if let Some(&explored) = self.visited_cache.get(&(current_board.clone(), depth)) {
            if explored {
                return;
            }
        }

        if self.is_target(&current_board) {
            // println!("Target attained! Length of path: {}", path.len());
            self.is_over = true;
            self.solution_path = path.clone();
            return;
        }

        self.visited_cache
            .insert((current_board.clone(), depth), false);

        for dir in current_board.next_directions() {
            // println!("dir {dir}");
            let mut new_board = current_board.clone();
            new_board.make_move(dir);

            // avoid cycles
            if path_states.contains(&new_board) {
                continue;
            }

            let mut temp_path = path.clone();
            temp_path.push(dir.clone());

            let mut new_path_states = path_states.clone();
            new_path_states.insert(new_board.clone());

            self.dfs(
                new_board,
                temp_path,
                new_path_states,
                depth + 1,
                max_depth,
                &heuristic,
            );
        }
        self.visited_cache
            .insert((current_board.clone(), depth), true);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Directions::*;

    #[test]
    fn new_test() {
        /*
        3
        0 1 3
        4 2 5
        7 8 6
        */
        let n = 3;
        let solver = Solver::new(n);
        assert_eq!(solver.target.get_n(), 3);
        assert_eq!(
            *solver.target.get_grid(),
            grid![[1, 2, 3][4, 5, 6][7, 8, 0]]
        );
        assert_eq!(
            *solver.target.get_grid(),
            grid![[1, 2, 3][4, 5, 6][7, 8, 0]]
        );
    }

    #[test]
    fn solution_path_to_string_test() {
        let solution_vector = vec![DOWN, LEFT, LEFT, RIGHT, UP];
        let mut solver = Solver::new(1);
        solver.is_over = true;
        solver.solution_path = solution_vector;
        assert_eq!(solver.solution_path_to_string(), "DOWN LEFT LEFT RIGHT UP ");
    }

    #[test]
    fn is_target_test1() {
        /*
        3
        1 2 3
        4 5 6
        7 8 0
        */
        let n = 3;
        let input_str = "1 2 3\n4 5 6\n7 8 0";
        let board = Board::load_from_str(n, input_str);
        let solver = Solver::new(n);
        assert!(solver.is_target(&board))
    }

    #[test]
    fn is_over_test2() {
        /*
        3
        0 1 3
        4 2 5
        7 8 6
        */
        let n = 3;
        let input_str = "0 1 3\n4 2 5\n7 8 6";
        let board = Board::load_from_str(n, input_str);
        let solver = Solver::new(n);
        assert!(!solver.is_target(&board))
    }

    #[test]
    fn id_a_star_test() {
        /*
        3
        0 1 3
        4 2 5
        7 8 6
        */
        let n = 3;
        let input_str = "0 1 3\n4 2 5\n7 8 6";
        let board = Board::load_from_str(n, input_str);
        let mut solver = Solver::new(3);
        solver.solve(board, Algo::IDASTAR, Heuristics::NONE);
        assert!(solver.is_over);
        assert_eq!(solver.solution_path.len(), 4)
    }
}
