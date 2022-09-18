// A sudoku solver using basic backtracking.
//
// If there is more than one solution, this will return an arbitrary one.

use std::fmt;
use std::mem;

#[derive(Debug, PartialEq, Eq)]
pub struct SudokuSolution {
  rows: Vec<Vec<u8>>,
}

impl SudokuSolution {
  fn create(values: &[u8]) -> Self {
    assert_eq!(values.len(), 81);
    let mut result: Vec<Vec<u8>> = Vec::with_capacity(9);
    for row in values.chunks(9) {
      result.push(row.to_vec());
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
pub struct InitialPosition {
  pub row: u8,   // Row number [0, 8]
  pub col: u8,   // Col number [0, 8]
  pub value: u8, // Value [1, 9]
}

impl InitialPosition {
  // Create a vector of initial positions from a 81 element array that is either
  // 0 (indicating an unset value) or gives the value at that position.
  pub fn create_from_values(values: &[u8; 81]) -> Vec<InitialPosition> {
    values
      .iter()
      .enumerate()
      .filter(|(_, &val)| val != 0)
      .map(|(idx, val)| InitialPosition {
        row: (idx / 9) as u8,
        col: (idx % 9) as u8,
        value: *val,
      })
      .collect()
  }

  pub fn create_from_vec(values: &Vec<u8>) -> Vec<InitialPosition> {
    values
      .iter()
      .enumerate()
      .filter(|(_, &val)| val != 0)
      .map(|(idx, val)| InitialPosition {
        row: (idx / 9) as u8,
        col: (idx % 9) as u8,
        value: *val,
      })
      .collect()
  }
}

// Bitwise encoded moves are represented as 1 << val where val is in [1, 9]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
struct Move {
  pos: u8,              // Position [0, 80]
  current_move: u16,    // Bitwise encoded current move
  available_moves: u16, // Bitwise or of all available moves, excluding current.
}

impl Move {
  // Converts the current position into row, column, box form.
  fn translate_position(&self) -> (usize, usize, usize) {
    let r = self.pos as usize / 9;
    let c = self.pos as usize % 9;
    (r, c, 3 * (r / 3) + c / 3)
  }

  // Converts the encoded current_move to the normal value [1, 9]
  fn value(&self) -> u8 {
    self.current_move.trailing_zeros() as u8
  }
}

#[derive(Debug)]
struct SolutionState {
  n: u8,        // Number of levels.  Length of m.
  l: u8,        // Current level.
  m: Vec<Move>, // Moves.  [0, l) are settled, l is under consideration.
  initial_position: Vec<InitialPosition>,
  c_row: [u16; 9], // Available moves per row
  c_col: [u16; 9], // Available moves per col
  c_box: [u16; 9], // Available moves per box
}

#[derive(Debug, Eq, PartialEq)]
struct NextMove {
  idx: usize,
  available_moves: u16,
}

impl SolutionState {
  fn create(mut initial_position: Vec<InitialPosition>) -> Option<Self> {
    if initial_position.len() > 81 {
      return None;
    }
    let mut unused = [true; 81];
    let mut c_row = [0b1111111110u16; 9];
    let mut c_col = c_row.clone();
    let mut c_box = c_row.clone();
    for pos in &initial_position {
      if pos.row > 8 || pos.col > 8 || pos.value < 1 || pos.value > 9 {
        // Invalid initial input.
        return None;
      }
      let b = (3 * (pos.row / 3) + pos.col / 3) as usize;
      let value = 1u16 << pos.value;
      if c_row[pos.row as usize] & c_col[pos.col as usize] & c_box[b] & value == 0 {
        // Conflict, no solution.
        return None;
      }
      c_row[pos.row as usize] &= !value;
      c_col[pos.col as usize] &= !value;
      c_box[b] &= !value;
      unused[9 * pos.row as usize + pos.col as usize] = false;
    }

    let m: Vec<Move>;
    if initial_position.len() == 81 {
      // This is a little tricky -- somebody gave us an already complete
      // solution.  The implementation won't quite work right with that,
      // so we need to artifically leave one of the moves off.
      let final_move = initial_position.pop().unwrap();
      m = vec![Move {
        pos: 9 * final_move.row + final_move.col,
        current_move: 0,
        available_moves: 1u16 << final_move.value,
      }];
    } else {
      m = unused
        .iter()
        .enumerate()
        .filter(|(_, &v)| v)
        .map(|(idx, _)| Move {
          pos: idx as u8,
          current_move: 0,
          available_moves: 0,
        })
        .collect();
    }

    Some(SolutionState {
      n: m.len() as u8,
      l: 0,
      m: m,
      initial_position: initial_position,
      c_row: c_row,
      c_col: c_col,
      c_box: c_box,
    })
  }

  fn to_solution(&self) -> SudokuSolution {
    let mut sol = [0u8; 81];
    for p in &self.initial_position {
      sol[(9 * p.row + p.col) as usize] = p.value;
    }
    for mv in &self.m {
      sol[mv.pos as usize] = mv.value();
    }
    SudokuSolution::create(&sol)
  }

  fn next_move(&self) -> NextMove {
    let (r, c, b) = self.m[self.l as usize].translate_position();
    let mut avail_best = self.c_row[r] & self.c_col[c] & self.c_box[b];
    if avail_best == 0 {
      return NextMove {
        idx: self.l as usize,
        available_moves: 0,
      };
    }
    let mut mrv_best = avail_best.count_ones();
    let mut idx_best = self.l as usize;
    for (idx, mv) in self.m.iter().enumerate().skip(1) {
      let (r, c, b) = mv.translate_position();
      let avail = self.c_row[r] & self.c_col[c] & self.c_box[b];
      if avail == 0 {
        return NextMove {
          idx: idx,
          available_moves: 0,
        };
      }
      let mrv = avail.count_ones();
      if mrv < mrv_best {
        idx_best = idx;
        mrv_best = mrv;
        avail_best = avail;
      }
    }

    NextMove {
      idx: idx_best,
      available_moves: avail_best,
    }
  }
}

impl Iterator for SolutionState {
  type Item = SudokuSolution;

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      // Backtrack from current position.
      while self.m[self.l as usize].available_moves == 0 {
        if self.l == 0 {
          return None;
        }
        let (r, c, b) = self.m[self.l as usize].translate_position();
        self.c_row[r] |= self.m[self.l as usize].current_move;
        self.c_col[c] |= self.m[self.l as usize].current_move;
        self.c_box[b] |= self.m[self.l as usize].current_move;
        self.l -= 1;
      }

      // Chose next move.  We are already guaranteed avail is not 0.
      let avail = self.m[self.l as usize].available_moves as i16;
      let v = (avail & -avail) as u16;
      self.m[self.l as usize].current_move = v;
      self.m[self.l as usize].available_moves &= !v;
      let (r, c, b) = self.m[self.l as usize].translate_position();
      self.c_row[r] &= !v;
      self.c_col[c] &= !v;
      self.c_box[b] &= !v;
      self.l += 1;

      // Are we done?
      if self.l == self.n {
        return Some(self.to_solution());
      }

      // Chose the next move and swap it into place.
      let next_move = self.next_move();
      self.m.swap(self.l as usize, next_move.idx);
      self.m[self.l as usize].available_moves = next_move.available_moves;
    }
  }
}

enum IteratorState {
  DONE,
  NEW(Vec<InitialPosition>),
  READY(SolutionState),
}

pub struct SudokuIterator {
  state: IteratorState,
}

impl SudokuIterator {
  pub fn create(input: Vec<InitialPosition>) -> Self {
    SudokuIterator {
      state: IteratorState::NEW(input),
    }
  }
}

impl Iterator for SudokuIterator {
  type Item = SudokuSolution;

  fn next(&mut self) -> Option<Self::Item> {
    match &mut self.state {
      IteratorState::NEW(initial_position) => {
        if initial_position.len() != 81 {
          self.state = IteratorState::DONE;
          return None;
        }
        // We need to take ownership of the initial position.
        let init_pos = mem::replace(initial_position, Vec::new());
        match SolutionState::create(init_pos) {
          None => {
            self.state = IteratorState::DONE;
            None
          }
          Some(mut solution_state) => {
            let result = solution_state.next();
            match result {
              None => {
                self.state = IteratorState::DONE;
              }
              Some(ref _sol) => {
                self.state = IteratorState::READY(solution_state);
              }
            }
            result
          }
        }
      }
      IteratorState::READY(ref mut solution_state) => {
        solution_state.l -= 1;
        let result = solution_state.next();
        if result.is_none() {
          self.state = IteratorState::DONE;
        }
        result
      }
      IteratorState::DONE => None,
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

  mod move_type {
    use crate::backtracking::sudoku::Move;

    #[test]
    fn positions() {
      let mut mv = Move {
        pos: 0,
        current_move: 0,
        available_moves: 0,
      };
      assert_eq!(mv.translate_position(), (0, 0, 0));
      mv.pos = 2;
      assert_eq!(mv.translate_position(), (0, 2, 0));
      mv.pos = 4;
      assert_eq!(mv.translate_position(), (0, 4, 1));
      mv.pos = 21;
      assert_eq!(mv.translate_position(), (2, 3, 1));
      mv.pos = 50;
      assert_eq!(mv.translate_position(), (5, 5, 4));
      mv.pos = 51;
      assert_eq!(mv.translate_position(), (5, 6, 5));
      mv.pos = 80;
      assert_eq!(mv.translate_position(), (8, 8, 8));
    }
  }

  mod sudoku_solution {
    use super::SOL;
    use crate::backtracking::sudoku::SudokuSolution;
    use std::fmt::Write;

    #[test]
    fn formatter_produces_expected_output() {
      let s = SudokuSolution::create(&SOL);

      let mut buf = String::new();
      write!(&mut buf, "{}", s).ok();

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
    use crate::backtracking::sudoku::{InitialPosition, SolutionState};

    #[test]
    fn invalid_input_value() {
      let mut bad_input = SOL;
      bad_input[10] = 10;

      assert!(SolutionState::create(InitialPosition::create_from_values(&bad_input)).is_none());
    }

    #[test]
    fn conflicting_input_row() {
      let bad_input = vec![0, 1, 1, 2, 3, 4, 5, 6, 7];

      assert!(SolutionState::create(InitialPosition::create_from_vec(&bad_input)).is_none());
    }

    #[test]
    fn select_position_from_almost_full_row() {
      let input = vec![1, 3, 4, 5, 0, 7, 6, 8, 9];
      let initial_position = InitialPosition::create_from_vec(&input);
      let s = SolutionState::create(initial_position).unwrap();

      let next_move = s.next_move();
      assert_eq!(
        next_move.available_moves,
        1u16 << 2,
        "Unexpected available moves"
      );
      assert_eq!(
        s.m[next_move.idx].pos, 4,
        "Unexpected position for next move"
      );
    }

    #[test]
    fn select_only_possible_move() {
      let mut almost_sol = SOL;
      // unset one position.
      almost_sol[21] = 0; // was 3.

      let s = SolutionState::create(InitialPosition::create_from_values(&almost_sol)).unwrap();
      let next_move = s.next_move();
      assert_eq!(
        next_move.available_moves,
        1u16 << 3,
        "Unexpected available moves"
      );
      assert_eq!(
        s.m[next_move.idx].pos, 21,
        "Unexpected position for next move"
      );
    }

    #[test]
    fn select_most_constrained_move_with_single_choice() {
      // The most constrained open space is the second 0 in the first row.
      let mut input = Vec::with_capacity(18);
      input.extend([0, 2, 3, 0, 5, 6, 7, 8, 9]);
      input.extend([0, 7, 8, 1, 2, 3, 4, 5, 0]);

      let initial_position = InitialPosition::create_from_vec(&input);
      let s = SolutionState::create(initial_position).unwrap();

      let next_move = s.next_move();
      assert_eq!(
        next_move.available_moves,
        1u16 << 4,
        "Unexpected available moves"
      );
      assert_eq!(
        s.m[next_move.idx].pos, 3,
        "Unexpected position for next move"
      );
    }

    #[test]
    fn select_most_constrained_move_with_multiple_choices() {
      // Similar to the previous test but with more unset values with
      // multiple valid choices in second row.
      // The best choice should be the second 0 in the first row with
      // available values 4, 6
      let mut input = Vec::with_capacity(18);
      input.extend([0, 2, 3, 0, 5, 0, 7, 8, 9]);
      input.extend([0, 0, 0, 1, 2, 0, 4, 5, 6]);

      let initial_position = InitialPosition::create_from_vec(&input);
      let s = SolutionState::create(initial_position).unwrap();

      let next_move = s.next_move();
      assert_eq!(
        s.m[next_move.idx].pos, 3,
        "Unexpected position for next move"
      );
      assert_eq!(
        next_move.available_moves,
        (1u16 << 4 | 1u16 << 6),
        "Unexpected available moves"
      );
    }

    #[test]
    fn select_when_no_move() {
      // Test what happens when we get into a corner where no move is available.
      // The last element on the second row has no available value.
      let mut input = Vec::with_capacity(27);
      input.extend([1, 2, 3, 4, 5, 6, 7, 8, 9]);
      input.extend([4, 5, 6, 7, 8, 3, 1, 2, 0]);
      input.extend([7, 8, 0, 1, 2, 0, 4, 5, 6]);

      let initial_position = InitialPosition::create_from_vec(&input);
      let s = SolutionState::create(initial_position).unwrap();

      let next_move = s.next_move();
      assert_eq!(next_move.available_moves, 0, "Should be no available moves");
      assert_eq!(
        s.m[next_move.idx].pos, 17,
        "Unexpected position for next move"
      );
    }
  }
}
