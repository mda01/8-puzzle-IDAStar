use core::fmt;

use grid::*;

#[derive(PartialEq, Debug)]
struct Pos {
    x: u16,
    y: u16,
}
pub struct Board {
    board: Grid<u16>,
    pub n: u16,
    pos_0: Pos,
}

#[derive(PartialEq, Debug)]
enum Directions {
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
    pub fn load_from_str(n: u16, puzzle_str: &str) -> Board {
        let mut puzzle_vec = vec![];
        let mut rows = puzzle_str.splitn(n.into(), "\n");
        let mut p_0 = Pos { x: n, y: n };
        for i in 0..n {
            let row: &str = rows.next().expect("Out of rows").trim();
            for val_str in row.split(' ') {
                if val_str.is_empty() {
                    continue;
                }
                let val: u16 = val_str
                    .parse()
                    .expect("Impossible to parse this str to u16");
                if val == 0 {
                    let y: u16 = i.into();
                    let x: u16 = u16::try_from(puzzle_vec.len()).unwrap() - (n * y);
                    p_0 = Pos { x, y };
                }
                puzzle_vec.push(val);
            }
        }
        if p_0.x == n {
            panic!("No 0 found!")
        }
        let board = Grid::from_vec(puzzle_vec, n.into());
        Board {
            board: board,
            n,
            pos_0: p_0,
        }
    }

    fn next_positions(self) -> Vec<Directions> {
        let mut next_pos = vec![];
        if self.pos_0.y > 0 {
            next_pos.push(Directions::DOWN);
        }
        if self.pos_0.y < self.n - 1 {
            next_pos.push(Directions::UP);
        }
        if self.pos_0.x > 0 {
            next_pos.push(Directions::RIGHT);
        }
        if self.pos_0.y < self.n - 1 {
            next_pos.push(Directions::LEFT);
        }
        return next_pos;
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
        assert_eq!(board.pos_0, Pos { x: 0, y: 0 });
    }

    #[test]
    fn next_positions_test1() {
        /*
        3
        0 1 3
        4 2 5
        7 8 6
        */
        let n = 3;
        let input_str = "0 1 3\n4 2 5\n7 8 6";
        let board = Board::load_from_str(n, input_str);
        let next_p = board.next_positions();

        assert_eq!(next_p.len(), 2);
        assert!(next_p.contains(&Directions::UP));
        assert!(next_p.contains(&Directions::LEFT));
    }

    #[test]
    fn next_positions_test2() {
        /*
        3
        6 1 3
        4 2 5
        7 8 0
        */
        let n = 3;
        let input_str = "6 1 3\n4 2 5\n7 8 0";
        let board = Board::load_from_str(n, input_str);
        let next_p = board.next_positions();

        assert_eq!(next_p.len(), 2);
        assert!(next_p.contains(&Directions::DOWN));
        assert!(next_p.contains(&Directions::RIGHT));
    }

    #[test]
    fn next_positions_test3() {
        /*
        3
        2 1 3
        4 0 5
        7 8 6
        */
        let n = 3;
        let input_str = "2 1 3\n4 0 5\n7 8 6";
        let board = Board::load_from_str(n, input_str);
        let next_p = board.next_positions();

        assert_eq!(next_p.len(), 4);
        assert!(next_p.contains(&Directions::UP));
        assert!(next_p.contains(&Directions::LEFT));
        assert!(next_p.contains(&Directions::DOWN));
        assert!(next_p.contains(&Directions::RIGHT));
    }
}
