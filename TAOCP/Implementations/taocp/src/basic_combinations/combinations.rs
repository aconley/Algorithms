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
pub fn basic_generate(n: u32, t: u32, v: &mut dyn Visitor) {
  assert!(n >= t, "n must be >= t");
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

// Knuth Algorithm T, TAOCP 4A 7.2.1.3
// Generates all t-combinations of the n numbers [0, n), calling
// visitor.visit for each one.
//
// Like Algorithm L but faster.
pub fn combinations(n: u32, t: u32, v: &mut dyn Visitor) {
  assert!(n >= t, "n must be >= t");
  if n == 0 || t == 0 {
    return;
  }
  if n == t {
    // Algorithm t assumes t < n.
    v.visit(&(0..t).collect::<Vec<u32>>());
    return;
  }

  let ts = t as usize;

  // We work with a 1 indexed array as in Knuth's specification,
  // then slice for visiting.

  // L1: Initialize
  let mut c = Vec::with_capacity(ts + 2);
  c.push(0); // Ignored
  for i in 0..t {
    c.push(i);
  }
  c.push(n);
  c.push(0);
  let mut j = ts;

  loop {
    // L2: Visit, terminate early if needed.
    if !v.visit(&c[1..=ts]) {
      return;
    }

    if j > 0 {
      // T6: increase c_j
      c[j] = j as u32;
      j -= 1;
    } else if c[1] + 1 < c[2] {  
      // T3: Easy case?
      c[1] += 1;
    } else {
      // T4: find j.
      c[1] = 0;
      j = 2;
      let mut x = c[2] + 1;
      while x == c[j + 1] {
        j += 1;
        c[j - 1] = (j - 2) as u32;
        x = c[j] + 1;
      }

      // T5: done?
      if j > ts {
        return;
      }

      // T6: increase cj
      c[j] = x;
      j -= 1;
    }
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

  #[test]
  fn combinations_count() {
    test_counts(&combinations);
  }

  #[test]
  fn combinations_visit() {
    test_visit(&combinations)
  }

  fn test_counts(f: &dyn Fn(u32, u32, &mut dyn Visitor)) {
    // 3 choose 3
    assert_eq!(count(f, 3, 3), 1);

    // 3 choose 2
    assert_eq!(count(f, 3, 2), 3);

    // 5 choose 2
    assert_eq!(count(f, 5, 2), 10);

    // 6 choose 3
    assert_eq!(count(f, 6, 3), 20);

    // 10 choose 4
    assert_eq!(count(f, 10, 4), 210);
  }

  fn test_visit(f: &dyn Fn(u32, u32, &mut dyn Visitor)) {

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

  fn count(f: &dyn Fn(u32, u32, &mut dyn Visitor), n: u32, t: u32) -> u64 {
    let mut cv = CountingVisitor::new();
    f(n, t, &mut cv);
    cv.n_solutions
  }
}