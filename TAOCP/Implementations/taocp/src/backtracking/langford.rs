// Finds Langford paris using backtracking.

// TODO: Halve the amount of work by computing mirror.

enum IteratorState {
  NEW,
  READY,
  DONE,
}

pub struct LangfordIterator {
  // Range of values is [1..n]
  n: u8,
  // Current solution array.  The second value of each pair is represented
  // as - the value.
  x: Vec<i8>,
  // Pointer array for active values.
  p: Vec<u8>,
  // Backtracking array.
  y: Vec<u8>,
  // Current level.
  l: u8,
  // Current state of iterator.
  state: IteratorState,
}

impl LangfordIterator {
  pub fn new(n: u8) -> Self {
    assert!(n > 0 && n < 128, "n not in valid range [0, 128)");
    // There can only be solutions for n mod 4 = 0 or 3
    if n & 3 != 0 && n & 3 != 3 {
      LangfordIterator {
        n: n,
        x: Vec::with_capacity(0),
        p: Vec::with_capacity(0),
        y: Vec::with_capacity(0),
        l: 0,
        state: IteratorState::DONE,
      }
    } else {
      let mut p: Vec<u8> = (1..=(n + 1)).collect();
      p[n as usize] = 0;
      LangfordIterator {
        n: n,
        x: vec![0; 2 * (n as usize)],
        p: p,
        y: vec![0; 2 * (n as usize)],
        l: 0,
        state: IteratorState::NEW,
      }
    }
  }

  fn to_solution(&self) -> Vec<u8> {
    self.x.iter().map(|v| v.abs() as u8).collect()
  }
}

impl Iterator for LangfordIterator {
  type Item = Vec<u8>;

  fn next(&mut self) -> Option<Self::Item> {
    // k = p[j] is the next element we are going to try, with k = 0 indicating
    // that we are out of options at this level.
    let (mut j, mut k) = match self.state {
      IteratorState::DONE => return None,
      IteratorState::READY => (0, 0), // This will cause backtrack.
      IteratorState::NEW => (0, self.p[0]),
    };

    let two_n = 2 * self.n;

    loop {
      if k == 0 {
        // Backtrack.
        if self.l == 0 {
          // No more options.
          self.state = IteratorState::DONE;
          return None;
        }
        self.l -= 1;

        // Undo all elements that are second copies.
        while self.x[self.l as usize] < 0 {
          self.l -= 1;
        }

        // Now undo the previous move using y.
        k = self.x[self.l as usize] as u8;
        self.x[self.l as usize] = 0;
        self.x[(self.l + k + 1) as usize] = 0;
        self.p[self.y[self.l as usize] as usize] = k;
        // Move to the next element.
        j = k;
        k = self.p[j as usize];
      } else if (self.l + k + 1) < two_n && (self.x[(self.l + k + 1) as usize] == 0) {
        // Take step.
        self.x[self.l as usize] = k as i8;
        self.x[(self.l + k + 1) as usize] = -(k as i8);
        // Set the undo.
        self.y[self.l as usize] = j;
        // Remove p[j]
        self.p[j as usize] = self.p[k as usize];
        self.l += 1;

        // Advance over already already set positions.
        while self.l < two_n && self.x[self.l as usize] != 0 {
          self.l += 1;
        }

        // Check to see if we are done.
        if self.l == two_n {
          self.state = IteratorState::READY;
          return Some(self.to_solution());
        }

        j = 0;
        k = self.p[0];
      } else {
        // Try the next j, k pair.
        j = k;
        k = self.p[j as usize];
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn count_no_solutions() {
    assert_eq!(LangfordIterator::new(1).count(), 0);
    assert_eq!(LangfordIterator::new(2).count(), 0);
    assert_eq!(LangfordIterator::new(5).count(), 0);
    assert_eq!(LangfordIterator::new(6).count(), 0);
    assert_eq!(LangfordIterator::new(9).count(), 0);
    assert_eq!(LangfordIterator::new(10).count(), 0);
  }

  #[test]
  fn count_with_solutions() {
    assert_eq!(LangfordIterator::new(3).count(), 2);
    assert_eq!(LangfordIterator::new(4).count(), 2);
    assert_eq!(LangfordIterator::new(7).count(), 52);
    assert_eq!(LangfordIterator::new(8).count(), 300);
  }

  #[test]
  fn count_large_number_solutions() {
    assert_eq!(LangfordIterator::new(11).count(), 35584);
  }

  #[test]
  fn expected_solutions_n3() {
    assert_eq!(
      LangfordIterator::new(3).collect::<Vec<_>>(),
      vec![vec![2, 3, 1, 2, 1, 3], vec![3, 1, 2, 1, 3, 2]]
    );
  }

  #[test]
  fn expected_solutions_n4() {
    assert_eq!(
      LangfordIterator::new(4).collect::<Vec<_>>(),
      vec![vec![2, 3, 4, 2, 1, 3, 1, 4], vec![4, 1, 3, 1, 2, 4, 3, 2]]
    );
  }
}
