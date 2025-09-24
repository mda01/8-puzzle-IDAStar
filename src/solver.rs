use grid::*;

use crate::board::Board;
use crate::board::Pos;
pub struct Solver {
    board: Board,
    target: Board,
}

impl Solver {
    pub fn new(board: Board) -> Self {
        let n = board.get_n();
        let mut target_vec = Vec::from_iter(1..n * n);
        target_vec.push(0);
        let target_grid = Grid::from_vec(target_vec, n.into());
        return Solver {
            board: board,
            target: Board::new(target_grid, n, Pos::new(n - 1, n - 1)),
        };
    }

    fn is_over(&self) -> bool {
        return self.board == self.target;
    }
}

#[cfg(test)]
mod tests {
    use crate::solver;

    use super::*;

    #[test]
    fn new_test() {
        /*
        3
        0 1 3
        4 2 5
        7 8 6
        */
        let n = 3;
        let input_str = "0 1 3\n4 2 5\n7 8 6";
        let board = Board::load_from_str(n, input_str);
        let solver = Solver::new(board);
        assert_eq!(solver.board.get_n(), 3);
        assert_eq!(
            *solver.board.get_board(),
            grid![[0, 1, 3][4, 2, 5][7, 8, 6]]
        );
        assert_eq!(
            *solver.target.get_board(),
            grid![[1, 2, 3][4, 5, 6][7, 8, 0]]
        );
    }

    #[test]
    fn is_over_test1() {
        /*
        3
        1 2 3
        4 5 6
        7 8 0
        */
        let n = 3;
        let input_str = "1 2 3\n4 5 6\n7 8 0";
        let board = Board::load_from_str(n, input_str);
        let solver = Solver::new(board);
        assert!(solver.is_over())
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
        let solver = Solver::new(board);
        assert!(!solver.is_over())
    }
}
