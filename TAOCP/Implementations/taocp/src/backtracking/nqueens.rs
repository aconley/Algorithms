use std::fmt;

pub struct Solution {
  // The number of queens.
  pub n: u8,
  // For each of the n rows, the colum number [0, n)
  pub rows: Vec<u8>,
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
  pub fn new() -> CountingVisitor {
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

impl RecordingVisitor {
  pub fn new() -> RecordingVisitor {
    RecordingVisitor { solutions: Vec::new() }
  }

  pub fn get_n_solutions(&self) -> u64 {
    self.solutions.len() as u64
  }

  pub fn get_solution(&self, idx: usize) -> &Solution {
    &self.solutions[idx]
  }
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
  pub fn new(n: u8) -> BitwiseNQueensSolver {
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

// NQueens solver using Walkers method.
pub struct WalkerNQueensSolver {
  n: usize,
  a: Vec<u32>,
  b: Vec<u32>,
  c: Vec<u32>,
  s: Vec<u32>,
  mu: u32,
  done: bool,
  solution: Solution
}

impl WalkerNQueensSolver {
  pub fn new(n: u8) -> WalkerNQueensSolver {
    let np1 = (n + 1) as usize;
    let mu = if n == 32 { !0u32 } else { (1u32 << n) - 1 };
    WalkerNQueensSolver {
      n: n as usize,
      a: vec![0; np1],
      b: vec![0; np1],
      c: vec![0; np1],
      s: vec![0; np1],
      mu: mu,
      done: false,
      solution: Solution { n : n, rows: vec![0; n as usize] }
    }
  }

  fn clear(&mut self) -> () {
    self.done = false;
    for v in &mut self.a { *v = 0; };
    for v in &mut self.b { *v = 0; };
    for v in &mut self.c { *v = 0; };
    for v in &mut self.s { *v = 0; };
  }

  fn least_set_bit(v: u32) -> u8 {
    return v.trailing_zeros() as u8;
  }

  fn fill_solution(&mut self) -> () {
    for i in 0..self.n {
      self.solution.rows[i] = 
        WalkerNQueensSolver::least_set_bit(self.a[i+1] - self.a[i]);
    }
  }

  fn visit_levels(&mut self, l: usize, v: &mut Visitor) {
    if self.done {
      return;
    }
    if l > self.n {
      self.fill_solution();
      if !v.visit(&self.solution) {
        self.done = true;
      }
      return
    }

    self.s[l] = self.mu & !self.a[l-1] & !self.b[l-1] & !self.c[l-1];
    while self.s[l] != 0 {
      let t = self.s[l] & (!self.s[l] + 1);
      self.a[l] = self.a[l - 1] + t;
      self.b[l] = (self.b[l - 1] + t) >> 1;
      self.c[l] = ((self.c[l - 1] + t) << 1) & self.mu;
      self.s[l] -= t;
      self.visit_levels(l + 1, v);
    }    
  }
}

impl NQueensSolver for WalkerNQueensSolver {
  fn visit(&mut self, v: &mut Visitor) {
    self.clear();
    self.visit_levels(1, v);
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
  fn bitwise_nqueens_count_n1() {
    assert_eq!(count_n(&mut BitwiseNQueensSolver::new(1)), 1);
  }

  #[test]
  fn bitwise_nqueens_count_n2() {
    assert_eq!(count_n(&mut BitwiseNQueensSolver::new(2)), 0);
  }

  #[test]
  fn bitwise_nqueens_count_n4() {
    assert_eq!(count_n(&mut BitwiseNQueensSolver::new(4)), 2);
  }

  #[test]
  fn bitwise_nqueens_count_n5() {
    assert_eq!(count_n(&mut BitwiseNQueensSolver::new(5)), 10);
  }

  #[test]
  fn bitwise_nqueens_count_n8() {
    assert_eq!(count_n(&mut BitwiseNQueensSolver::new(8)), 92);
  }

  #[test]
  fn bitwise_nqueens_n4_solutions() {
    let mut v = RecordingVisitor::new();
    BitwiseNQueensSolver::new(4).visit(&mut v);
    assert_eq!(v.get_n_solutions(), 2);
    assert_eq!(v.get_solution(0).rows, [1, 3, 0, 2]);
    assert_eq!(v.get_solution(1).rows, [2, 0, 3, 1]);
  }

  #[test]
  fn walker_nqueens_count_n1() {
    assert_eq!(count_n(&mut WalkerNQueensSolver::new(1)), 1);
  }

  #[test]
  fn walker_nqueens_count_n2() {
    assert_eq!(count_n(&mut WalkerNQueensSolver::new(2)), 0);
  }

  #[test]
  fn walker_nqueens_count_n4() {
    assert_eq!(count_n(&mut WalkerNQueensSolver::new(4)), 2);
  }

  #[test]
  fn walker_nqueens_count_n5() {
    assert_eq!(count_n(&mut WalkerNQueensSolver::new(5)), 10);
  }

  #[test]
  fn walker_nqueens_count_n8() {
    assert_eq!(count_n(&mut WalkerNQueensSolver::new(8)), 92);
  }

  #[test]
  fn walker_nqueens_n4_solutions() {
    let mut v = RecordingVisitor::new();
    WalkerNQueensSolver::new(4).visit(&mut v);
    assert_eq!(v.get_n_solutions(), 2);
    assert_eq!(v.get_solution(0).rows, [1, 3, 0, 2]);
    assert_eq!(v.get_solution(1).rows, [2, 0, 3, 1]);
  }

  fn count_n(nq: &mut NQueensSolver) -> u64 {
    let mut cv = CountingVisitor::new();
    nq.visit(&mut cv);
    cv.n_solutions
  }
}
