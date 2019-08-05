use std::fmt;

// A solution to the nqueens problem for n queens.
#[derive(PartialEq, Eq, Debug)]
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

// An iterator over nqueens solutions.
pub struct NQueensIterator {
    // Number of queens.
    n: usize,
    // Current level.
    l: usize,
    // State vectors.
    a: Vec<u32>,
    b: Vec<u32>,
    c: Vec<u32>,
    s: Vec<u32>,
    mu: u32,
    // Are we done?
    done: bool
}

impl NQueensIterator {
    pub fn new(n: u8) -> NQueensIterator {
        assert!(n > 0, "n = 0");
        assert!(n <= 32, "n > 32");
        let mu = if n == 32 { !0u32 } else { (1u32 << n) - 1 };
        let nu = n as usize;
        
        NQueensIterator {
            n: nu,
            l: 0,
            a: vec![0; nu],
            b: vec![0; nu],
            c: vec![0; nu],
            s: vec![mu; nu],
            mu: mu,
            done: false,
        }
    }
}

impl Iterator for NQueensIterator {
    type Item = Solution;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        if self.l == self.n {
            // Backtrack
            self.l -= 1;
            while self.s[self.l] == 0 {
                if self.l == 0 {
                    self.done = true;
                    return None
                }
                self.l -= 1;
            }
        }

        let mut t;

        loop {
            // Backtrack if no choices on level l.
            while self.s[self.l] == 0 {
                if self.l == 0 {
                    self.done = true;
                    return None
                }
                self.l -= 1;
            }
       
            t = self.s[self.l] & (!self.s[self.l] + 1);
            self.s[self.l] -= t;
            self.l += 1;
            
            if self.l == self.n {
                // Found solution.
                let mut sv = vec![0; self.n];
                for i in 0..(self.n - 1) {
                    sv[i] = (self.a[i + 1] - self.a[i]).trailing_zeros() as u8;
                }
                sv[self.n - 1] = t.trailing_zeros() as u8;
                return Some(Solution { n: self.n as u8, rows: sv });
            }

            self.a[self.l] = self.a[self.l - 1] + t;
            self.b[self.l] = (self.b[self.l - 1] + t) >> 1;
            self.c[self.l] = ((self.c[self.l - 1] + t) << 1) & self.mu;
            self.s[self.l] = 
              self.mu & !self.a[self.l] & !self.b[self.l] & !self.c[self.l];
        }
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
    fn count_n1() {
        assert_eq!(NQueensIterator::new(1).count(), 1);
    }

    #[test]
    fn count_n2() {
        assert_eq!(NQueensIterator::new(2).count(), 0);
    }

    #[test]
    fn count_n4() {
        assert_eq!(NQueensIterator::new(4).count(), 2);
    }

    #[test]
    fn count_n5() {
        assert_eq!(NQueensIterator::new(5).count(), 10);
    }


    #[test]
    fn count_n8() {
        assert_eq!(NQueensIterator::new(8).count(), 92);
    }

    #[test]
    fn count_n10() {
        assert_eq!(NQueensIterator::new(10).count(), 724);
    }

    #[test]
    fn values_n4() {
        let mut q = NQueensIterator::new(4);
        assert_eq!(q.next(), Some(Solution{ n: 4, rows: vec![1, 3, 0, 2] }));
        assert_eq!(q.next(), Some(Solution{ n: 4, rows: vec![2, 0, 3, 1] }));
        assert_eq!(q.next(), None);
    }
}
