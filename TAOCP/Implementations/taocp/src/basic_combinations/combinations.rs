// Routines to generate combinations -- n things taken t at a time.

pub trait Visitor {
  // Visits a single combination.  Returns true if further solutions should
  // be visited, otherwise false.
  fn visit(&mut self, combination: &[u32]) -> bool;
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
  fn visit(&mut self, _: &[u32]) -> bool {
    self.n_solutions += 1;       
    true
  }
}

// A visitor which records solutions.
pub struct RecordingVisitor {
  solutions: Vec<Vec<u32>>
}

impl RecordingVisitor {
  pub fn new() -> RecordingVisitor {
    RecordingVisitor { solutions: Vec::new() }
  }

  pub fn get_n_solutions(&self) -> u64 {
    self.solutions.len() as u64
  }

  pub fn get_solution(&self, idx: usize) -> &[u32] {
    &self.solutions[idx][..]
  }
}

impl Visitor for RecordingVisitor {
  fn visit(&mut self, c: &[u32]) -> bool {
    self.solutions.push(c.to_vec());
    true
  }
}

// Knuth Algorithm L, TAOCP 4A 7.2.1.3
// Generates all t-combinations of the n numbers [0, n), calling
// visitor.visit for each one.
pub fn basic_generate(n: u32, t: u32, v: &mut Visitor) {
  if n == 0 || t == 0 {
    return;
  }

  let ts = t as usize;

  // L1: Initialize
  let mut c = Vec::with_capacity(ts + 2);
  for i in 0..ts {
    c.push(i as u32);
  }
  c.push(n);
  c.push(0);

  loop {
    // L2: Visit, terminate early if needed.
    if !v.visit(&c[0..ts]) {
      return;
    }

    // L3: Find c[j] to increase.
    let mut j = 0;
    while c[j] + 1 == c[j + 1] {
      c[j] = j as u32;
      j += 1;
    }

    // L4: Terminate.
    if j >= ts {
      return;
    }

    // L5: Increase c[j]
    c[j] += 1;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn basic_count() {
    test_counts(&basic_generate);
  }

  #[test]
  fn basic_visit() {
    test_visit(&basic_generate)
  }

  fn test_counts(f: &Fn(u32, u32, &mut Visitor)) {
    // 3 choose 3
    assert_eq!(count(f, 3, 3), 1);

    // 5 choose 2
    assert_eq!(count(f, 5, 2), 10);

    // 6 choose 3
    assert_eq!(count(f, 6, 3), 20);

    // 10 choose 4
    assert_eq!(count(f, 10, 4), 210);
  }

  fn test_visit(f: &Fn(u32, u32, &mut Visitor)) {

    // 4 choose 4
    let mut v = RecordingVisitor::new();
    f(4, 4, &mut v);
    assert_eq!(v.get_n_solutions(), 1);
    assert_eq!(v.get_solution(0), [0, 1, 2, 3]);

    // 6 choose 3
    v = RecordingVisitor::new();
    f(6, 3, &mut v);
    assert_eq!(v.get_n_solutions(), 20);
    assert_eq!(v.get_solution(0), [0, 1, 2]);
    assert_eq!(v.get_solution(1), [0, 1, 3]);
  }

  fn count(f: &Fn(u32, u32, &mut Visitor), n: u32, t: u32) -> u64 {
    let mut cv = CountingVisitor::new();
    f(n, t, &mut cv);
    cv.n_solutions
  }
}