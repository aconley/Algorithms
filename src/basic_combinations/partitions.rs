// Integer partitions
use std::iter;

// Knuth 4A 7.2.1.4 Algorithm P

// An iterator over integer partitions.
pub struct IntegerPartitions {
    // The integer being partitioned.
    n: usize,
    // Current state
    a: Vec<usize>,
    m: usize,
    q: usize,
    // Are we done?
    done: bool,
}

impl IntegerPartitions {
    pub fn new(n: usize) -> IntegerPartitions {
        let mut a = vec![1; (n + 1) as usize];
        a[0] = 0;
        if n > 0 {
            a[1] = n;
        }
        IntegerPartitions {
            n,
            a,
            m: 1,
            q: if n == 1 { 0 } else { 1 },
            done: n == 0,
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
        let ret = Some(self.a[1..=self.m].to_vec());

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
                self.q = if self.n == 1 { self.m - 1 } else { self.m }
            }
        }
        ret
    }
}

// An iterator over integer partitions into a fixed number of parts.
// Knuth 4A 7.2.1.4 Algorithm H

pub enum IntegerPartitionsIntoParts {
    IntegerIteratorSingle(iter::Once<Vec<usize>>),
    IntegerIteratorTwo(IteratorTwoData),
    IteratorGeneral(IteratorGeneralData),
}

// Iterator with two pieces
pub struct IteratorTwoData {
    a: Vec<usize>,
}

impl Iterator for IteratorTwoData {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.a[0] < self.a[1] {
            None
        } else {
            let r = self.a.clone();
            self.a[0] -= 1;
            self.a[1] += 1;
            Some(r)
        }
    }
}

// General case.
pub struct IteratorGeneralData {
    m: usize,
    a: Vec<usize>,
    done: bool,
}

impl Iterator for IteratorGeneralData {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        // Make a copy of the current solution.
        let ret = self.a.to_vec();

        // Attempt to advance to the next one.
        if self.a[1] < self.a[0] - 1 {
            //Easy case.
            self.a[0] -= 1;
            self.a[1] += 1;
        } else if self.m > 2 {
            // Find the smallest j such that a[j] < a[0] - 1 and let s = (sum_k=0^j-1 a_k) - 1
            let mut j = 2;
            let mut s = self.a[0] + self.a[1] - 1;
            let a0m1 = self.a[0] - 1;
            while j < self.m - 1 && self.a[j] >= a0m1 {
                s += self.a[j];
                j += 1;
            }

            // Try to increase a[j].
            if j == self.m - 1 && self.a[j] >= a0m1 {
                self.done = true;
            } else {
                self.a[j] += 1;
                let x = self.a[j];
                j -= 1;

                // Fix up a[0..j].
                while j > 0 {
                    self.a[j] = x;
                    s -= x;
                    j -= 1;
                }
                self.a[0] = s;
            }
        } else {
            self.done = true;
        }
        Some(ret)
    }
}

impl IntegerPartitionsIntoParts {
    pub fn new(n: usize, m: usize) -> IntegerPartitionsIntoParts {
        assert!(n > 0, "n = 0");
        assert!(m > 0, "m = 0");
        assert!(n >= m, "m > n");

        if m == n {
            IntegerPartitionsIntoParts::IntegerIteratorSingle(iter::once(vec![1; n]))
        } else if m == 1 {
            IntegerPartitionsIntoParts::IntegerIteratorSingle(iter::once(vec![n]))
        } else if m == 2 {
            IntegerPartitionsIntoParts::IntegerIteratorTwo(IteratorTwoData { a: vec![n - 1, 1] })
        } else {
            let mut a = vec![1; m as usize];
            a[0] = n - m + 1;
            IntegerPartitionsIntoParts::IteratorGeneral(IteratorGeneralData { m, a, done: false })
        }
    }
}

impl Iterator for IntegerPartitionsIntoParts {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            IntegerPartitionsIntoParts::IntegerIteratorSingle(ref mut i) => i.next(),
            IntegerPartitionsIntoParts::IntegerIteratorTwo(ref mut i) => i.next(),
            IntegerPartitionsIntoParts::IteratorGeneral(ref mut i) => i.next(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Partitions
    #[test]
    fn count_n0() {
        assert_eq!(IntegerPartitions::new(0).count(), 0);
    }

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
        let expected: Vec<Vec<usize>> = vec![vec![1]];
        let actual: Vec<Vec<usize>> = IntegerPartitions::new(1).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn values_n3() {
        let expected: Vec<Vec<usize>> = vec![vec![3], vec![2, 1], vec![1, 1, 1]];
        let actual: Vec<Vec<usize>> = IntegerPartitions::new(3).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn values_n4() {
        let expected: Vec<Vec<usize>> =
            vec![vec![4], vec![3, 1], vec![2, 2], vec![2, 1, 1], vec![1; 4]];
        let actual: Vec<Vec<usize>> = IntegerPartitions::new(4).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn values_n8() {
        let expected: Vec<Vec<usize>> = vec![
            vec![8],
            vec![7, 1],
            vec![6, 2],
            vec![6, 1, 1],
            vec![5, 3],
            vec![5, 2, 1],
            vec![5, 1, 1, 1],
            vec![4, 4],
            vec![4, 3, 1],
            vec![4, 2, 2],
            vec![4, 2, 1, 1],
            vec![4, 1, 1, 1, 1],
            vec![3, 3, 2],
            vec![3, 3, 1, 1],
            vec![3, 2, 2, 1],
            vec![3, 2, 1, 1, 1],
            vec![3, 1, 1, 1, 1, 1],
            vec![2; 4],
            vec![2, 2, 2, 1, 1],
            vec![2, 2, 1, 1, 1, 1],
            vec![2, 1, 1, 1, 1, 1, 1],
            vec![1; 8],
        ];
        let actual: Vec<Vec<usize>> = IntegerPartitions::new(8).collect();
        assert_eq!(actual, expected);
    }

    // Partitions into a fixed number of parts
    #[test]
    fn count_m1() {
        assert_eq!(IntegerPartitionsIntoParts::new(1, 1).count(), 1);
        assert_eq!(IntegerPartitionsIntoParts::new(10, 1).count(), 1);
    }

    #[test]
    fn count_n_eq_m() {
        assert_eq!(IntegerPartitionsIntoParts::new(2, 2).count(), 1);
        assert_eq!(IntegerPartitionsIntoParts::new(10, 10).count(), 1);
    }

    #[test]
    fn count_n7() {
        assert_eq!(IntegerPartitionsIntoParts::new(7, 2).count(), 3);
        assert_eq!(IntegerPartitionsIntoParts::new(7, 3).count(), 4);
        assert_eq!(IntegerPartitionsIntoParts::new(7, 4).count(), 3);
        assert_eq!(IntegerPartitionsIntoParts::new(7, 5).count(), 2);
        assert_eq!(IntegerPartitionsIntoParts::new(7, 6).count(), 1);
        assert_eq!(IntegerPartitionsIntoParts::new(7, 7).count(), 1);
    }

    #[test]
    fn count_n11() {
        assert_eq!(IntegerPartitionsIntoParts::new(11, 3).count(), 10);
        assert_eq!(IntegerPartitionsIntoParts::new(11, 4).count(), 11);
    }

    #[test]
    fn values_n11_m4() {
        let expected: Vec<Vec<usize>> = vec![
            vec![8, 1, 1, 1],
            vec![7, 2, 1, 1],
            vec![6, 3, 1, 1],
            vec![5, 4, 1, 1],
            vec![6, 2, 2, 1],
            vec![5, 3, 2, 1],
            vec![4, 4, 2, 1],
            vec![4, 3, 3, 1],
            vec![5, 2, 2, 2],
            vec![4, 3, 2, 2],
            vec![3, 3, 3, 2],
        ];
        let actual: Vec<Vec<usize>> = IntegerPartitionsIntoParts::new(11, 4).collect();
        assert_eq!(actual, expected);
    }
}
