use grid::*;

pub struct Board {
    board: Grid<u16>,
}

impl Board {
    pub fn load_from_str(n: usize, puzzle_str: &str) -> Board {
        let mut puzzle_vec = vec![];
        let mut rows = puzzle_str.splitn(n, "\n");
        for _ in 0..n {
            let row: &str = rows.next().expect("Out of rows").trim();
            for val_str in row.split(' ') {
                if val_str.is_empty() {
                    continue;
                }
                let val: u16 = val_str
                    .parse()
                    .expect("Impossible to parse this str to u16");
                puzzle_vec.push(val);
            }
        }
        let board = Grid::from_vec(puzzle_vec, n);
        Board { board }
    }
}
