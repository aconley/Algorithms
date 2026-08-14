// Integer partitions
use std::iter;

use lending_iterator::prelude::*;

// Error returned by the LendingIterator constructor for invalid parameters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidPartitionParams(String);

impl InvalidPartitionParams {
    pub fn new(message: impl Into<String>) -> InvalidPartitionParams {
        InvalidPartitionParams(message.into())
    }
}

impl std::fmt::Display for InvalidPartitionParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for InvalidPartitionParams {}

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
        let mut a = vec![1; n + 1];
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

// LendingIterator version of IntegerPartitions.  Yields a borrowed view into
// the internal working buffer instead of cloning it, using the same
// advance-on-next-call structure as the LendingIterator types in
// combinations.rs.
pub struct IntegerPartitionsLending {
    n: usize,
    a: Vec<usize>,
    m: usize,
    q: usize,
    started: bool,
    done: bool,
}

impl IntegerPartitionsLending {
    pub fn new(n: usize) -> IntegerPartitionsLending {
        let mut a = vec![1; n + 1];
        a[0] = 0;
        if n > 0 {
            a[1] = n;
        }
        IntegerPartitionsLending {
            n,
            a,
            m: 1,
            q: if n == 1 { 0 } else { 1 },
            started: false,
            done: n == 0,
        }
    }
}

#[gat]
impl LendingIterator for IntegerPartitionsLending {
    type Item<'next>
    where
        Self: 'next,
    = &'next [usize];

    fn next(&'_ mut self) -> Option<&'_ [usize]> {
        if self.done {
            return None;
        }

        if self.started {
            // Attempt to advance to the next one.
            if self.a[self.q] == 2 {
                // Easy case -- change a 2 to a 1, 1
                self.a[self.q] = 1;
                self.q -= 1;
                self.m += 1;
                // Now a[q+1..n+1] = 1.
            } else if self.q == 0 {
                // Nothing left to decrease.
                self.done = true;
                return None;
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
        self.started = true;

        Some(&self.a[1..=self.m])
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
            IntegerPartitionsIntoParts::IntegerIteratorTwo(IteratorTwoData {
                a: vec![n - 1, 1],
            })
        } else {
            let mut a = vec![1; m as usize];
            a[0] = n - m + 1;
            IntegerPartitionsIntoParts::IteratorGeneral(IteratorGeneralData {
                m,
                a,
                done: false,
            })
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

// LendingIterator version of IntegerPartitionsIntoParts.  Each branch yields
// a borrowed view into its own internal buffer instead of cloning it, using
// the same advance-on-next-call structure as IntegerPartitionsLending.
#[derive(Debug)]
pub enum IntegerPartitionsIntoPartsLending {
    Single(SingleLendingData),
    Two(IteratorTwoLendingData),
    General(IteratorGeneralLendingData),
}

// A single, fixed solution (the m == n and m == 1 cases).
#[derive(Debug)]
pub struct SingleLendingData {
    value: Vec<usize>,
    done: bool,
}

#[gat]
impl LendingIterator for SingleLendingData {
    type Item<'next>
    where
        Self: 'next,
    = &'next [usize];

    fn next(&'_ mut self) -> Option<&'_ [usize]> {
        if self.done {
            return None;
        }
        self.done = true;
        Some(&self.value[..])
    }
}

// LendingIterator with two pieces.
#[derive(Debug)]
pub struct IteratorTwoLendingData {
    a: Vec<usize>,
    started: bool,
    done: bool,
}

#[gat]
impl LendingIterator for IteratorTwoLendingData {
    type Item<'next>
    where
        Self: 'next,
    = &'next [usize];

    fn next(&'_ mut self) -> Option<&'_ [usize]> {
        if self.done {
            return None;
        }
        if self.started {
            self.a[0] -= 1;
            self.a[1] += 1;
        }
        self.started = true;

        if self.a[0] < self.a[1] {
            self.done = true;
            return None;
        }
        Some(&self.a[..])
    }
}

// LendingIterator for the general case.
#[derive(Debug)]
pub struct IteratorGeneralLendingData {
    m: usize,
    a: Vec<usize>,
    started: bool,
    done: bool,
}

#[gat]
impl LendingIterator for IteratorGeneralLendingData {
    type Item<'next>
    where
        Self: 'next,
    = &'next [usize];

    fn next(&'_ mut self) -> Option<&'_ [usize]> {
        if self.done {
            return None;
        }

        if self.started {
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
                    return None;
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
                return None;
            }
        }
        self.started = true;

        Some(&self.a[..])
    }
}

impl IntegerPartitionsIntoPartsLending {
    pub fn new(
        n: usize,
        m: usize,
    ) -> Result<IntegerPartitionsIntoPartsLending, InvalidPartitionParams> {
        if n == 0 {
            return Err(InvalidPartitionParams::new("n must be > 0"));
        }
        if m == 0 {
            return Err(InvalidPartitionParams::new("m must be > 0"));
        }
        if n < m {
            return Err(InvalidPartitionParams::new("m must be <= n"));
        }

        if m == n {
            Ok(IntegerPartitionsIntoPartsLending::Single(
                SingleLendingData {
                    value: vec![1; n],
                    done: false,
                },
            ))
        } else if m == 1 {
            Ok(IntegerPartitionsIntoPartsLending::Single(
                SingleLendingData {
                    value: vec![n],
                    done: false,
                },
            ))
        } else if m == 2 {
            Ok(IntegerPartitionsIntoPartsLending::Two(
                IteratorTwoLendingData {
                    a: vec![n - 1, 1],
                    started: false,
                    done: false,
                },
            ))
        } else {
            let mut a = vec![1; m];
            a[0] = n - m + 1;
            Ok(IntegerPartitionsIntoPartsLending::General(
                IteratorGeneralLendingData {
                    m,
                    a,
                    started: false,
                    done: false,
                },
            ))
        }
    }
}

#[gat]
impl LendingIterator for IntegerPartitionsIntoPartsLending {
    type Item<'next>
    where
        Self: 'next,
    = &'next [usize];

    fn next(&'_ mut self) -> Option<&'_ [usize]> {
        match self {
            IntegerPartitionsIntoPartsLending::Single(i) => i.next(),
            IntegerPartitionsIntoPartsLending::Two(i) => i.next(),
            IntegerPartitionsIntoPartsLending::General(i) => i.next(),
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

    // LendingIterator versions.

    #[test]
    fn lending_count_n0() {
        assert_eq!(IntegerPartitionsLending::new(0).count(), 0);
    }

    #[test]
    fn lending_count_n8() {
        assert_eq!(IntegerPartitionsLending::new(8).count(), 22);
    }

    #[test]
    fn lending_values_n8() {
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
        let actual = collect_lending(IntegerPartitionsLending::new(8));
        assert_eq!(actual, expected);
    }

    #[test]
    fn into_parts_lending_count_m1() {
        assert_eq!(
            IntegerPartitionsIntoPartsLending::new(1, 1)
                .unwrap()
                .count(),
            1
        );
        assert_eq!(
            IntegerPartitionsIntoPartsLending::new(10, 1)
                .unwrap()
                .count(),
            1
        );
    }

    #[test]
    fn into_parts_lending_count_n7() {
        assert_eq!(
            IntegerPartitionsIntoPartsLending::new(7, 2)
                .unwrap()
                .count(),
            3
        );
        assert_eq!(
            IntegerPartitionsIntoPartsLending::new(7, 3)
                .unwrap()
                .count(),
            4
        );
        assert_eq!(
            IntegerPartitionsIntoPartsLending::new(7, 4)
                .unwrap()
                .count(),
            3
        );
        assert_eq!(
            IntegerPartitionsIntoPartsLending::new(7, 5)
                .unwrap()
                .count(),
            2
        );
        assert_eq!(
            IntegerPartitionsIntoPartsLending::new(7, 6)
                .unwrap()
                .count(),
            1
        );
        assert_eq!(
            IntegerPartitionsIntoPartsLending::new(7, 7)
                .unwrap()
                .count(),
            1
        );
    }

    #[test]
    fn into_parts_lending_values_n11_m4() {
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
        let actual =
            collect_lending(IntegerPartitionsIntoPartsLending::new(11, 4).unwrap());
        assert_eq!(actual, expected);
    }

    #[test]
    fn into_parts_lending_invalid_params() {
        assert_eq!(
            IntegerPartitionsIntoPartsLending::new(0, 1).unwrap_err(),
            InvalidPartitionParams::new("n must be > 0")
        );
        assert_eq!(
            IntegerPartitionsIntoPartsLending::new(1, 0).unwrap_err(),
            InvalidPartitionParams::new("m must be > 0")
        );
        assert_eq!(
            IntegerPartitionsIntoPartsLending::new(1, 2).unwrap_err(),
            InvalidPartitionParams::new("m must be <= n")
        );
    }

    #[apply(Gat!)]
    fn collect_lending<I>(mut iter: I) -> Vec<Vec<usize>>
    where
        I: for<'n> LendingIterator<Item<'n> = &'n [usize]>,
    {
        let mut result = Vec::new();
        while let Some(v) = iter.next() {
            result.push(v.to_vec());
        }
        result
    }
}
