use core::fmt;
use grid::*;

pub struct Board {
    grid: Grid<usize>,
    n: usize,
    pos_0: (usize, usize),
}

#[derive(PartialEq, Debug)]
pub enum Directions {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl fmt::Display for Directions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Directions::DOWN => write!(f, "DOWN"),
            Directions::LEFT => write!(f, "LEFT"),
            Directions::RIGHT => write!(f, "RIGHT"),
            Directions::UP => write!(f, "UP"),
        }
    }
}

impl Board {
    pub fn get_n(&self) -> usize {
        return self.n;
    }

    pub fn get_grid(&self) -> &Grid<usize> {
        return &self.grid;
    }

    pub fn new(board: Grid<usize>, n: usize, pos_0: (usize, usize)) -> Self {
        return Board {
            grid: board,
            n,
            pos_0,
        };
    }

    pub fn load_from_str(n: usize, puzzle_str: &str) -> Board {
        let mut puzzle_vec = vec![];
        let mut rows = puzzle_str.splitn(n.into(), "\n");
        let mut p_0 = (n, n);
        for i in 0..n {
            let row: &str = rows.next().expect("Out of rows").trim();
            for val_str in row.split(' ') {
                if val_str.is_empty() {
                    continue;
                }
                let val: usize = val_str
                    .parse()
                    .expect("Impossible to parse this str to usize");
                if val == 0 {
                    let y: usize = i;
                    let x: usize = puzzle_vec.len() - (n * y);
                    p_0 = (x, y);
                }
                puzzle_vec.push(val);
            }
        }
        if p_0.0 == n {
            panic!("No 0 found!")
        }
        let board = Grid::from_vec(puzzle_vec, n);
        Board {
            grid: board,
            n,
            pos_0: p_0,
        }
    }

    fn next_directions(&self) -> Vec<Directions> {
        let mut next_pos = vec![];
        if self.pos_0.1 > 0 {
            next_pos.push(Directions::DOWN);
        }
        if self.pos_0.1 < self.n - 1 {
            next_pos.push(Directions::UP);
        }
        if self.pos_0.0 > 0 {
            next_pos.push(Directions::RIGHT);
        }
        if self.pos_0.0 < self.n - 1 {
            next_pos.push(Directions::LEFT);
        }
        return next_pos;
    }

    pub fn next_boards(&self) -> Vec<Self> {
        let mut next_boards = vec![];
        for dir in self.next_directions() {
            let mut new_grid = self.get_grid().clone();
            let x = self.pos_0.0;
            let y = self.pos_0.1;
            let n = self.n;
            match dir {
                Directions::DOWN => {
                    new_grid.swap((x, y), (x, y - 1));
                    next_boards.push(Board {
                        grid: new_grid,
                        n,
                        pos_0: (x, y - 1),
                    });
                }
                Directions::UP => {
                    new_grid.swap((x, y), (x, y + 1));
                    next_boards.push(Board {
                        grid: new_grid,
                        n,
                        pos_0: (x, y + 1),
                    });
                }
                Directions::RIGHT => {
                    new_grid.swap((x - 1, y), (x, y));
                    next_boards.push(Board {
                        grid: new_grid,
                        n,
                        pos_0: (x - 1, y),
                    });
                }
                Directions::LEFT => {
                    new_grid.swap((x, y), (x + 1, y));
                    next_boards.push(Board {
                        grid: new_grid,
                        n,
                        pos_0: (x + 1, y),
                    });
                }
            }
        }
        return next_boards;
    }

    fn heuristic_manhattan(&self) -> usize {
        let mut manhattan: usize = 0;
        for j in 0..self.n {
            for i in 0..self.n {
                let val = self.grid[(j, i)];
                if val != 0 {
                    let x = (val - 1) % self.n;
                    let y = (val - 1) / self.n;
                    let dx: i16 = i16::abs(i16::try_from(x).unwrap() - i16::try_from(i).unwrap());
                    let dy: i16 = i16::abs(i16::try_from(y).unwrap() - i16::try_from(j).unwrap());
                    manhattan += usize::try_from(dx + dy).unwrap();
                }
            }
        }
        return manhattan;
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        return &self.grid == &other.grid;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_from_str_test() {
        /*
        3
        0 1 3
        4 2 5
        7 8 6
        */
        let n = 3;
        let input_str = "0 1 3\n4 2 5\n7 8 6";
        let board = Board::load_from_str(n, input_str);

        assert_eq!(board.n, 3);
        assert_eq!(board.pos_0, (0, 0));
    }

    #[test]
    fn next_directions_test1() {
        /*
        3
        0 1 3
        4 2 5
        7 8 6
        */
        let n = 3;
        let input_str = "0 1 3\n4 2 5\n7 8 6";
        let board = Board::load_from_str(n, input_str);
        let next_d = board.next_directions();

        assert_eq!(next_d.len(), 2);
        assert!(next_d.contains(&Directions::UP));
        assert!(next_d.contains(&Directions::LEFT));
    }

    #[test]
    fn next_directions_test2() {
        /*
        3
        6 1 3
        4 2 5
        7 8 0
        */
        let n = 3;
        let input_str = "6 1 3\n4 2 5\n7 8 0";
        let board = Board::load_from_str(n, input_str);
        let next_d = board.next_directions();

        assert_eq!(next_d.len(), 2);
        assert!(next_d.contains(&Directions::DOWN));
        assert!(next_d.contains(&Directions::RIGHT));
    }

    #[test]
    fn next_directions_test3() {
        /*
        3
        2 1 3
        4 0 5
        7 8 6
        */
        let n = 3;
        let input_str = "2 1 3\n4 0 5\n7 8 6";
        let board = Board::load_from_str(n, input_str);
        let next_d = board.next_directions();

        assert_eq!(next_d.len(), 4);
        assert!(next_d.contains(&Directions::UP));
        assert!(next_d.contains(&Directions::LEFT));
        assert!(next_d.contains(&Directions::DOWN));
        assert!(next_d.contains(&Directions::RIGHT));
    }

    #[test]
    fn next_boards_test1() {
        /*
        3
        0 1 3
        4 2 5
        7 8 6
        */
        let n = 3;
        let input_str = "0 1 3\n4 2 5\n7 8 6";
        let board = Board::load_from_str(n, input_str);
        let next_b = board.next_boards();

        assert_eq!(next_b.len(), 2);
        assert_eq!(*next_b[0].get_grid(), grid![[1, 0, 3] [4, 2, 5] [7, 8, 6]]);
        assert_eq!(*next_b[1].get_grid(), grid![[4, 1, 3] [0, 2, 5] [7, 8, 6]]);
    }

    #[test]
    fn next_boards_test2() {
        /*
        3
        6 1 3
        4 2 5
        7 8 0
        */
        let n = 3;
        let input_str = "6 1 3\n4 2 5\n7 8 0";
        let board = Board::load_from_str(n, input_str);
        let next_b = board.next_boards();

        assert_eq!(next_b.len(), 2);
        assert_eq!(*next_b[0].get_grid(), grid![[6, 1, 3] [4, 2, 5] [7, 0, 8]]);
        assert_eq!(*next_b[1].get_grid(), grid![[6, 1, 3] [4, 2, 0] [7, 8, 5]]);
    }

    #[test]
    fn heuristic_manhattan_test1() {
        /*
        3
        1 2 3
        4 5 6
        7 8 0
        */
        let n = 3;
        let input_str = "1 2 3\n4 5 6\n7 8 0";
        let board = Board::load_from_str(n, input_str);

        assert_eq!(board.heuristic_manhattan(), 0);
    }

    #[test]
    fn heuristic_manhattan_test2() {
        /*
        3
        2 1 3
        4 0 5
        7 8 6
        */
        let n = 3;
        let input_str = "2 1 3\n4 0 5\n7 8 6";
        let board = Board::load_from_str(n, input_str);

        assert_eq!(board.heuristic_manhattan(), 4);
    }
}
