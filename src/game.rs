use std::cmp;

pub const BOARD_LENGTH: usize = 3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellMarking {
    X,
    O,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell {
    mark: Option<CellMarking>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Move {
    pub position: (usize, usize),
    pub marking: CellMarking,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    cells: Vec<Cell>,
    row_size: usize,
    col_size: usize,
}

impl Board {
    pub fn new() -> Board {
        Board::with_dimensions(BOARD_LENGTH, BOARD_LENGTH)
    }

    pub fn with_dimensions(row_size: usize, col_size: usize) -> Board {
        let default_cell = Cell { mark: None };
        Board {
            cells: vec![default_cell; row_size * col_size],
            row_size: row_size,
            col_size: col_size,
        }
    }

    pub fn apply_move(&mut self, m: &Move) {
        self.get_cell_mut(&m.position).map(|ref mut cell| {
            cell.mark = Some(m.marking);
        });
    }

    pub fn undo_move(&mut self, m: &Move) {
        self.get_cell_mut(&m.position).map(|ref mut cell| {
            cell.mark = None;
        });
    }

    pub fn next_move(&mut self, player_marking: CellMarking) -> Option<Move> {
        if self.has_won().is_some() {
            return None;
        }

        let moves = self.moves(player_marking);
        let mut best = -1000;
        let mut best_move = None;

        let opposite_marking = match player_marking {
            CellMarking::X => CellMarking::O,
            CellMarking::O => CellMarking::X,
        };

        for m in moves {
            self.apply_move(&m);
            let move_value = minimax(self, 0, opposite_marking, false);
            self.undo_move(&m);

            if move_value > best {
                best = move_value;
                best_move = Some(m);
            }
        }

        best_move
    }

    pub fn moves(&self, marking: CellMarking) -> Vec<Move> {
        self.cells
            .iter()
            .enumerate()
            .filter(|&(_, cell)| cell.mark.is_none())
            .map(move |(i, _)| {
                let pos = self.index_to_pos(i);
                Move {
                    position: pos,
                    marking: marking,
                }
            })
            .collect()
    }

    pub fn index_to_pos(&self, index: usize) -> (usize, usize) {
        let row = index / self.row_size;
        let col = index % self.row_size;
        (row, col)
    }

    pub fn cell_index(&self, pos: &(usize, usize)) -> usize {
        pos.0 * self.row_size + pos.1
    }

    pub fn get_cell<'a>(&'a self, pos: &(usize, usize)) -> Option<&'a Cell> {
        let index = self.cell_index(pos);
        self.cells.get(index)
    }

    pub fn get_cell_mut<'a>(&'a mut self, pos: &(usize, usize)) -> Option<&'a mut Cell> {
        let index = self.cell_index(pos);
        self.cells.get_mut(index)
    }

    pub fn has_won(&self) -> Option<CellMarking> {
        self.check_line(self.col_size, self.row_size, |row, col| (row, col))
            .or_else(|| self.check_line(self.row_size, self.col_size, |col, row| (row, col)))
            .or_else(|| self.check_diagonals())
    }

    pub fn to_string(&self) -> String {
        let mut s = Vec::with_capacity(self.row_size * self.col_size + self.col_size);
        for row in 0..self.col_size {
            for col in 0..self.row_size {
                let cell = self.get_cell(&(row, col)).unwrap();
                let ch = match cell.mark {
                    Some(CellMarking::X) => b'X',
                    Some(CellMarking::O) => b'O',
                    None => b'_',
                };
                s.push(ch);
            }
            s.push(b'\n');
        }
        String::from_utf8(s).unwrap()
    }

    fn check_line<F>(&self, size1: usize, size2: usize, cell_fn: F) -> Option<CellMarking>
    where
        F: Fn(usize, usize) -> (usize, usize),
    {
        for a in 0..size1 {
            let mut marking = None;
            let mut count = 0;
            for b in 0..size2 {
                let pos = cell_fn(a, b);
                let cell_marking = self.get_cell(&pos).and_then(|cell| cell.mark);
                if cell_marking.is_some() && marking.is_none() {
                    marking = cell_marking;
                } else if cell_marking.is_none() || marking != cell_marking {
                    break;
                }
                count += 1;
            }
            if count == size2 && !marking.is_none() {
                return marking;
            }
        }

        None
    }

    fn check_diagonals(&self) -> Option<CellMarking> {
        let shortest_length = cmp::min(self.row_size, self.col_size);
        let top_right = (0, self.row_size - 1);

        self.check_diagonal((0, 0), 1, shortest_length)
            .or_else(|| self.check_diagonal(top_right, -1, shortest_length))
    }

    fn check_diagonal(
        &self,
        start: (usize, usize),
        direction: i32,
        shortest_length: usize,
    ) -> Option<CellMarking> {
        let mut diagonal_marking = None;
        let mut count = 0;
        for counter in 0..shortest_length {
            let offset = counter as i32 * direction;
            let row = start.0 + counter;
            let col = start.1 as i32 + offset;
            let position = (row, col as usize);
            let cell_marking = self.get_cell(&position).and_then(|cell| cell.mark);
            if cell_marking.is_some() && diagonal_marking.is_none() {
                diagonal_marking = cell_marking;
            } else if cell_marking.is_none() || diagonal_marking != cell_marking {
                break;
            }
            count += 1;
        }
        if count == shortest_length {
            diagonal_marking
        } else {
            None
        }
    }
}

fn minimax(board: &mut Board, depth: i32, player_marking: CellMarking, is_max: bool) -> i32 {
    if let Some(marking) = board.has_won() {
        if (player_marking == marking && !is_max) || (player_marking != marking && is_max) {
            return -10 - depth;
        }
        return 10 - depth;
    }

    let moves = board.moves(player_marking);
    if moves.len() == 0 {
        return 0;
    }

    let opposite_marking = match player_marking {
        CellMarking::X => CellMarking::O,
        CellMarking::O => CellMarking::X,
    };

    if is_max {
        let mut best = -1000;
        for m in moves {
            board.apply_move(&m);
            best = cmp::max(best, minimax(board, depth + 1, opposite_marking, !is_max));
            board.undo_move(&m);
        }
        best
    } else {
        let mut best = 1000;
        for m in moves {
            board.apply_move(&m);
            best = cmp::min(best, minimax(board, depth + 1, opposite_marking, !is_max));
            board.undo_move(&m);
        }
        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_BOARD_ROW_SIZE: usize = 3;
    const TEST_BOARD_COL_SIZE: usize = 5;

    #[test]
    fn test_has_won_row() {
        for marked_row in 0..TEST_BOARD_COL_SIZE {
            for marking in [CellMarking::X, CellMarking::O].iter() {
                let mut board = Board::with_dimensions(TEST_BOARD_ROW_SIZE, TEST_BOARD_COL_SIZE);
                for row in 0..TEST_BOARD_COL_SIZE {
                    for col in 0..TEST_BOARD_ROW_SIZE {
                        if row == marked_row {
                            let updated_cell = Cell {
                                mark: Some(*marking),
                            };
                            *board.get_cell_mut(&(row, col)).unwrap() = updated_cell;
                        }
                    }
                }
                assert_eq!(board.has_won(), Some(*marking));
            }
        }
    }

    #[test]
    fn test_has_won_col() {
        for marked_col in 0..TEST_BOARD_ROW_SIZE {
            for marking in [CellMarking::X, CellMarking::O].iter() {
                let mut board = Board::with_dimensions(TEST_BOARD_ROW_SIZE, TEST_BOARD_COL_SIZE);
                for row in 0..TEST_BOARD_COL_SIZE {
                    for col in 0..TEST_BOARD_ROW_SIZE {
                        if col == marked_col {
                            let updated_cell = Cell {
                                mark: Some(*marking),
                            };
                            *board.get_cell_mut(&(row, col)).unwrap() = updated_cell;
                        }
                    }
                }
                assert_eq!(board.has_won(), Some(*marking));
            }
        }
    }

    #[test]
    fn test_has_won_diagonal() {
        let mark = Cell {
            mark: Some(CellMarking::X),
        };
        let mut board = Board::with_dimensions(5, 3);
        *board.get_cell_mut(&(0, 0)).unwrap() = mark;
        *board.get_cell_mut(&(1, 1)).unwrap() = mark;
        *board.get_cell_mut(&(2, 2)).unwrap() = mark;
        assert_eq!(board.has_won(), Some(CellMarking::X));

        let mark = Cell {
            mark: Some(CellMarking::O),
        };
        let mut board = Board::with_dimensions(3, 5);
        *board.get_cell_mut(&(0, 2)).unwrap() = mark;
        *board.get_cell_mut(&(1, 1)).unwrap() = mark;
        *board.get_cell_mut(&(2, 0)).unwrap() = mark;
        assert_eq!(board.has_won(), Some(CellMarking::O));
    }

    #[test]
    fn test_has_won_diagonal_incomplete() {
        let mark = Cell {
            mark: Some(CellMarking::X),
        };
        let mut board = Board::with_dimensions(5, 3);
        *board.get_cell_mut(&(0, 0)).unwrap() = mark;
        *board.get_cell_mut(&(2, 2)).unwrap() = mark;
        assert_eq!(board.has_won(), None);
    }

    #[test]
    fn test_next_move_doesnt_modify_board() {
        let mut board = Board::with_dimensions(3, 3);
        let saved_board = board.clone();

        board.next_move(CellMarking::X);
        board.next_move(CellMarking::O);

        assert_eq!(board, saved_board);
    }

    #[test]
    fn test_index_to_pos() {
        let board = Board::with_dimensions(3, 3);
        for row in 0..board.col_size {
            for col in 0..board.row_size {
                let index = board.cell_index(&(row, col));
                assert_eq!(board.index_to_pos(index), (row, col));
            }
        }
    }
}
