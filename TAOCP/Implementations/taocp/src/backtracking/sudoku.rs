// A sudoku solver using basic backtracking.

use std::fmt;
use std::fmt::Write;

// How the entries are represented.
const VALUES: [u16; 9] = [
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

#[cfg(test)]
mod tests {
  use super::*;

  const SOL: [u8; 81] = [
    5, 3, 4, 6, 7, 8, 9, 1, 2, 6, 7, 2, 1, 9, 5, 3, 4, 8, 1, 9, 8, 3, 4, 2, 5, 6, 7, 8, 5, 9, 7, 6,
    1, 4, 2, 3, 4, 2, 6, 8, 5, 3, 7, 9, 1, 7, 1, 3, 9, 2, 4, 8, 5, 6, 9, 6, 1, 5, 3, 7, 2, 8, 4, 2,
    8, 7, 4, 1, 9, 6, 3, 5, 3, 4, 5, 2, 8, 6, 1, 7, 9,
  ];

  #[test]
  fn normal_and_bitencoded_create_agree() {
    let base = SudokuSolution::create(&SOL);
    let encoded_sol: Vec<u16> = SOL.iter().map(|v| VALUES[*v as usize - 1]).collect();
    let bit: SudokuSolution = unsafe { SudokuSolution::create_from_bitencoded(&encoded_sol) };

    assert_eq!(base, bit);
  }

  #[test]
  fn formatter_produces_expected_output() {
    let encoded_sol: Vec<u16> = SOL.iter().map(|v| VALUES[*v as usize - 1]).collect();
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
