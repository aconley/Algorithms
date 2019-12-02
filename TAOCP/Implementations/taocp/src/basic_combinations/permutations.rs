// Integer partitions
// Knuth 4A 7.2.1.4 Algorithm P

// An iterator over integer partitions.
pub struct IntegerPartitions {
  n: usize,
  // Current state
  a: Vec<usize>,
  m: usize,
  q: usize,
  // Are we done?
  done: bool
}

impl IntegerPartitions {
  pub fn new(n: usize) -> IntegerPartitions {
    assert!(n > 0, "n = 0");
    let mut a = vec![1; (n+1) as usize];
    a[0] = 0;
    a[1] = n;
    IntegerPartitions {
      n: n,
      a: a,
      m: 1,
      q: if n == 1 { 0 } else { 1 },
      done: false
    }
  }
}

impl Iterator for IntegerPartitions {
  type Item = Vec<usize>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.done {
      return None;
    }

    // Make a copy of the current solution.
    let ret = self.a[1..=self.m].to_vec();

    // Attempt to advance to the next one.
    if self.a[self.q] == 2 {
      // Easy case -- change a 2 to a 1, 1
      self.a[self.q] = 1;
      self.q -= 1;
      self.m += 1;
      // Now a[q+1..n+1] = 1.
    } else {
      // Try to decrease a[q]
      if self.q == 0 {
        self.done = true;
      } else {
        let x = self.a[self.q] - 1;
        
        self.a[self.q] = x;
        self.n = self.m - self.q + 1;
        self.m = self.q + 1;

        // Insert as many copies of x as we can.
        while self.n > x {
          self.a[self.m] = x;
          self.m += 1;
          self.n -= x;
        }
        self.a[self.m] = self.n;
        self.q = if  self.n == 1 { self.m - 1 } else { self.m }
      }
    }
    Some(ret)
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_n1() {
        assert_eq!(IntegerPartitions::new(1).count(), 1);
    }

    #[test]
    fn count_n2() {
        assert_eq!(IntegerPartitions::new(2).count(), 2);
    }

    #[test]
    fn count_n3() {
        assert_eq!(IntegerPartitions::new(3).count(), 3);
    }

    #[test]
    fn count_n8() {
        assert_eq!(IntegerPartitions::new(8).count(), 22);
    }

    #[test]
    fn values_n1() {
      let expected: Vec<Vec<usize>>= vec![vec![1]];
      let actual: Vec<Vec<usize>> = IntegerPartitions::new(1).collect();
      assert_eq!(actual, expected);
    }

    #[test]
    fn values_n3() {
      let expected: Vec<Vec<usize>>= vec![vec![3], vec![2, 1], vec![1, 1, 1]];
      let actual: Vec<Vec<usize>> = IntegerPartitions::new(3).collect();
      assert_eq!(actual, expected);
    }

    #[test]
    fn values_n4() {
      let expected: Vec<Vec<usize>>= vec![vec![4], vec![3, 1], vec![2, 2], vec![2, 1, 1], vec![1; 4]];
      let actual: Vec<Vec<usize>> = IntegerPartitions::new(4).collect();
      assert_eq!(actual, expected);
    }

    #[test]
    fn values_n8() {
      let expected: Vec<Vec<usize>>= vec![
        vec![8], vec![7, 1], vec![6, 2], vec![6, 1, 1], vec![5, 3], vec![5, 2, 1], vec![5, 1, 1, 1], vec![4, 4], 
        vec![4, 3, 1], vec![4, 2, 2], vec![4, 2, 1, 1], vec![4, 1, 1, 1, 1], vec![3, 3, 2], vec![3, 3, 1, 1], 
        vec![3, 2, 2, 1], vec![3, 2, 1, 1, 1], vec![3, 1, 1, 1, 1, 1], vec![2; 4], vec![2, 2, 2, 1, 1],
        vec![2, 2, 1, 1, 1, 1], vec![2, 1, 1, 1, 1, 1, 1], vec![1; 8]];
      let actual: Vec<Vec<usize>> = IntegerPartitions::new(8).collect();
      assert_eq!(actual, expected);
    }
}

