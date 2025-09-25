use grid::*;

use crate::board::Board;
use crate::board::Directions;
use crate::board::Heuristics;

#[derive(PartialEq)]
pub enum Algo {
    IDDFS,
    ASTAR,
    IDASTAR,
}
pub struct Solver {
    target: Board,
    is_over: bool,
    solution_path: Vec<Directions>,
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
        };
    }

    pub fn solution_path_to_string(&self) -> String {
        if self.solution_path.is_empty() {
            return "No solution!".to_string();
        }
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

    pub fn solve(&mut self, init_board: &Board, algo: Algo, heuristic: Heuristics) {
        match algo {
            Algo::IDDFS => self.id_dfs(init_board, &heuristic),
            Algo::ASTAR => (),
            Algo::IDASTAR => (),
        }
    }

    fn is_target(&self, board: &Board) -> bool {
        return *board == self.target;
    }

    fn id_dfs(&mut self, init_board: &Board, heuristic: &Heuristics) {
        for i in 1..100 {
            self.dfs(&init_board.clone(), vec![], 1, i, heuristic);
            if !self.solution_path.is_empty() {
                println!("Solution found at depth {}!", i - 1);
                println!("{}", self.solution_path_to_string());
                break;
            } else {
                println!("Trying depth {}", i)
            }
        }
    }

    fn dfs(
        &mut self,
        current_board: &Board,
        path: Vec<Directions>,
        depth: usize,
        max_depth: usize,
        heuristic: &Heuristics,
    ) {
        if depth + current_board.heuristic(*heuristic) > max_depth || self.is_over {
            return;
        }
        if self.is_target(current_board) {
            // println!("Target attained! Length of path: {}", path.len());
            self.is_over = true;
            self.solution_path = path.clone();
            return;
        }

        for dir in current_board.next_directions() {
            // println!("dir {dir}");
            let mut temp_path = path.clone();
            temp_path.push(dir.clone());
            let mut new_board = current_board.clone();
            new_board.make_move(dir);
            self.dfs(&new_board, temp_path, depth + 1, max_depth, &heuristic);
        }
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
    fn dfs_test() {
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
        solver.dfs(&board, vec![], 1, 5, &Heuristics::NONE);
        assert!(solver.is_over);
        assert!(!solver.solution_path.is_empty());
    }

    #[test]
    fn id_dfs_test() {
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
        solver.solve(&board, Algo::IDDFS, Heuristics::NONE);
        assert!(solver.is_over);
        assert_eq!(solver.solution_path.len(), 4)
    }
}
