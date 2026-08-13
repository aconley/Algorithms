// Routines to generate combinations -- n things taken t at a time.

use lending_iterator::prelude::*;

pub trait Visitor {
    // Visits a single combination.  Returns true if further solutions should
    // be visited, otherwise false.
    fn visit(&mut self, combination: &[u32]) -> bool;
}

// A visitor which counts solutions.
pub struct CountingVisitor {
    pub n_solutions: u64,
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
    solutions: Vec<Vec<u32>>,
}

impl RecordingVisitor {
    pub fn new() -> RecordingVisitor {
        RecordingVisitor {
            solutions: Vec::new(),
        }
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

// Iterator version of Algorithm L (basic_generate).  Yields owned combinations,
// cloning out of the internal working buffer on each call to next().
pub struct BasicGenerateIter {
    c: Vec<u32>,
    ts: usize,
    started: bool,
    done: bool,
}

impl BasicGenerateIter {
    pub fn new(n: u32, t: u32) -> BasicGenerateIter {
        assert!(n >= t, "n must be >= t");

        let ts = t as usize;

        // L1: Initialize
        let mut c = Vec::with_capacity(ts + 2);
        for i in 0..ts {
            c.push(i as u32);
        }
        c.push(n);
        c.push(0);

        BasicGenerateIter {
            c,
            ts,
            started: false,
            done: n == 0 || t == 0,
        }
    }
}

impl Iterator for BasicGenerateIter {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Vec<u32>> {
        if self.done {
            return None;
        }

        if self.started {
            // L3: Find c[j] to increase.
            let mut j = 0;
            while self.c[j] + 1 == self.c[j + 1] {
                self.c[j] = j as u32;
                j += 1;
            }

            // L4: Terminate.
            if j >= self.ts {
                self.done = true;
                return None;
            }

            // L5: Increase c[j]
            self.c[j] += 1;
        }
        self.started = true;

        // L2: Visit.
        Some(self.c[0..self.ts].to_vec())
    }
}

// Iterator version of Algorithm T (combinations).  Yields owned combinations,
// cloning out of the internal working buffer on each call to next().
pub struct CombinationsIter {
    c: Vec<u32>,
    ts: usize,
    j: usize,
    started: bool,
    done: bool,
    single: bool,
}

impl CombinationsIter {
    pub fn new(n: u32, t: u32) -> CombinationsIter {
        assert!(n >= t, "n must be >= t");

        let ts = t as usize;
        let done = n == 0 || t == 0;
        // Algorithm T assumes t < n.
        let single = !done && n == t;

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

        CombinationsIter {
            c,
            ts,
            j: ts,
            started: false,
            done,
            single,
        }
    }
}

impl Iterator for CombinationsIter {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Vec<u32>> {
        if self.done {
            return None;
        }

        if self.single {
            self.done = true;
            return Some((0..self.ts as u32).collect());
        }

        if self.started {
            if self.j > 0 {
                // T6: increase c_j
                self.c[self.j] = self.j as u32;
                self.j -= 1;
            } else if self.c[1] + 1 < self.c[2] {
                // T3: Easy case?
                self.c[1] += 1;
            } else {
                // T4: find j.
                self.c[1] = 0;
                self.j = 2;
                let mut x = self.c[2] + 1;
                while x == self.c[self.j + 1] {
                    self.j += 1;
                    self.c[self.j - 1] = (self.j - 2) as u32;
                    x = self.c[self.j] + 1;
                }

                // T5: done?
                if self.j > self.ts {
                    self.done = true;
                    return None;
                }

                // T6: increase cj
                self.c[self.j] = x;
                self.j -= 1;
            }
        }
        self.started = true;

        // L2: Visit.
        Some(self.c[1..=self.ts].to_vec())
    }
}

// LendingIterator version of Algorithm L (basic_generate).  Like the Visitor
// pattern, yields each combination as a borrowed view into the internal
// working buffer instead of cloning it.
pub struct BasicGenerateLendingIter {
    c: Vec<u32>,
    ts: usize,
    started: bool,
    done: bool,
}

impl BasicGenerateLendingIter {
    pub fn new(n: u32, t: u32) -> BasicGenerateLendingIter {
        assert!(n >= t, "n must be >= t");

        let ts = t as usize;

        // L1: Initialize
        let mut c = Vec::with_capacity(ts + 2);
        for i in 0..ts {
            c.push(i as u32);
        }
        c.push(n);
        c.push(0);

        BasicGenerateLendingIter {
            c,
            ts,
            started: false,
            done: n == 0 || t == 0,
        }
    }
}

#[gat]
impl LendingIterator for BasicGenerateLendingIter {
    type Item<'next>
    where
        Self: 'next,
    = &'next [u32];

    fn next(&'_ mut self) -> Option<&'_ [u32]> {
        if self.done {
            return None;
        }

        if self.started {
            // L3: Find c[j] to increase.
            let mut j = 0;
            while self.c[j] + 1 == self.c[j + 1] {
                self.c[j] = j as u32;
                j += 1;
            }

            // L4: Terminate.
            if j >= self.ts {
                self.done = true;
                return None;
            }

            // L5: Increase c[j]
            self.c[j] += 1;
        }
        self.started = true;

        // L2: Visit.
        Some(&self.c[0..self.ts])
    }
}

// LendingIterator version of Algorithm T (combinations).  Like the Visitor
// pattern, yields each combination as a borrowed view into the internal
// working buffer instead of cloning it.
pub struct CombinationsLendingIter {
    c: Vec<u32>,
    ts: usize,
    j: usize,
    started: bool,
    done: bool,
    single: bool,
}

impl CombinationsLendingIter {
    pub fn new(n: u32, t: u32) -> CombinationsLendingIter {
        assert!(n >= t, "n must be >= t");

        let ts = t as usize;
        let done = n == 0 || t == 0;
        // Algorithm T assumes t < n.
        let single = !done && n == t;

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

        CombinationsLendingIter {
            c,
            ts,
            j: ts,
            started: false,
            done,
            single,
        }
    }
}

#[gat]
impl LendingIterator for CombinationsLendingIter {
    type Item<'next>
    where
        Self: 'next,
    = &'next [u32];

    fn next(&'_ mut self) -> Option<&'_ [u32]> {
        if self.done {
            return None;
        }

        if self.single {
            self.done = true;
            // L1's initialization already set c[1..=ts] to [0, ts).
            return Some(&self.c[1..=self.ts]);
        }

        if self.started {
            if self.j > 0 {
                // T6: increase c_j
                self.c[self.j] = self.j as u32;
                self.j -= 1;
            } else if self.c[1] + 1 < self.c[2] {
                // T3: Easy case?
                self.c[1] += 1;
            } else {
                // T4: find j.
                self.c[1] = 0;
                self.j = 2;
                let mut x = self.c[2] + 1;
                while x == self.c[self.j + 1] {
                    self.j += 1;
                    self.c[self.j - 1] = (self.j - 2) as u32;
                    x = self.c[self.j] + 1;
                }

                // T5: done?
                if self.j > self.ts {
                    self.done = true;
                    return None;
                }

                // T6: increase cj
                self.c[self.j] = x;
                self.j -= 1;
            }
        }
        self.started = true;

        // L2: Visit.
        Some(&self.c[1..=self.ts])
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

    #[test]
    fn basic_generate_iter_count() {
        test_iter_counts(BasicGenerateIter::new);
    }

    #[test]
    fn basic_generate_iter_visit() {
        test_iter_visit(BasicGenerateIter::new);
    }

    #[test]
    fn combinations_iter_count() {
        test_iter_counts(CombinationsIter::new);
    }

    #[test]
    fn combinations_iter_visit() {
        test_iter_visit(CombinationsIter::new);
    }

    #[test]
    fn basic_generate_lending_count() {
        test_lending_counts(BasicGenerateLendingIter::new);
    }

    #[test]
    fn basic_generate_lending_visit() {
        test_lending_visit(BasicGenerateLendingIter::new);
    }

    #[test]
    fn combinations_lending_count() {
        test_lending_counts(CombinationsLendingIter::new);
    }

    #[test]
    fn combinations_lending_visit() {
        test_lending_visit(CombinationsLendingIter::new);
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

    fn test_iter_counts<I: Iterator<Item = Vec<u32>>>(f: impl Fn(u32, u32) -> I) {
        // 3 choose 3
        assert_eq!(f(3, 3).count(), 1);

        // 3 choose 2
        assert_eq!(f(3, 2).count(), 3);

        // 5 choose 2
        assert_eq!(f(5, 2).count(), 10);

        // 6 choose 3
        assert_eq!(f(6, 3).count(), 20);

        // 10 choose 4
        assert_eq!(f(10, 4).count(), 210);
    }

    fn test_iter_visit<I: Iterator<Item = Vec<u32>>>(f: impl Fn(u32, u32) -> I) {
        // 4 choose 4
        let solutions: Vec<_> = f(4, 4).collect();
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0], [0, 1, 2, 3]);

        // 6 choose 3
        let solutions: Vec<_> = f(6, 3).collect();
        assert_eq!(solutions.len(), 20);
        assert_eq!(solutions[0], [0, 1, 2]);
        assert_eq!(solutions[1], [0, 1, 3]);
    }

    #[apply(Gat!)]
    fn test_lending_counts<I: LendingIterator>(f: impl Fn(u32, u32) -> I) {
        // 3 choose 3
        assert_eq!(f(3, 3).count(), 1);

        // 3 choose 2
        assert_eq!(f(3, 2).count(), 3);

        // 5 choose 2
        assert_eq!(f(5, 2).count(), 10);

        // 6 choose 3
        assert_eq!(f(6, 3).count(), 20);

        // 10 choose 4
        assert_eq!(f(10, 4).count(), 210);
    }

    #[apply(Gat!)]
    fn test_lending_visit<I>(f: impl Fn(u32, u32) -> I)
    where
        I: for<'n> LendingIterator<Item<'n> = &'n [u32]>,
    {
        // 4 choose 4
        let mut iter = f(4, 4);
        assert_eq!(iter.next(), Some(&[0, 1, 2, 3][..]));
        assert_eq!(iter.next(), None);

        // 6 choose 3
        let mut iter = f(6, 3);
        assert_eq!(iter.next(), Some(&[0, 1, 2][..]));
        assert_eq!(iter.next(), Some(&[0, 1, 3][..]));
    }
}
