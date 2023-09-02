// Solve Sudoku problems using Dancing Links.
use std::fmt;

use crate::backtracking::dancing_links::{
    DancingLinksError, DancingLinksIterator, ProblemOption, ProblemOptionBuilder,
};

#[derive(Debug, PartialEq, Eq)]
pub struct DancingSudokuSolution {
    rows: Vec<Vec<u8>>,
}

impl DancingSudokuSolution {
    fn create(values: &[u8]) -> Self {
        assert_eq!(values.len(), 81);
        let mut result: Vec<Vec<u8>> = Vec::with_capacity(9);
        for row in values.chunks(9) {
            result.push(row.to_vec());
        }
        DancingSudokuSolution { rows: result }
    }
}

impl fmt::Display for DancingSudokuSolution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row_chunk in self.rows.chunks(3) {
            write!(f, "+---+---+---+\n")?;
            for row in row_chunk {
                write!(
                    f,
                    "|{}{}{}|{}{}{}|{}{}{}|\n",
                    row[0], row[1], row[2], row[3], row[4], row[5], row[6], row[7], row[8]
                )?;
            }
        }
        write!(f, "+---+---+---+")
    }
}

// Each box has a postion [0, 81) where the numbering starts in the top left
// of the sudoku and increases along columns then rows.  The positions are
// therefore:
// +----------+----------+----------+
// |  0  1  2 |  3  4  5 |  6  7  8 |
// |  9 10 11 | 12 13 14 | 15 16 17 |
// | 18 19 20 | 21 22 23 | 24 25 26 |
// +----------+----------+----------+
// | 27 28 29 | 30 31 32 | 33 34 35 |
// | 36 37 38 | 39 40 41 | 42 43 44 |
// | 45 46 47 | 48 49 50 | 51 52 53 |
// +----------+----------+----------+
// | 54 55 56 | 57 58 59 | 60 61 62 |
// | 63 64 65 | 66 67 68 | 69 70 71 |
// | 72 73 74 | 75 76 77 | 78 79 80 |
// +----------+----------+----------+
//
// The row, column, and box can be found via:
//   row = pos / 9
//   col = pos mod 9
//   box = 3 * (row / 3) + (col / 3) =  3 * (pos / 27) + (pos mod 9) / 3

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct SudokuEntry {
    pub row: u8,   // Row number [0, 8]
    pub col: u8,   // Col number [0, 8]
    pub value: u8, // Value [1, 9]
}

impl SudokuEntry {
    // Create a vector of initial positions from a 81 element array that is either
    // 0 (indicating an unset value) or gives the value at that position.
    pub fn create_from_values(values: &[u8; 81]) -> Vec<Self> {
        values
            .iter()
            .enumerate()
            .filter(|(_, &val)| val != 0)
            .map(|(idx, val)| SudokuEntry {
                row: (idx / 9) as u8,
                col: (idx % 9) as u8,
                value: *val,
            })
            .collect()
    }

    pub fn create_from_vec(values: &Vec<u8>) -> Vec<Self> {
        values
            .iter()
            .enumerate()
            .filter(|(_, &val)| val != 0)
            .map(|(idx, val)| SudokuEntry {
                row: (idx / 9) as u8,
                col: (idx % 9) as u8,
                value: *val,
            })
            .collect()
    }

    fn box_index(&self) -> usize {
        (3 * (self.row / 3) + self.col / 3) as usize
    }
}

#[derive(Debug)]
pub struct DancingSudokuIterator {
    initial_position: Vec<SudokuEntry>,
    inner: DancingLinksIterator<SudokuItem, SudokuEntry>,
}

impl DancingSudokuIterator {
    pub fn new(initial_position: Vec<SudokuEntry>) -> Result<Self, DancingLinksError> {
        if initial_position.len() > 81 {
            return Err("Initial position has more than 81 values".into());
        }

        let mut used = [false; 81];
        let mut c_row = [0b1111111110u16; 9];
        let mut c_col = c_row.clone();
        let mut c_box = c_row.clone();
        for pos in &initial_position {
            if pos.row > 8 || pos.col > 8 || pos.value < 1 || pos.value > 9 {
                // Invalid initial input.
                return Err(DancingLinksError::new(format!(
                    "Invalid initial position: {:?}",
                    pos
                )));
            }
            let b = pos.box_index();
            let value = 1u16 << pos.value;
            if c_row[pos.row as usize] & c_col[pos.col as usize] & c_box[b] & value == 0 {
                // Conflict, no solution.
                return Err("Invalid initial position".into());
            }
            c_row[pos.row as usize] &= !value;
            c_col[pos.col as usize] &= !value;
            c_box[b] &= !value;
            let idx = 9 * pos.row as usize + pos.col as usize;
            if used[idx] {
                return Err(DancingLinksError::new(format!(
                    "Initial position had multiple values for row: {} col: {}",
                    pos.row, pos.col
                )));
            }
            used[idx] = true;
        }

        if initial_position.len() == 81 {
          // In this case we want the iterator to return the one solution,
          // so add one option with a single value.
          let single_option = initial_position[0].clone();
            return Ok(DancingSudokuIterator {
                initial_position,
                inner: DancingLinksIterator::new(vec![single_option])?
            });
        }
        // There will be at least this many, almost certainly more.
        let mut options = Vec::<SudokuEntry>::with_capacity(81 - initial_position.len());
        for pos in 0..81 {
            if used[pos] {
                continue;
            }
            let row_idx = pos / 9;
            let col_idx = pos % 9;
            let mut avail =
                (c_row[row_idx] & c_col[col_idx] & c_box[3 * (row_idx / 3) + col_idx / 3]) as i16;
            while avail != 0 {
                let next_value = avail & -avail;
                options.push(SudokuEntry {
                    row: row_idx as u8,
                    col: col_idx as u8,
                    value: next_value.trailing_zeros() as u8,
                });
                avail &= !next_value;
            }
        }
        Ok(DancingSudokuIterator {
            initial_position,
            inner: DancingLinksIterator::new(options)?,
        })
    }
}

impl Iterator for DancingSudokuIterator {
    type Item = DancingSudokuSolution;
    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(options) => {
                let mut rows: Vec<Vec<u8>> = Vec::with_capacity(9);
                for _ in 0..9 {
                    rows.push(vec![0u8; 9]);
                }
                for pos in &self.initial_position {
                    rows[pos.row as usize][pos.col as usize] = pos.value;
                }
                for pos in options {
                    rows[pos.row as usize][pos.col as usize] = pos.value;
                }
                Some(DancingSudokuSolution { rows })
            }
            None => None,
        }
    }
}

impl std::iter::FusedIterator for DancingSudokuIterator {}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
enum SudokuItem {
    Position { row: u8, col: u8 },
    Row { row: u8, value: u8 },
    Column { col: u8, value: u8 },
    Box { box_number: u8, value: u8 },
}

impl ProblemOption<SudokuItem> for SudokuEntry {
    type IteratorType = std::vec::IntoIter<SudokuItem>;
    type BuilderType = Self;

    fn primary_items(&self) -> Self::IteratorType {
        vec![
            SudokuItem::Position {
                row: self.row,
                col: self.col,
            },
            SudokuItem::Row {
                row: self.row,
                value: self.value,
            },
            SudokuItem::Column {
                col: self.col,
                value: self.value,
            },
            SudokuItem::Box {
                box_number: 3 * (self.row / 3) + self.col / 3,
                value: self.value,
            },
        ]
        .into_iter()
    }

    fn secondary_items(&self) -> Self::IteratorType {
        vec![].into_iter()
    }

    fn builder() -> Self::BuilderType {
        SudokuEntry {
            row: 0,
            col: 0,
            value: 0,
        }
    }
}

impl ProblemOptionBuilder<SudokuItem> for SudokuEntry {
    type ProblemOptionType = Self;

    fn add_primary(&mut self, item: &SudokuItem) -> &mut Self {
        match item {
            SudokuItem::Position { row, col } => {
                self.row = *row;
                self.col = *col;
            }
            SudokuItem::Row { row, value } => {
                self.row = *row;
                self.value = *value;
            }
            // Ignore the others, we have what we need and all Options will
            // provide both Position and Row.
            _ => (),
        }
        self
    }

    fn add_secondary(&mut self, _item: &SudokuItem) -> &mut Self {
        // There are no secondary langford items.
        self
    }

    fn build(self) -> Self::ProblemOptionType {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DancingSudokuIterator, DancingSudokuSolution, SudokuEntry
    };
    use claim::{assert_none, assert_ok, assert_some_eq};

    // A fully solved sudoku puzzle.
    #[rustfmt::skip]
    const SOL: [u8; 81] = [
      5, 3, 4, 6, 7, 8, 9, 1, 2, 
      6, 7, 2, 1, 9, 5, 3, 4, 8, 
      1, 9, 8, 3, 4, 2, 5, 6, 7, 
      8, 5, 9, 7, 6, 1, 4, 2, 3, 
      4, 2, 6, 8, 5, 3, 7, 9, 1, 
      7, 1, 3, 9, 2, 4, 8, 5, 6, 
      9, 6, 1, 5, 3, 7, 2, 8, 4, 
      2, 8, 7, 4, 1, 9, 6, 3, 5, 
      3, 4, 5, 2, 8, 6, 1, 7, 9,
    ];

    // A partially solved sudoku puzzle.
    #[rustfmt::skip]
    const PARTIAL: [u8; 81] = [
      0, 6, 9, 0, 1, 3, 7, 8, 0, 
      0, 7, 3, 0, 0, 8, 6, 0, 0, 
      8, 2, 0, 0, 9, 0, 3, 0, 0, 
      7, 0, 0, 9, 3, 1, 2, 6, 8, 
      1, 9, 6, 0, 8, 2, 4, 0, 3, 
      3, 8, 2, 4, 0, 0, 0, 0, 0, 
      6, 1, 7, 3, 2, 0, 8, 0, 4, 
      9, 3, 0, 8, 7, 0, 1, 2, 6, 
      2, 0, 8, 1, 0, 0, 0, 3, 7,
    ];

    #[test]
    fn solves_already_solved_puzzle() {
        let initial_position = SudokuEntry::create_from_values(&SOL);
        let mut iterator = assert_ok!(DancingSudokuIterator::new(initial_position));

        assert_some_eq!(iterator.next(), DancingSudokuSolution::create(&SOL));
        assert_none!(iterator.next());
    }

    #[test]
    fn solves_using_only_forced_moves() {
        // Take the last row of the full solution and remove all the values
        // on the bottom row.  As a result, all moves will be forced.
        let mut puzzle = SOL;
        for i in 72..81 {
            puzzle[i] = 0;
        }

        let initial_position = SudokuEntry::create_from_values(&puzzle);
        let mut iterator = assert_ok!(DancingSudokuIterator::new(initial_position));

        assert_some_eq!(iterator.next(), DancingSudokuSolution::create(&SOL));
        assert_none!(iterator.next());
    }

    #[test]
    fn solves_partial_solution() {
        #[rustfmt::skip]
        let expected_solution : [u8; 81] = [
            4, 6, 9, 5, 1, 3, 7, 8, 2, 
            5, 7, 3, 2, 4, 8, 6, 1, 9, 
            8, 2, 1, 6, 9, 7, 3, 4, 5, 
            7, 5, 4, 9, 3, 1, 2, 6, 8, 
            1, 9, 6, 7, 8, 2, 4, 5, 3, 
            3, 8, 2, 4, 5, 6, 9, 7, 1, 
            6, 1, 7, 3, 2, 5, 8, 9, 4, 
            9, 3, 5, 8, 7, 4, 1, 2, 6, 
            2, 4, 8, 1, 6, 9, 5, 3, 7,
          ];

        let initial_position = SudokuEntry::create_from_values(&PARTIAL);
        let mut iterator = assert_ok!(DancingSudokuIterator::new(initial_position));

        assert_some_eq!(
            iterator.next(),
            DancingSudokuSolution::create(&expected_solution)
        );
        assert_none!(iterator.next());
    }

    #[test]
    fn solves_medium_problem() {
        #[rustfmt::skip]
        let problem : [u8; 81] = [
          0, 2, 0, 0, 6, 0, 0, 0, 0,
          0, 0, 0, 0, 0, 1, 9, 5, 2,
          9, 0, 0, 8, 5, 2, 4, 7, 0,
          0, 0, 6, 4, 0, 0, 0, 0, 9,
          0, 0, 0, 0, 2, 0, 8, 0, 0,
          1, 0, 0, 0, 0, 8, 3, 6, 7,
          0, 0, 9, 7, 3, 0, 6, 0, 0,
          7, 0, 0, 0, 0, 0, 5, 9, 0,
          0, 0, 0, 6, 8, 9, 7, 0, 4
        ];

        #[rustfmt::skip]
        let expected_solution : [u8; 81] = [
          5, 2, 4, 9, 6, 7, 1, 8, 3,
          6, 7, 8, 3, 4, 1, 9, 5, 2,
          9, 3, 1, 8, 5, 2, 4, 7, 6,
          8, 5, 6, 4, 7, 3, 2, 1, 9,
          3, 9, 7, 1, 2, 6, 8, 4, 5,
          1, 4, 2, 5, 9, 8, 3, 6, 7,
          4, 8, 9, 7, 3, 5, 6, 2, 1,
          7, 6, 3, 2, 1, 4, 5, 9, 8,
          2, 1, 5, 6, 8, 9, 7, 3, 4
       ];

        let initial_position = SudokuEntry::create_from_values(&problem);
        let mut iterator = assert_ok!(DancingSudokuIterator::new(initial_position));

        assert_some_eq!(
            iterator.next(),
            DancingSudokuSolution::create(&expected_solution)
        );
        assert_none!(iterator.next());
    }

    #[test]
    fn solves_demanding_problem() {
      #[rustfmt::skip]
      let problem : [u8; 81] = [
        2, 0, 0, 0, 0, 9, 0, 0, 1,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        9, 0, 0, 0, 0, 0, 0, 3, 4,
        0, 0, 0, 0, 4, 0, 0, 0, 0,
        1, 0, 7, 0, 0, 8, 2, 0, 0,
        0, 2, 8, 1, 0, 5, 0, 0, 9,
        0, 5, 0, 6, 0, 1, 0, 0, 0,
        0, 0, 0, 8, 0, 0, 0, 6, 0,
        0, 0, 0, 4, 7, 0, 8, 0, 0
      ];
      #[rustfmt::skip]
      let expected_solution : [u8; 81] = [
          2, 3, 4, 5, 8, 9, 6, 7, 1,
          7, 6, 5, 3, 1, 4, 9, 8, 2,
          9, 8, 1, 7, 2, 6, 5, 3, 4,
          5, 9, 6, 2, 4, 7, 3, 1, 8,
          1, 4, 7, 9, 3, 8, 2, 5, 6,
          3, 2, 8, 1, 6, 5, 7, 4, 9,
          8, 5, 3, 6, 9, 1, 4, 2, 7,
          4, 7, 9, 8, 5, 2, 1, 6, 3,
          6, 1, 2, 4, 7, 3, 8, 9, 5
       ];

      let initial_position = SudokuEntry::create_from_values(&problem);
      let mut iterator = assert_ok!(DancingSudokuIterator::new(initial_position));

      assert_some_eq!(iterator.next(), DancingSudokuSolution::create(&expected_solution));
      assert_none!(iterator.next());
    }

    #[test]
    fn solves_extreme_problem() {
        #[rustfmt::skip]
        let problem : [u8; 81] = [
          4, 0, 0, 0, 0, 0, 0, 1, 0,
          0, 0, 0, 4, 0, 2, 3, 0, 0,
          8, 3, 6, 0, 1, 0, 0, 0, 0,
          2, 0, 0, 0, 6, 0, 0, 5, 7,
          0, 9, 0, 5, 0, 0, 6, 0, 1,
          0, 0, 7, 1, 0, 0, 0, 0, 0,
          0, 0, 0, 0, 8, 6, 0, 0, 3,
          7, 0, 0, 0, 0, 0, 0, 0, 0,
          6, 4, 0, 0, 7, 0, 0, 0, 2
       ];
        #[rustfmt::skip]
        let expected_solution : [u8; 81] = [
          4, 2, 9, 6, 3, 8, 7, 1, 5,
          1, 7, 5, 4, 9, 2, 3, 6, 8,
          8, 3, 6, 7, 1, 5, 2, 4, 9,
          2, 1, 4, 8, 6, 3, 9, 5, 7,
          3, 9, 8, 5, 4, 7, 6, 2, 1,
          5, 6, 7, 1, 2, 9, 8, 3, 4,
          9, 5, 1, 2, 8, 6, 4, 7, 3,
          7, 8, 2, 3, 5, 4, 1, 9, 6,
          6, 4, 3, 9, 7, 1, 5, 8, 2
         ];

        let initial_position = SudokuEntry::create_from_values(&problem);
        let mut iterator = assert_ok!(DancingSudokuIterator::new(initial_position));

        assert_some_eq!(
            iterator.next(),
            DancingSudokuSolution::create(&expected_solution)
        );
        assert_none!(iterator.next());
    }

    #[test]
    fn solves_minimum_clue_problem() {
        // This problem has only 17 clues, the minimum possible for the solution
        // to be unique.
        #[rustfmt::skip]
      let problem : [u8; 81] = [
        0, 0, 0, 0, 0, 0, 3, 0, 0,
        1, 0, 0, 4, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 1, 0, 5,
        9, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 2, 6, 0, 0,
        0, 0, 0, 0, 5, 3, 0, 0, 0,
        0, 5, 0, 8, 0, 0, 0, 0, 0,
        0, 0, 0, 9, 0, 0, 0, 7, 0,
        0, 8, 3, 0, 0, 0, 0, 4, 0
      ];
        #[rustfmt::skip]
      let expected_solution : [u8; 81] = [
        5, 9, 7, 2, 1, 8, 3, 6, 4,
        1, 3, 2, 4, 6, 5, 8, 9, 7,
        8, 6, 4, 3, 7, 9, 1, 2, 5,
        9, 1, 5, 6, 8, 4, 7, 3, 2,
        3, 4, 8, 7, 9, 2, 6, 5, 1,
        2, 7, 6, 1, 5, 3, 4, 8, 9,
        6, 5, 9, 8, 4, 7, 2, 1, 3,
        4, 2, 1, 9, 3, 6, 5, 7, 8,
        7, 8, 3, 5, 2, 1, 9, 4, 6 
       ];

        let initial_position = SudokuEntry::create_from_values(&problem);
        let mut iterator = assert_ok!(DancingSudokuIterator::new(initial_position));

        assert_some_eq!(
            iterator.next(),
            DancingSudokuSolution::create(&expected_solution)
        );
        assert_none!(iterator.next());
    }

    #[test]
    fn solves_non_unique_problem() {
        // This problem has only 16 clues, so the solution is not unique.
        #[rustfmt::skip]
        let problem : [u8; 81] = [
          0, 3, 0, 0, 1, 0, 0, 0, 0,
          0, 0, 0, 4, 0, 0, 1, 0, 0,
          0, 5, 0, 0, 0, 0, 0, 9, 0,
          2, 0, 0, 0, 0, 0, 6, 0, 4,
          0, 0, 0, 0, 3, 5, 0, 0, 0,
          1, 0, 0, 0, 0, 0, 0, 0, 0,
          4, 0, 0, 6, 0, 0, 0, 0, 0,
          0, 0, 0, 0, 0, 0, 0, 5, 0,
          0, 9, 0, 0, 0, 0, 0, 0, 0
        ];
        #[rustfmt::skip]
        let expected_solution1 : [u8; 81] = [
          9, 3, 4, 5, 1, 7, 2, 6, 8,
          8, 6, 2, 4, 9, 3, 1, 7, 5,
          7, 5, 1, 8, 6, 2, 4, 9, 3,
          2, 7, 5, 9, 8, 1, 6, 3, 4,
          6, 4, 9, 2, 3, 5, 8, 1, 7,
          1, 8, 3, 7, 4, 6, 5, 2, 9,
          4, 1, 7, 6, 5, 9, 3, 8, 2,
          3, 2, 8, 1, 7, 4, 9, 5, 6,
          5, 9, 6, 3, 2, 8, 7, 4, 1
        ];
        #[rustfmt::skip]
        let expected_solution2 : [u8; 81] = [
          9, 3, 4, 5, 1, 8, 2, 6, 7,
          7, 6, 2, 4, 9, 3, 1, 8, 5,
          8, 5, 1, 7, 6, 2, 4, 9, 3,
          2, 8, 5, 9, 7, 1, 6, 3, 4,
          6, 4, 9, 2, 3, 5, 7, 1, 8,
          1, 7, 3, 8, 4, 6, 5, 2, 9,
          4, 1, 8, 6, 5, 9, 3, 7, 2,
          3, 2, 7, 1, 8, 4, 9, 5, 6,
          5, 9, 6, 3, 2, 7, 8, 4, 1
        ];

        let initial_position = SudokuEntry::create_from_values(&problem);
        let mut iterator = assert_ok!(DancingSudokuIterator::new(initial_position));

        assert_some_eq!(
            iterator.next(),
            DancingSudokuSolution::create(&expected_solution1)
        );
        assert_some_eq!(
            iterator.next(),
            DancingSudokuSolution::create(&expected_solution2)
        );
        assert_none!(iterator.next());
    }

    #[test]
    fn very_hard_sudoku() {
        // This is the problem from Knuth 7.2.2.1.50
        #[rustfmt::skip]
        let problem : [u8; 81] = [
          1, 2, 0, 3, 0, 0, 4, 0, 0,
          4, 0, 0, 1, 0, 0, 0, 0, 0,
          0, 0, 5, 0, 6, 0, 0, 0, 0,
          3, 0, 0, 0, 0, 0, 0, 1, 0,
          0, 7, 0, 0, 0, 0, 2, 3, 0,
          0, 0, 0, 0, 0, 0, 6, 0, 8,
          0, 4, 0, 2, 0, 0, 0, 7, 0,
          0, 0, 9, 0, 8, 0, 0, 0, 0,
          0, 0, 0, 0, 0, 5, 0, 0, 6
        ];

        #[rustfmt::skip]
        let expected_solution : [u8; 81] = [
          1, 2, 8, 3, 7, 9, 4, 6, 5,
          4, 6, 7, 1, 5, 2, 9, 8, 3,
          9, 3, 5, 8, 6, 4, 7, 2, 1,
          3, 9, 4, 6, 2, 8, 5, 1, 7,
          8, 7, 6, 5, 9, 1, 2, 3, 4,
          2, 5, 1, 7, 4, 3, 6, 9, 8,
          5, 4, 3, 2, 1, 6, 8, 7, 9,
          6, 1, 9, 4, 8, 7, 3, 5, 2,
          7, 8, 2, 9, 3, 5, 1, 4, 6
        ];

        let initial_position = SudokuEntry::create_from_values(&problem);
        let mut iterator = assert_ok!(DancingSudokuIterator::new(initial_position));

        assert_some_eq!(
            iterator.next(),
            DancingSudokuSolution::create(&expected_solution)
        );
        assert_none!(iterator.next());
    }
}
