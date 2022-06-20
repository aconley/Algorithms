// Finds Langford paris using backtracking.

// TODO: Halve the amount of work by computing mirror.

#[derive(PartialEq, Eq, Debug)]
enum IteratorState {
  New,
  Ready,
  Done,
}

#[derive(Debug)]
pub struct LangfordIterator {
  // Range of values is [1..n]
  n: u8,
  // Current solution array.  The second value of each pair is represented
  // as - the value.
  x: Vec<i8>,
  // Pointer array for active values.
  p: Vec<u8>,
  // Values that have not been used; a[k-1] is [k has not been used]
  unused: Vec<bool>,
  // Backtracking array.
  y: Vec<u8>,
  // Current level.
  l: u8,
  // Current state of iterator.
  state: IteratorState,
}

impl LangfordIterator {
  pub fn new(n: u8) -> Self {
    assert!(n > 0 && n <= 32, "n not in valid range (0, 32]");
    // There can only be solutions for n mod 4 = 0 or 3
    if n & 3 != 0 && n & 3 != 3 {
      LangfordIterator {
        n,
        x: Vec::with_capacity(0),
        p: Vec::with_capacity(0),
        unused: Vec::with_capacity(0),
        y: Vec::with_capacity(0),
        l: 0,
        state: IteratorState::Done,
      }
    } else {
      let mut p: Vec<u8> = (1..=(n + 1)).collect();
      p[n as usize] = 0;
      LangfordIterator {
        n,
        x: vec![0; 2 * (n as usize)],
        p,
        unused: vec![true; n as usize],
        y: vec![0; 2 * (n as usize)],
        l: 0,
        state: IteratorState::New,
      }
    }
  }

  fn to_solution(&self) -> Vec<u8> {
    self.x.iter().map(|v| v.abs() as u8).collect()
  }

  fn set_used(&mut self, k: u8) {
    self.x[self.l as usize] = k as i8;
    self.x[(self.l + k + 1) as usize] = -(k as i8);
    self.unused[k as usize - 1] = false;
  }

  fn set_unused(&mut self, k: u8) {
    self.x[self.l as usize] = 0;
    self.x[(self.l + k + 1) as usize] = 0;
    self.p[self.y[self.l as usize] as usize] = k;
    self.unused[k as usize - 1] = true;
  }

  // Takes a forward step.
  fn take_step(&mut self, j: u8, k: u8) {
    self.set_used(k);

    // Set the undo.
    self.y[self.l as usize] = j;
    // Remove p[j]
    self.p[j as usize] = self.p[k as usize];
    self.l += 1;

    // Advance over already already set positions.
    while self.l < 2 * self.n && self.x[self.l as usize] != 0 {
      self.l += 1;
    }
  }

  // Backtracks, returning the next element to try
  fn backtrack(&mut self) -> u8 {
    if self.l == 0 {
      // No more options.
      self.state = IteratorState::Done;
      return 0;
    }
    self.l -= 1;

    // Undo all elements that are second copies.
    while self.x[self.l as usize] < 0 {
      self.l -= 1;
    }

    // Now undo the previous move using y.
    let k = self.x[self.l as usize] as u8; // Get next value.
    self.p[self.y[self.l as usize] as usize] = k;
    self.set_unused(k);
    k
  }
}

impl Iterator for LangfordIterator {
  type Item = Vec<u8>;

  fn next(&mut self) -> Option<Self::Item> {
    // k = p[j] is the next element we are going to try, with k = 0 indicating
    // that we are out of options at this level.
    let mut j = match self.state {
      IteratorState::Done => return None,
      IteratorState::Ready => self.n, // This will cause backtrack since p[n] == 0.
      IteratorState::New => 0,
    };

    let two_n = 2 * self.n;

    loop {
      // We are always trying element j in the active list with p[j] = k.
      let mut k = self.p[j as usize];
      if k == 0 {
        // There are either no more values to try at this level, or the value
        // of k doesn't fit.
        j = self.backtrack();
        if self.state == IteratorState::Done {
          return None;
        }
        continue;
      }

      if self.l + k + 1 >= two_n {
        // k won't fit, so advance.  Even if we force below, we will only
        // increase k.
        j = k;
        continue;
      }

      if self.l >= self.n - 2 && self.unused[(two_n - self.l - 3) as usize] {
        if self.x[two_n as usize - 1] != 0 {
          // Last slot already full, we have to backtrack.
          j = self.backtrack();
          if self.state == IteratorState::Done {
            return None;
          }
          continue;
        }

        // The value of k is forced or else there won't be room to include k
        // in the array.
        let forced_k = two_n - self.l - 2;
        // Advance until we find the forced value.  We know it's present
        // because it wasn't set in a in the test above.
        while k != forced_k {
          j = k;
          k = self.p[j as usize];
        }
      }

      if self.x[(self.l + k + 1) as usize] == 0 {
        // Try k.
        self.take_step(j, k);

        // Check if we are done.
        if self.l == two_n {
          self.state = IteratorState::Ready;
          return Some(self.to_solution());
        }

        // Go back to the start of the available elements.
        j = 0;
      } else {
        // Try the next j, k pair.
        j = k;
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::collections::HashSet;

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
  fn count_small_with_solutions() {
    assert_eq!(LangfordIterator::new(3).count(), 2);
    assert_eq!(LangfordIterator::new(4).count(), 2);
  }

  #[test]
  fn count_medium_with_solutions() {
    assert_eq!(LangfordIterator::new(7).count(), 52);
    assert_eq!(LangfordIterator::new(8).count(), 300);
  }

  #[test]
  fn count_large_number_solutions() {
    assert_eq!(LangfordIterator::new(11).count(), 35584);
    assert_eq!(LangfordIterator::new(12).count(), 216288);
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

  #[test]
  fn number_unique_solutions_medium() {
    assert_eq!(LangfordIterator::new(7).collect::<HashSet<_>>().len(), 52);
    assert_eq!(LangfordIterator::new(8).collect::<HashSet<_>>().len(), 300);
  }
}
