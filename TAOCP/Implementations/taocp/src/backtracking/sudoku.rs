// A sudoku solver using basic backtracking.
//
// If there is more than one solution, this will return an arbitrary one.

use std::error::Error;
use std::fmt;

// How the values in the puzzle are represented.
const VALUES: [u16; 10] = [
  0, // Not used.
  1,
  1 << 1,
  1 << 2,
  1 << 3,
  1 << 4,
  1 << 5,
  1 << 6,
  1 << 7,
  1 << 8,
];

#[derive(Debug, PartialEq, Eq)]
pub struct SudokuSolution {
  rows: Vec<Vec<u8>>,
}

impl SudokuSolution {
  // This checks that the inputs are the right size and in range, but does not
  // validate that the solution is valid.
  pub fn create(values: &[u8]) -> Self {
    assert_eq!(values.len(), 81);
    let mut result: Vec<Vec<u8>> = Vec::with_capacity(9);
    for row in values.chunks(9) {
      assert!(*row.iter().min().unwrap() >= 1);
      assert!(*row.iter().max().unwrap() <= 9);
      result.push(row.to_vec());
    }
    SudokuSolution { rows: result }
  }

  // Unsafe because the inputs are not checked.
  unsafe fn create_from_bitencoded(values: &[u16]) -> Self {
    let mut result: Vec<Vec<u8>> = Vec::with_capacity(9);
    for row in values.chunks(9) {
      result.push(row.iter().map(|v| (16 - v.leading_zeros()) as u8).collect());
    }
    SudokuSolution { rows: result }
  }
}

impl fmt::Display for SudokuSolution {
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
//    0  1  2  3  4  5  6  7  8
//    9 10 11 12 13 14 15 16 17
//   18 19 20 21 22 23 24 25 26
//   27 28 29 30 31 32 33 34 35
//   36 37 38 39 40 41 42 43 44
//   45 46 47 48 49 50 51 52 53
//   54 55 56 57 58 59 60 61 62
//   63 64 65 66 67 68 69 70 71
//   72 73 74 75 76 77 78 79 80
//
// The row, column, and box can be found via:
//   row = pos / 9
//   col = pos mod 9
//   box = 3 * (row / 3) + (col / 3) = 3 pos / 27 + (pos mod 9) / 3

fn translate_position(pos: usize) -> (usize, usize, usize) {
  let r = pos / 9;
  let c = pos % 9;
  (r, c, 3 * (r / 3) + c / 3)
}

// Errors for bad inputs.

#[derive(Debug)]
struct SudokuError {
  details: String,
}

impl SudokuError {
  fn new(msg: String) -> Self {
    SudokuError { details: msg }
  }

  fn from_str(msg: &str) -> Self {
    SudokuError {
      details: msg.to_string(),
    }
  }
}

impl Error for SudokuError {}

impl fmt::Display for SudokuError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.details)
  }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
struct Move {
  pos: u8,    // Position in sudoku, [0, 81)
  value: u16, // Bitwise encoded current value at position.
  avail: u16, // Bitwise encoded other available moves at position.
}

#[derive(Debug)]
struct SolutionState {
  l: usize,        // Current level.
  x: Vec<Move>,    // Current moves.  Only elements [0, l) are valid.
  c_row: Vec<u16>, // Available values for each row.
  c_col: Vec<u16>, // Available values for each column.
  c_box: Vec<u16>, // Available values for each box.
  n_active: usize, // Number of active (available) values in m.
  m: Vec<u8>,      // All currently free (unset) squares. Elements [0, n_active) are valid.
}

impl SolutionState {
  // Creates a new solution.  The input is a vector of length 81 containing
  // the values [1, 9] for pre-filled squares and 0 for an empty square.
  pub fn create(input: &[u8]) -> Result<SolutionState, SudokuError> {
    if input.len() != 81 {
      return Err(SudokuError::new(format!(
        "Invalid input length {} != 81",
        input.len()
      )));
    }
    let mut unused = vec![true; 81];
    let mut c_row = vec![(1 << 9) - 1; 9];
    let mut c_col = c_row.clone();
    let mut c_box = c_row.clone();
    for (pos, val) in input.iter().enumerate().filter(|(_, &v)| v > 0) {
      if *val > 9 {
        return Err(SudokuError::new(format!(
          "Invalid input value {} at position {}.",
          *val, pos
        )));
      }
      let encoded = VALUES[*val as usize];
      let (r, c, b) = translate_position(pos);
      if c_row[r] & c_col[c] & c_box[b] & encoded == 0 {
        return Err(SudokuError::new(format!(
          "Invalid input; can't have {} at pos {}.",
          *val, pos
        )));
      }
      c_row[r] &= !encoded;
      c_col[c] &= !encoded;
      c_box[b] &= !encoded;
      unused[pos] = false;
    }

    let m: Vec<u8> = unused
      .iter()
      .enumerate()
      .filter(|(_, &v)| v)
      .map(|(p, _)| p as u8)
      .collect();
    let n_active = m.len();
    let mut x = Vec::with_capacity(n_active);
    x.resize_with(n_active, Default::default);
    Ok(SolutionState {
      l: 0,
      x: x,
      c_row: c_row,
      c_col: c_col,
      c_box: c_box,
      n_active: n_active,
      m: m,
    })
  }

  // Selects the next move.  The move with the fewest possible options
  // is selected.  Returns none if no move is possible, either because there
  // are no unset squares or because one square has no options.
  fn select_move(&self) -> Option<Move> {
    self.m[..self.n_active]
      .iter()
      .map(|&mv| self.get_move(mv))
      .min_by_key(|&mv| mv.avail.count_ones())
  }

  fn get_move(&self, pos: u8) -> Move {
    let (r, c, b) = translate_position(pos as usize);
    Move {
      pos,
      value: 0,
      avail: self.c_row[r] & self.c_col[c] & self.c_box[b],
    }
  }
}

#[cfg(test)]
mod tests {
  // A fully solved sudoku square.
  const SOL: [u8; 81] = [
    5, 3, 4, 6, 7, 8, 9, 1, 2, 6, 7, 2, 1, 9, 5, 3, 4, 8, 1, 9, 8, 3, 4, 2, 5, 6, 7, 8, 5, 9, 7, 6,
    1, 4, 2, 3, 4, 2, 6, 8, 5, 3, 7, 9, 1, 7, 1, 3, 9, 2, 4, 8, 5, 6, 9, 6, 1, 5, 3, 7, 2, 8, 4, 2,
    8, 7, 4, 1, 9, 6, 3, 5, 3, 4, 5, 2, 8, 6, 1, 7, 9,
  ];

  mod sudoku_solution {
    use super::SOL;
    use crate::backtracking::sudoku::{SudokuSolution, VALUES};
    use std::fmt::Write;

    #[test]
    fn normal_and_bitencoded_create_agree() {
      let base = SudokuSolution::create(&SOL);
      let encoded_sol: Vec<u16> = SOL.iter().map(|v| VALUES[*v as usize]).collect();
      let bit: SudokuSolution = unsafe { SudokuSolution::create_from_bitencoded(&encoded_sol) };

      assert_eq!(base, bit);
    }

    #[test]
    fn formatter_produces_expected_output() {
      let encoded_sol: Vec<u16> = SOL.iter().map(|v| VALUES[*v as usize]).collect();
      let s = unsafe { SudokuSolution::create_from_bitencoded(&encoded_sol) };

      let mut buf = String::new();
      write!(&mut buf, "{}", s);

      let expected = "+---+---+---+\n\
                    |534|678|912|\n\
                    |672|195|348|\n\
                    |198|342|567|\n\
                    +---+---+---+\n\
                    |859|761|423|\n\
                    |426|853|791|\n\
                    |713|924|856|\n\
                    +---+---+---+\n\
                    |961|537|284|\n\
                    |287|419|635|\n\
                    |345|286|179|\n\
                    +---+---+---+";
      assert_eq!(buf, expected);
    }
  }

  mod solution_state {
    use super::SOL;
    use crate::backtracking::sudoku::{SolutionState, VALUES};

    #[test]
    fn full_solution_input() {
      match SolutionState::create(&SOL) {
        Ok(s) => {
          assert_eq!(s.n_active, 0);
          assert_eq!(s.c_row, vec![0; 9]);
          assert_eq!(s.c_col, vec![0; 9]);
          assert_eq!(s.c_box, vec![0; 9]);
        }
        Err(e) => assert!(
          false,
          "Valid solution should be acceptable initialization, got error {}",
          e
        ),
      }
    }

    #[test]
    fn invalid_input_value() {
      let mut bad_input = vec![10];
      bad_input.resize_with(81, Default::default);

      match SolutionState::create(&bad_input) {
        Ok(_) => assert!(false, "Expected input error."),
        Err(e) => assert_eq!(e.to_string(), "Invalid input value 10 at position 0."),
      };
    }

    #[test]
    fn conflicting_input() {
      let mut bad_input = vec![0, 1, 1, 2, 3, 4, 5, 6, 7];
      bad_input.resize_with(81, Default::default);

      match SolutionState::create(&bad_input) {
        Ok(_) => assert!(false, "Expected input error."),
        Err(e) => assert_eq!(e.to_string(), "Invalid input; can't have 1 at pos 2."),
      };
    }

    #[test]
    fn select_single_move_row() {
      let mut input = vec![1, 3, 4, 5, 0, 7, 6, 8, 9];
      input.resize_with(81, Default::default);
      let s = SolutionState::create(&input).unwrap();

      match s.select_move() {
        Some(m) => {
          assert_eq!(m.pos, 4);
          assert_eq!(m.avail, VALUES[2]);
        }
        None => assert!(false, "Expected to select move"),
      };
    }

    #[test]
    fn select_only_possible_move() {
      let mut almost_sol = SOL.clone();
      // unset one position.
      almost_sol[21] = 0;

      let s = SolutionState::create(&almost_sol).unwrap();
      match s.select_move() {
        Some(m) => {
          assert_eq!(m.pos, 21);
          assert_eq!(m.avail, VALUES[SOL[21] as usize]);
        }
        None => assert!(false, "Should have selected only possible move."),
      }
    }

    #[test]
    fn select_when_no_possible_move() {
      let s = SolutionState::create(&SOL).unwrap();
      assert_eq!(s.select_move(), None, "No move possible.");
    }
  }
}
