use std::fmt;

pub struct Solution {
  // The number of queens.
  n: u8,
  // For each of the n rows, the colum number [0, n)
  rows: Vec<u8>,
}

impl Clone for Solution {
  fn clone(&self) -> Solution { 
    Solution{ n: self.n, rows: self.rows.clone() } 
  }
}

impl fmt::Display for Solution {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut board = vec![vec![false; self.n as usize]; self.n as usize];

    for (i, col) in self.rows.iter().enumerate() {
      board[i as usize][* col as usize] = true;
    }

    let mut output = String::from("");
    for row in board {
      for val in row {
        if val {
          output += "Q ";
        } else {
          output += ". ";
        }
      }
      output += "\n";
    }
    write!(f, "{}", output)
  }
}

// An object which visits N Queens solutions.
pub trait Visitor {
  // Visits a single solution.  Returns true if further solutions should
  // be visited, otherwise false.
  fn visit(&mut self, s: &Solution) -> bool;
}

// A visitor which counts solutions.
pub struct CountingVisitor {
  pub n_solutions: u64
}

impl CountingVisitor {
  fn new() -> CountingVisitor {
    CountingVisitor { n_solutions: 0 }
  }
}

impl Visitor for CountingVisitor {
  fn visit(&mut self, _s: &Solution) -> bool {
    self.n_solutions += 1;       
    true
  }
}

// A visitor which records solutions.
pub struct RecordingVisitor {
  solutions: Vec<Solution>
}

impl Visitor for RecordingVisitor {
  fn visit(&mut self, s: &Solution) -> bool {
    self.solutions.push(s.clone());
    true
  }
}

pub trait NQueensSolver {
  fn visit(&mut self, v: &mut Visitor);
}

pub struct BitwiseNQueensSolver {
  n: usize,
  nm1: usize, 
  a: u64,
  b: u64, 
  c: u64,
  s: Solution,
  done: bool
}

impl BitwiseNQueensSolver {
  fn new(n: u8) -> BitwiseNQueensSolver {
    BitwiseNQueensSolver {
      a: 0,
      b: 0,
      c: 0,
      n: n as usize,
      nm1: (n - 1) as usize,
      done: false,
      s: Solution { n : n, rows: vec![0; n as usize] }
    }
  }

  fn clear(&mut self) -> () {
    self.a = 0;
    self.b = 0;
    self.c = 0;
    self.done = false;
    self.s = Solution { n : self.n as u8, rows: vec![0; self.n] }
  }

  fn visit_levels(&mut self, l: usize, v: &mut Visitor) {
    if self.done {
      return;
    }
    if l > self.nm1 {
      if !v.visit(&self.s) {
        self.done = true;
      }
      return
    }

    for xl in 0..self.n {
      let xla : u64 = 1u64 << xl;
      let xlb : u64 = xla << l;
      let xlc : u64 = 1u64 << (xl + self.nm1 - l);
      if (self.a & xla == 0) 
          && (self.b & xlb == 0) 
          && (self.c & xlc == 0) {

          // Valid candidate x_l
          self.s.rows[l] = xl as u8;
          self.a |= xla;
          self.b |= xlb;
          self.c |= xlc;

          self.visit_levels(l + 1, v);

          // Undo. 
          self.a &= !xla;
          self.b &= !xlb;
          self.c &= !xlc;
      }
    }
  }
}

impl NQueensSolver for BitwiseNQueensSolver {
  fn visit(&mut self, v: &mut Visitor) {
    self.clear();
    self.visit_levels(0, v);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn formatter_works() {
    let sol = Solution { n: 4, rows: vec![2, 0, 3, 1]};
    assert_eq!(format!("{}", sol), 
      ". . Q . \nQ . . . \n. . . Q \n. Q . . \n");
  }

  #[test]
  fn bitwise_nqueens_n4_count() {
    let mut cv = CountingVisitor::new();
    BitwiseNQueensSolver::new(4).visit(&mut cv);
    assert_eq!(cv.n_solutions, 2);
  }
}
