use core::fmt;
use grid::*;

#[derive(Clone, Hash, Eq)]
pub struct Board {
    grid: Grid<usize>,
    n: usize,
    pos_0: (usize, usize),
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Directions {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Clone, Copy)]
pub enum Heuristics {
    NONE,
    MANHATTAN,
    LINCONFLICT,
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
        self.n
    }

    pub fn get_grid(&self) -> &Grid<usize> {
        &self.grid
    }

    pub fn get_pos_0(&self) -> (usize, usize) {
        self.pos_0
    }

    pub fn new(board: Grid<usize>, n: usize, pos_0: (usize, usize)) -> Self {
        Board {
            grid: board,
            n,
            pos_0,
        }
    }

    pub fn heuristic(&self, heuristic_type: Heuristics) -> usize {
        match heuristic_type {
            Heuristics::NONE => 0,
            Heuristics::MANHATTAN => self.heuristic_manhattan(),
            Heuristics::LINCONFLICT => self.heuristic_linear_conflict(),
        }
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
                    p_0 = (y, x);
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

    pub fn next_directions(&self) -> Vec<Directions> {
        let mut next_pos = vec![];
        if self.pos_0.0 > 0 {
            next_pos.push(Directions::UP);
        }
        if self.pos_0.0 < self.n - 1 {
            next_pos.push(Directions::DOWN);
        }
        if self.pos_0.1 > 0 {
            next_pos.push(Directions::LEFT);
        }
        if self.pos_0.1 < self.n - 1 {
            next_pos.push(Directions::RIGHT);
        }
        return next_pos;
    }

    pub fn make_move(&mut self, move_d: Directions) {
        let x = self.pos_0.0;
        let y = self.pos_0.1;
        match move_d {
            Directions::UP => {
                self.grid.swap((x - 1, y), (x, y));

                self.pos_0 = (x - 1, y)
            }
            Directions::DOWN => {
                self.grid.swap((x + 1, y), (x, y));

                self.pos_0 = (x + 1, y);
            }
            Directions::RIGHT => {
                self.grid.swap((x, y + 1), (x, y));

                self.pos_0 = (x, y + 1);
            }
            Directions::LEFT => {
                self.grid.swap((x, y - 1), (x, y));

                self.pos_0 = (x, y - 1);
            }
        }
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
    fn heuristic_linear_conflict(&self) -> usize {
        // Commencer avec la distance Manhattan
        let mut total = self.heuristic_manhattan();

        // Ajouter les conflits linéaires pour les lignes
        total += self.count_row_conflicts();

        // Ajouter les conflits linéaires pour les colonnes
        total += self.count_col_conflicts();

        total
    }

    fn count_row_conflicts(&self) -> usize {
        let mut conflicts = 0;

        for row in 0..self.n {
            // Collecter les tuiles qui appartiennent à cette ligne
            let mut tiles_in_correct_row = Vec::new();

            for col in 0..self.n {
                let value = self.grid[(row, col)];
                if value != 0 {
                    // Calculer la ligne cible de cette tuile
                    let target_row = (value - 1) / self.n;

                    // Si la tuile est sur la bonne ligne
                    if target_row == row {
                        let target_col = (value - 1) % self.n;
                        tiles_in_correct_row.push((col, target_col));
                    }
                }
            }

            // Compter les conflits dans cette ligne
            conflicts += self.count_conflicts_in_line(&tiles_in_correct_row);
        }

        conflicts
    }

    fn count_col_conflicts(&self) -> usize {
        let mut conflicts = 0;

        for col in 0..self.n {
            // Collecter les tuiles qui appartiennent à cette colonne
            let mut tiles_in_correct_col = Vec::new();

            for row in 0..self.n {
                let value = self.grid[(row, col)];
                if value != 0 {
                    // Calculer la colonne cible de cette tuile
                    let target_col = (value - 1) % self.n;

                    // Si la tuile est sur la bonne colonne
                    if target_col == col {
                        let target_row = (value - 1) / self.n;
                        tiles_in_correct_col.push((row, target_row));
                    }
                }
            }

            // Compter les conflits dans cette colonne
            conflicts += self.count_conflicts_in_line(&tiles_in_correct_col);
        }

        conflicts
    }

    fn count_conflicts_in_line(&self, tiles: &[(usize, usize)]) -> usize {
        let mut conflicts = 0;
        let n = tiles.len();

        for i in 0..n {
            for j in (i + 1)..n {
                let (pos1, target1) = tiles[i];
                let (pos2, target2) = tiles[j];

                // Conflit si les tuiles sont dans le mauvais ordre relatif
                // par rapport à leurs positions cibles
                if (pos1 < pos2 && target1 > target2) || (pos1 > pos2 && target1 < target2) {
                    conflicts += 2; // Chaque conflit coûte 2 mouvements supplémentaires
                }
            }
        }

        conflicts
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
        assert!(next_d.contains(&Directions::RIGHT));
        assert!(next_d.contains(&Directions::DOWN));
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
        assert!(next_d.contains(&Directions::UP));
        assert!(next_d.contains(&Directions::LEFT));
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
    fn make_move_test1() {
        /*
        3
        0 1 3
        4 2 5
        7 8 6
        */
        let n = 3;
        let input_str = "0 1 3\n4 2 5\n7 8 6";
        let mut board = Board::load_from_str(n, input_str);
        board.make_move(Directions::RIGHT);

        assert_eq!(*board.get_grid(), grid![[1, 0, 3] [4, 2, 5] [7, 8, 6]]);
    }

    #[test]
    fn make_move_test2() {
        /*
        3
        6 1 3
        4 2 5
        7 8 0
        */
        let n = 3;
        let input_str = "6 1 3\n4 2 5\n7 8 0";
        let mut board = Board::load_from_str(n, input_str);
        board.make_move(Directions::UP);

        assert_eq!(*board.get_grid(), grid![[6, 1, 3] [4, 2, 0] [7, 8, 5]]);
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

    #[test]
    fn heuristic_linear_conflict_test() {
        /*
        Board actuel:
        2 1 3
        4 0 5
        7 8 6

        Analyse:
        - Manhattan: 4 (calculé dans le test existant)
        - Conflits ligne 0: tuiles 1 et 2 sont sur la bonne ligne mais inversées → +2
        - Conflits ligne 1: tuile 5 est sur la bonne ligne, position correcte → 0
        - Conflits ligne 2: tuile 6 est sur la bonne ligne mais mauvaise colonne → pas de conflit car pas même ligne cible
        - Conflits colonnes: aucun conflit détecté

        Total attendu: Manhattan(4) + Conflits(2) = 6
        */
        let n = 3;
        let input_str = "2 1 3\n4 0 5\n7 8 6";
        let board = Board::load_from_str(n, input_str);

        let linear_conflict_result = board.heuristic(Heuristics::LINCONFLICT);

        // Vérifier que Linear Conflict >= Manhattan
        let manhattan_result = board.heuristic(Heuristics::MANHATTAN);
        assert!(linear_conflict_result >= manhattan_result);

        // Vérifier le résultat exact
        // Manhattan = 4, Conflits linéaires = 2, Total = 6
        assert_eq!(linear_conflict_result, 6);
    }
}
