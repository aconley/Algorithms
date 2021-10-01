use std::fmt;

// A solution to the nqueens problem for n queens.
#[derive(PartialEq, Eq, Debug)]
pub struct Solution {
    // For each of the n rows, the column number [0, n)
    pub rows: Vec<u8>,
}

impl Clone for Solution {
    fn clone(&self) -> Solution {
        Solution {
            rows: self.rows.clone(),
        }
    }
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = self.rows.len();
        let mut board = vec![vec![false; n]; n];

        for (i, col) in self.rows.iter().enumerate() {
            board[i as usize][*col as usize] = true;
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

#[derive(Debug)]
enum IteratorState {
    // New iterator.
    NEW,
    // Ready to generate next solution.
    READY,
    // Iterator is exhausted.
    DONE,
}

pub struct NQueensIterator {
    // Number of queens.
    n: u8,
    // Current level.
    l: usize,
    // State vectors.
    a: Vec<u32>,
    b: Vec<u32>,
    c: Vec<u32>,
    // Move in final row.
    final_move: u8,
    // Current allowed values at each level.
    s: Vec<u32>,
    // Bitmask for values.
    mu: u32,
    // State of iterator.
    state: IteratorState,
}

impl NQueensIterator {
    pub fn new(n: u8) -> NQueensIterator {
        assert!(n > 0, "n must be positive");
        assert!(n <= 32, "n must be <= 32");
        let nu = n as usize;
        let mu = if n == 32 { !0u32 } else { (1u32 << n) - 1 };
        NQueensIterator {
            n: n,
            l: 0,
            a: vec![0; nu],
            b: vec![0; nu],
            c: vec![0; nu],
            s: vec![mu; nu],
            final_move: 0,
            mu: mu,
            state: IteratorState::NEW,
        }
    }

    fn to_solution(&self) -> Solution {
        let mut sv = vec![0; self.n as usize];
        let nm1 = self.n as usize - 1;
        for i in 0..nm1 {
            sv[i] = (self.a[i + 1] - self.a[i]).trailing_zeros() as u8;
        }
        sv[nm1] = self.final_move;
        Solution { rows: sv }
    }
}

impl Iterator for NQueensIterator {
    type Item = Solution;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            IteratorState::NEW => {}
            IteratorState::DONE => return None,
            IteratorState::READY => {
                // Backtrack!
                self.l -= 1;
            }
        }

        loop {
            if self.s[self.l] == 0 {
                // No options remaining on level l.
                if self.l == 0 {
                    // No more things to try.
                    self.state = IteratorState::DONE;
                    return None;
                }
                self.l -= 1;
                continue;
            }
            // Find the next possible move at level l.
            let t = self.s[self.l] & (!self.s[self.l] + 1);
            // We pre-remove t rather than remove on backtrack.
            self.s[self.l] -= t;
            self.l += 1;

            if self.l == self.n as usize {
                // Solution!
                self.state = IteratorState::READY;
                self.final_move = t.trailing_zeros() as u8;
                return Some(self.to_solution());
            }

            // Update the state
            self.a[self.l] = self.a[self.l - 1] + t;
            self.b[self.l] = (self.b[self.l - 1] + t) >> 1;
            self.c[self.l] = ((self.c[self.l - 1] + t) << 1) & self.mu;
            self.s[self.l] = self.mu & !self.a[self.l] & !self.b[self.l] & !self.c[self.l];
        }
    }
}

// An iterator over nqueens solutions using Walkers method.
// Alternative implementation.
pub struct NQueensIteratorAlt {
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
    done: bool,
}

impl NQueensIteratorAlt {
    pub fn new(n: u8) -> NQueensIteratorAlt {
        assert!(n > 0, "n = 0");
        assert!(n <= 32, "n > 32");
        let mu = if n == 32 { !0u32 } else { (1u32 << n) - 1 };
        let nu = n as usize;
        NQueensIteratorAlt {
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

impl Iterator for NQueensIteratorAlt {
    type Item = Solution;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        if self.l == self.n {
            // Backtrack
            self.l -= 1;
        }

        loop {
            // Backtrack if no choices on level l.
            while self.s[self.l] == 0 {
                if self.l == 0 {
                    self.done = true;
                    return None;
                }
                self.l -= 1;
            }
            let t = self.s[self.l] & (!self.s[self.l] + 1);
            self.s[self.l] -= t;
            self.l += 1;
            if self.l == self.n {
                // Found solution.
                let mut sv = vec![0; self.n];
                for i in 0..(self.n - 1) {
                    sv[i] = (self.a[i + 1] - self.a[i]).trailing_zeros() as u8;
                }
                sv[self.n - 1] = t.trailing_zeros() as u8;
                return Some(Solution { rows: sv });
            }

            self.a[self.l] = self.a[self.l - 1] + t;
            self.b[self.l] = (self.b[self.l - 1] + t) >> 1;
            self.c[self.l] = ((self.c[self.l - 1] + t) << 1) & self.mu;
            self.s[self.l] = self.mu & !self.a[self.l] & !self.b[self.l] & !self.c[self.l];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formatter_works() {
        let sol = Solution {
            rows: vec![2, 0, 3, 1],
        };
        assert_eq!(
            format!("{}", sol),
            ". . Q . \nQ . . . \n. . . Q \n. Q . . \n"
        );
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
        assert_eq!(
            q.next(),
            Some(Solution {
                rows: vec![1, 3, 0, 2]
            })
        );
        assert_eq!(
            q.next(),
            Some(Solution {
                rows: vec![2, 0, 3, 1]
            })
        );
        assert_eq!(q.next(), None);
        assert_eq!(q.next(), None);
    }
}
