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

  fn box_index(&self) -> usize {
    (3 * (self.row / 3) + self.col / 3) as usize
  }
}

// Row, column, box position.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
struct Square {
  row: u8,
  col: u8,
  r#box: u8,
}

impl Square {
  fn create(row: u8, col: u8) -> Self {
    Square {
      row: row,
      col: col,
      r#box: 3 * (row / 3) + col / 3,
    }
  }

  fn position(&self) -> usize {
    (9 * self.row + self.col) as usize
  }
}

// Bitwise encoded moves are represented as 1 << val where val is in [1, 9]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
struct Move {
  current_move: u16,    // Bitwise encoded current move
  available_moves: u16, // Bitwise or of all available moves, excluding current.
  square: Square,       // The position in the puzzle of this move.
}

impl Move {
  // Converts the encoded current_move to the normal value [1, 9]
  fn value(&self) -> u8 {
    self.current_move.trailing_zeros() as u8
  }
}

#[derive(Debug)]
struct SolutionState {
  l: usize,        // Current level.
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
      let b = pos.box_index();
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
      let available_move = 1u16 << final_move.value;
      let square = Square::create(final_move.row, final_move.col);
      c_row[square.row as usize] = available_move;
      c_col[square.col as usize] = available_move;
      c_box[square.r#box as usize] = available_move;
      m = vec![Move {
        square: square,
        current_move: 0,
        available_moves: available_move,
      }];
    } else {
      m = unused
        .iter()
        .enumerate()
        .filter(|(_, &v)| v)
        .map(|(idx, _)| {
          Move {
            square: Square::create(idx as u8 / 9, idx as u8 % 9),
            current_move: 0,
            available_moves: 0,
          }
        })
        .collect();
    }

    Some(SolutionState {
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
      let v = mv.value();
      sol[mv.square.position()] = if v == 16 { 0 } else { v };
    }
    SudokuSolution::create(&sol)
  }

  // Applies the move in m[l] to the state.
  // Assumes that self.l is in the range [0, n) and that m[l].available_moves
  // is not zero (that is, there is an available move).
  #[inline(always)]
  unsafe fn apply_next_move(&mut self) {
    // Assumed non-zero.
    let avail = self.m[self.l].available_moves as i16;

    let v = (avail & -avail) as u16;
    self.m[self.l].current_move = v;
    self.m[self.l].available_moves &= !v;

    let sq = self.m[self.l].square;
    self.c_row[sq.row as usize] &= !v;
    self.c_col[sq.col as usize] &= !v;
    self.c_box[sq.r#box as usize] &= !v;
  }

  // Undoes the move in position m[l].  Assumes self.l is in the range [0, n)
  #[inline(always)]
  unsafe fn undo_current_move(&mut self) {
    let sq = self.m[self.l].square;
    self.c_row[sq.row as usize] |= self.m[self.l].current_move;
    self.c_col[sq.col as usize] |= self.m[self.l].current_move;
    self.c_box[sq.r#box as usize] |= self.m[self.l].current_move;
  }

  // Chooses the next move and swaps it into place as m[l].
  // Assumes that self.l is in the range [0, n)
  #[inline(always)]
  unsafe fn choose_next_move(&mut self) -> () {
    let next_move = self.suggest_next_move();
    self.m.swap(self.l, next_move.idx);
    self.m[self.l].current_move = 0;
    self.m[self.l].available_moves = next_move.available_moves;
  }

  // Returns the next move that should be made.  Assumes that self.l is in
  // the range [0, n)
  #[inline(always)]
  unsafe fn suggest_next_move(&self) -> NextMove {
    let sq = self.m[self.l].square;
    let mut avail_best = self.c_row[sq.row as usize]
      & self.c_col[sq.col as usize]
      & self.c_box[sq.r#box as usize];
    if avail_best == 0 {
      return NextMove {
        idx: self.l,
        available_moves: 0,
      };
    }
    let mut mrv_best = avail_best.count_ones();
    let mut idx_best = self.l;
    for (idx, mv) in self.m.iter().enumerate().skip(self.l + 1) {
      let avail =
        self.c_row[mv.square.row as usize] 
          & self.c_col[mv.square.col as usize] 
          & self.c_box[mv.square.r#box as usize];
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
      while self.m[self.l].available_moves == 0 {
        if self.l == 0 {
          return None;
        }
        unsafe {
          self.undo_current_move();
        }
        self.l -= 1;
      }

      // Undo the current move, then apply the next one.
      // Apply the move in position m[l] and advance l.
      unsafe {
        self.undo_current_move();
        self.apply_next_move();
      }
      self.l += 1;

      // Are we done?
      if self.l == self.m.len() {
        return Some(self.to_solution());
      }

      // Chose the next move and put it in m[l].
      unsafe {
        self.choose_next_move();
      }
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
        // We need to take ownership of the initial position.
        let init_pos = mem::replace(initial_position, Vec::new());
        match SolutionState::create(init_pos) {
          None => {
            self.state = IteratorState::DONE;
            None
          }
          Some(mut solution_state) => {
            unsafe {
              solution_state.choose_next_move();
            }
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
    use super::{PARTIAL, SOL};
    use crate::backtracking::sudoku::{InitialPosition, NextMove, SolutionState, Square};

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

      let next_move = unsafe { s.suggest_next_move() };
      assert_eq!(
        next_move.available_moves,
        1u16 << 2,
        "Unexpected available moves"
      );
      assert_eq!(
        s.m[next_move.idx].square,
        Square { row: 0, col: 4, r#box: 1},
        "Unexpected position for next move"
      );
    }

    #[test]
    fn select_only_possible_move() {
      let mut almost_sol = SOL;
      // unset one position.
      almost_sol[21] = 0; // was 3.

      let s = SolutionState::create(InitialPosition::create_from_values(&almost_sol)).unwrap();
      let next_move = unsafe { s.suggest_next_move() };
      assert_eq!(
        next_move.available_moves,
        1u16 << 3,
        "Unexpected available moves"
      );
      assert_eq!(
        s.m[next_move.idx].square,
        Square { row: 2, col: 3, r#box: 1},
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

      let next_move = unsafe { s.suggest_next_move() };
      assert_eq!(
        next_move.available_moves,
        1u16 << 4,
        "Unexpected available moves"
      );
      assert_eq!(
        s.m[next_move.idx].square,
        Square { row: 0, col: 3, r#box: 1 },
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

      let next_move = unsafe { s.suggest_next_move() };
      assert_eq!(
        s.m[next_move.idx].square,
        Square { row: 0, col: 3, r#box: 1 },
        "Unexpected position for next move"
      );
      assert_eq!(
        next_move.available_moves,
        (1u16 << 4 | 1u16 << 6),
        "Unexpected available moves"
      );
    }

    #[test]
    fn select_next_move_from_partial_puzzle() {
      // Try selecting from a real puzzle.
      let initial_position = InitialPosition::create_from_values(&PARTIAL);
      let s = SolutionState::create(initial_position).unwrap();

      let next_move = unsafe { s.suggest_next_move() };
      assert_eq!(
        s.m[next_move.idx].square,
        Square { row: 0, col: 0, r#box: 0 },
        "Unexpected position for next move"
      );
      assert_eq!(
        next_move.available_moves,
        (1u16 << 4 | 1u16 << 5),
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

      let next_move = unsafe { s.suggest_next_move() };
      assert_eq!(next_move.available_moves, 0, "Should be no available moves");
      assert_eq!(
        s.m[next_move.idx].square,
        Square { row: 1, col: 8, r#box: 2 },
        "Unexpected position for next move"
      );
    }

    #[test]
    fn already_solved_puzzle_should_force_single_move() {
      let initial_position = InitialPosition::create_from_values(&SOL);
      match SolutionState::create(initial_position) {
        None => panic!("Should have been able to initialize from completed solution"),
        Some(state) => {
          assert_eq!(state.m.len(), 1, "Should be single move in m");
          assert_eq!(
            unsafe { state.suggest_next_move() },
            NextMove {
              idx: 0,
              available_moves: 1u16 << SOL[state.m[0].square.position()]
            }
          );
        }
      }
    }
  }

  mod iterator {
    use super::{PARTIAL, SOL};
    use crate::backtracking::sudoku::{
      InitialPosition, IteratorState, SudokuIterator, SudokuSolution,
    };

    #[test]
    fn solves_already_solved_puzzle() {
      let initial_position = InitialPosition::create_from_values(&SOL);
      let mut iterator = SudokuIterator::create(initial_position);
      assert!(matches!(iterator.state, IteratorState::NEW(_)));

      match iterator.next() {
        None => panic!("Should have found solution"),
        Some(solution) => {
          let expected_solution = SudokuSolution::create(&SOL);
          assert_eq!(solution, expected_solution, "Did not get expected solution");
        }
      }

      assert_eq!(
        iterator.next(),
        None,
        "DONE iterator should produce no more solutions"
      );
      assert!(
        matches!(iterator.state, IteratorState::DONE),
        "Iterator should be done after discovering there are no more solutions"
      );
    }

    #[test]
    fn solves_using_only_forced_moves() {
      // Take the last row of the full solution and remove all the values
      // on the bottom row.  As a result, all moves will be forced.
      let mut puzzle = SOL;
      for i in 72..81 {
        puzzle[i] = 0;
      }
      let initial_position = InitialPosition::create_from_values(&puzzle);
      let mut iterator = SudokuIterator::create(initial_position);
      assert!(
        matches!(iterator.state, IteratorState::NEW(_)),
        "Iterator not in initial state"
      );

      match iterator.next() {
        None => panic!("Should have found solution"),
        Some(solution) => {
          let expected_solution = SudokuSolution::create(&SOL);
          assert_eq!(solution, expected_solution, "Did not get expected solution");
        }
      }

      assert_eq!(
        iterator.next(),
        None,
        "DONE iterator should produce no more solutions"
      );
      assert!(
        matches!(iterator.state, IteratorState::DONE),
        "Iterator should be done after discovering there are no more solutions"
      );
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

      let initial_position = InitialPosition::create_from_values(&PARTIAL);
      let mut iterator = SudokuIterator::create(initial_position);
      assert!(matches!(iterator.state, IteratorState::NEW(_)));

      match iterator.next() {
        None => panic!("Should have found solution"),
        Some(solution) => {
          let expected_solution = SudokuSolution::create(&expected_solution);
          assert_eq!(solution, expected_solution, "Did not get expected solution");
        }
      }

      assert_eq!(
        iterator.next(),
        None,
        "DONE iterator should produce no more solutions"
      );
      assert!(
        matches!(iterator.state, IteratorState::DONE),
        "Iterator should be done after discovering there are no more solutions"
      );
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

      let initial_position = InitialPosition::create_from_values(&problem);
      let mut iterator = SudokuIterator::create(initial_position);
      assert!(matches!(iterator.state, IteratorState::NEW(_)));

      match iterator.next() {
        None => panic!("Should have found solution"),
        Some(solution) => {
          let expected_solution = SudokuSolution::create(&expected_solution);
          assert_eq!(solution, expected_solution, "Did not get expected solution");
        }
      }

      assert_eq!(
        iterator.next(),
        None,
        "DONE iterator should produce no more solutions"
      );
      assert!(
        matches!(iterator.state, IteratorState::DONE),
        "Iterator should be done after discovering there are no more solutions"
      );
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

      let initial_position = InitialPosition::create_from_values(&problem);
      let mut iterator = SudokuIterator::create(initial_position);
      assert!(matches!(iterator.state, IteratorState::NEW(_)));

      match iterator.next() {
        None => panic!("Should have found solution"),
        Some(solution) => {
          let expected_solution = SudokuSolution::create(&expected_solution);
          assert_eq!(solution, expected_solution, "Did not get expected solution");
        }
      }

      assert_eq!(
        iterator.next(),
        None,
        "DONE iterator should produce no more solutions"
      );
      assert!(
        matches!(iterator.state, IteratorState::DONE),
        "Iterator should be done after discovering there are no more solutions"
      );
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

      let initial_position = InitialPosition::create_from_values(&problem);
      let mut iterator = SudokuIterator::create(initial_position);
      assert!(matches!(iterator.state, IteratorState::NEW(_)));

      match iterator.next() {
        None => panic!("Should have found solution"),
        Some(solution) => {
          let expected_solution = SudokuSolution::create(&expected_solution);
          assert_eq!(solution, expected_solution, "Did not get expected solution");
        }
      }

      assert_eq!(
        iterator.next(),
        None,
        "DONE iterator should produce no more solutions"
      );
      assert!(
        matches!(iterator.state, IteratorState::DONE),
        "Iterator should be done after discovering there are no more solutions"
      );
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

      let initial_position = InitialPosition::create_from_values(&problem);
      let iterator = SudokuIterator::create(initial_position);
      assert!(matches!(iterator.state, IteratorState::NEW(_)));

      let sols: Vec<SudokuSolution> = iterator.collect();
      assert_eq!(sols.len(), 2);

      assert_eq!(sols[0], SudokuSolution::create(&expected_solution1));
      assert_eq!(sols[1], SudokuSolution::create(&expected_solution2));
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

      let initial_position = InitialPosition::create_from_values(&problem);
      let iterator = SudokuIterator::create(initial_position);

      let solutions: Vec<SudokuSolution> = iterator.collect();
      assert_eq!(solutions.len(), 1, "Solution should be unique");
      assert_eq!(solutions[0], SudokuSolution::create(&expected_solution));
    }
  }
}
