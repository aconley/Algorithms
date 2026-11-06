// Generate all set partitions of a slice.
//
// A partition of n elements is represented by a restricted growth string
// (RGS): a[i] is the block of element i, with a[0] = 0 and
// a[i] <= 1 + max(a[0..i]).  Each partition has exactly one such string, so
// enumerating strings enumerates partitions with no duplicates, and blocks are
// numbered in order of their smallest element.  Strings are visited in
// lexicographic order (Knuth 4A 7.2.1.5, Algorithm H); for n = 3:
//
//   000, 001, 010, 011, 012
//
// Four iterators are provided, in two pairs (plain Iterator / LendingIterator):
//
//   - SetPartitionIndices[Lending]: the RGS itself.
//   - SetPartitions[Lending]: partitions of a &[T] as blocks of references.
//
// The element-aware lending iterator lends a `Partition` view whose blocks are
// real `&[&T]` slices, rebuilt each step by a counting sort of the RGS into
// buffers allocated once up front.  The plain iterators are thin wrappers that
// copy each lent value out, so they and the lending versions share one
// implementation and a benchmark compares like with like.
//
// Elements are distinguished by position, not value.  There is always at least
// one partition -- for n = 0 it is the empty partition with no blocks -- so the
// constructors cannot fail.

use std::fmt;

use lending_iterator::prelude::*;

// ---------------------------------------------------------------------------
// Restricted growth strings
// ---------------------------------------------------------------------------

// LendingIterator over the restricted growth strings of length n.
#[derive(Debug, Clone)]
pub struct SetPartitionIndicesLending {
    // The current string.
    a: Vec<usize>,
    // lim[j] = 1 + max(a[0..j]) is the largest value a[j] may take (lim[0] = 0).
    // It is constant across a run of trailing positions that were zeroed together.
    lim: Vec<usize>,
    started: bool,
    done: bool,
}

impl SetPartitionIndicesLending {
    pub fn new(n: usize) -> Self {
        let mut lim = vec![1; n];
        if n > 0 {
            lim[0] = 0;
        }
        SetPartitionIndicesLending {
            a: vec![0; n],
            lim,
            started: false,
            done: false,
        }
    }

    // Number of blocks in the most recently lent string, i.e. its largest
    // entry plus one (0 for n = 0).
    pub fn num_blocks(&self) -> usize {
        match self.a.len() {
            0 => 0,
            n => self.lim[n - 1] + usize::from(self.a[n - 1] == self.lim[n - 1]),
        }
    }

    // The current string.  Only meaningful after step() returned true.
    pub(crate) fn current(&self) -> &[usize] {
        &self.a
    }

    // Advance to the next string, returning false once they are exhausted.
    pub(crate) fn step(&mut self) -> bool {
        if self.done {
            return false;
        }
        if !self.started {
            self.started = true;
            return true;
        }
        let n = self.a.len();
        if n == 0 {
            self.done = true;
            return false;
        }
        // Common case: just bump the last entry.
        if self.a[n - 1] < self.lim[n - 1] {
            self.a[n - 1] += 1;
            return true;
        }
        // Otherwise find the rightmost earlier entry that can still grow,
        // grow it, and reset everything after it to zero.
        let Some(j) = (1..n - 1).rev().find(|&j| self.a[j] < self.lim[j]) else {
            self.done = true;
            return false;
        };
        self.a[j] += 1;
        // Zeros don't raise the maximum, so the tail shares one limit.
        let m = self.lim[j] + usize::from(self.a[j] == self.lim[j]);
        for k in j + 1..n {
            self.a[k] = 0;
            self.lim[k] = m;
        }
        true
    }
}

#[gat]
impl LendingIterator for SetPartitionIndicesLending {
    type Item<'next>
    where
        Self: 'next,
    = &'next [usize];

    fn next(&'_ mut self) -> Option<&'_ [usize]> {
        if self.step() { Some(&self.a) } else { None }
    }
}

// Iterator over the restricted growth strings of length n, each as a fresh Vec.
#[derive(Debug, Clone)]
pub struct SetPartitionIndices {
    inner: SetPartitionIndicesLending,
}

impl SetPartitionIndices {
    pub fn new(n: usize) -> Self {
        SetPartitionIndices {
            inner: SetPartitionIndicesLending::new(n),
        }
    }
}

impl Iterator for SetPartitionIndices {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Vec<usize>> {
        if self.inner.step() {
            Some(self.inner.current().to_vec())
        } else {
            None
        }
    }
}

// ---------------------------------------------------------------------------
// Partitions of a slice of elements
// ---------------------------------------------------------------------------

// A view of one partition of a slice, borrowed from the iterator that lent it.
//
// 'n is the lifetime of the borrow of the iterator's buffers (one step); 'a is
// the lifetime of the caller's elements.  Keeping them separate lets
// `to_vecs` and `block` return references that outlive the step.
pub struct Partition<'n, 'a, T> {
    // starts[b]..starts[b + 1] is the range of block b in `refs`.
    starts: &'n [usize],
    refs: &'n [&'a T],
    rgs: &'n [usize],
}

// Manual impls: derive would demand T: Clone / Copy.
impl<T> Clone for Partition<'_, '_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Partition<'_, '_, T> {}

impl<'n, 'a, T> Partition<'n, 'a, T> {
    // Number of blocks.
    pub fn len(&self) -> usize {
        self.starts.len() - 1
    }

    // True only for the partition of an empty slice.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // Block `i`, whose elements are in increasing order of original position.
    // Panics if `i >= self.len()`.
    pub fn block(&self, i: usize) -> &'n [&'a T] {
        &self.refs[self.starts[i]..self.starts[i + 1]]
    }

    pub fn get(&self, i: usize) -> Option<&'n [&'a T]> {
        (i < self.len()).then(|| self.block(i))
    }

    // The blocks, ordered by smallest element.
    pub fn blocks(&self) -> Blocks<'n, 'a, T> {
        Blocks {
            starts: self.starts,
            refs: self.refs,
            front: 0,
            back: self.len(),
        }
    }

    // The block containing the element at position `elem` of the input.
    // Panics if `elem` is out of range.
    pub fn block_of(&self, elem: usize) -> usize {
        self.rgs[elem]
    }

    // The restricted growth string of this partition.
    pub fn rgs(&self) -> &'n [usize] {
        self.rgs
    }

    // Copy the partition out of the borrowed buffers.  Allocates, but the
    // result borrows only the caller's elements, so it can outlive the step.
    pub fn to_vecs(&self) -> Vec<Vec<&'a T>> {
        self.blocks().map(|b| b.to_vec()).collect()
    }
}

impl<'n, 'a, T> IntoIterator for Partition<'n, 'a, T> {
    type Item = &'n [&'a T];
    type IntoIter = Blocks<'n, 'a, T>;

    fn into_iter(self) -> Blocks<'n, 'a, T> {
        self.blocks()
    }
}

impl<'n, 'a, T> IntoIterator for &Partition<'n, 'a, T> {
    type Item = &'n [&'a T];
    type IntoIter = Blocks<'n, 'a, T>;

    fn into_iter(self) -> Blocks<'n, 'a, T> {
        self.blocks()
    }
}

impl<T: fmt::Debug> fmt::Debug for Partition<'_, '_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.blocks()).finish()
    }
}

// Iterator over the blocks of a Partition.
pub struct Blocks<'n, 'a, T> {
    starts: &'n [usize],
    refs: &'n [&'a T],
    front: usize,
    back: usize,
}

impl<T> Clone for Blocks<'_, '_, T> {
    fn clone(&self) -> Self {
        Blocks { ..*self }
    }
}

impl<'n, 'a, T> Iterator for Blocks<'n, 'a, T> {
    type Item = &'n [&'a T];

    fn next(&mut self) -> Option<&'n [&'a T]> {
        if self.front == self.back {
            return None;
        }
        let b = &self.refs[self.starts[self.front]..self.starts[self.front + 1]];
        self.front += 1;
        Some(b)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.back - self.front;
        (n, Some(n))
    }
}

impl<T> DoubleEndedIterator for Blocks<'_, '_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front == self.back {
            return None;
        }
        self.back -= 1;
        Some(&self.refs[self.starts[self.back]..self.starts[self.back + 1]])
    }
}

impl<T> ExactSizeIterator for Blocks<'_, '_, T> {}

// LendingIterator over the partitions of a slice, lending a `Partition` view.
// No allocation happens after construction.
pub struct SetPartitionsLending<'a, T> {
    elements: &'a [T],
    rgs: SetPartitionIndicesLending,
    // Block boundaries into `refs`, rebuilt each step (length blocks + 1).
    starts: Vec<usize>,
    // Scatter cursor per block, scratch for the rebuild.
    cursor: Vec<usize>,
    // The elements grouped by block, rebuilt each step.
    refs: Vec<&'a T>,
}

impl<'a, T> SetPartitionsLending<'a, T> {
    pub fn new(elements: &'a [T]) -> Self {
        let n = elements.len();
        SetPartitionsLending {
            elements,
            rgs: SetPartitionIndicesLending::new(n),
            starts: Vec::with_capacity(n + 1),
            cursor: Vec::with_capacity(n),
            // Every slot is overwritten before being read; the initial value
            // only exists because the buffer must be initialized.
            refs: match elements.first() {
                Some(first) => vec![first; n],
                None => Vec::new(),
            },
        }
    }

    // Regroup `elements` by block according to the current RGS: a counting
    // sort, O(n + blocks), stable so blocks keep increasing element order.
    fn rebuild(&mut self) {
        let a = self.rgs.current();
        let k = self.rgs.num_blocks();

        // Block sizes, then prefix sums -> block start offsets.
        self.starts.clear();
        self.starts.resize(k + 1, 0);
        for &b in a {
            self.starts[b + 1] += 1;
        }
        for b in 0..k {
            self.starts[b + 1] += self.starts[b];
        }

        // Scatter each element to the next free slot of its block.
        self.cursor.clear();
        self.cursor.extend_from_slice(&self.starts[..k]);
        for (i, &b) in a.iter().enumerate() {
            self.refs[self.cursor[b]] = &self.elements[i];
            self.cursor[b] += 1;
        }
    }
}

#[gat]
impl<'a, T> LendingIterator for SetPartitionsLending<'a, T> {
    type Item<'next>
    where
        Self: 'next,
    = Partition<'next, 'a, T>;

    fn next(&'_ mut self) -> Option<Partition<'_, 'a, T>> {
        if !self.rgs.step() {
            return None;
        }
        self.rebuild();
        Some(Partition {
            starts: &self.starts,
            refs: &self.refs,
            rgs: self.rgs.current(),
        })
    }
}

// Iterator over the partitions of a slice, each copied out as a fresh
// Vec<Vec<&T>>.
pub struct SetPartitions<'a, T> {
    inner: SetPartitionsLending<'a, T>,
}

impl<'a, T> SetPartitions<'a, T> {
    pub fn new(elements: &'a [T]) -> Self {
        SetPartitions {
            inner: SetPartitionsLending::new(elements),
        }
    }
}

impl<'a, T> Iterator for SetPartitions<'a, T> {
    type Item = Vec<Vec<&'a T>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|p| p.to_vecs())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BELL: [usize; 11] = [1, 1, 2, 5, 15, 52, 203, 877, 4140, 21147, 115975];

    /// Returns true if the provided slice is a restricted growth string.
    fn is_rgs(a: &[usize]) -> bool {
        let mut max_plus_one = 0;
        for &x in a {
            if x > max_plus_one {
                return false;
            }
            max_plus_one = max_plus_one.max(x + 1);
        }
        true
    }

    // All restricted growth strings of length n, in lexicographic order, by
    // brute force.
    fn brute_force_rgs(n: usize) -> Vec<Vec<usize>> {
        let mut out = Vec::new();
        let mut a = vec![0; n];
        loop {
            if is_rgs(&a) {
                out.push(a.clone());
            }
            // Odometer increment over base n.
            let mut i = n;
            loop {
                if i == 0 {
                    return out;
                }
                i -= 1;
                a[i] += 1;
                if a[i] < n {
                    break;
                }
                a[i] = 0;
            }
        }
    }

    fn collect_indices_lending(n: usize) -> Vec<Vec<usize>> {
        let mut it = SetPartitionIndicesLending::new(n);
        let mut out = Vec::new();
        while let Some(a) = it.next() {
            out.push(a.to_vec());
        }
        out
    }

    fn collect_lending<T: Clone>(v: &[T]) -> Vec<Vec<Vec<T>>> {
        let mut it = SetPartitionsLending::new(v);
        let mut out = Vec::new();
        while let Some(p) = it.next() {
            out.push(
                p.blocks()
                    .map(|b| b.iter().map(|&x| x.clone()).collect())
                    .collect(),
            );
        }
        out
    }

    #[test]
    fn bell_counts() {
        for (n, &bell) in BELL.iter().enumerate() {
            assert_eq!(SetPartitionIndices::new(n).count(), bell, "indices n={n}");
            assert_eq!(
                SetPartitionIndicesLending::new(n).count(),
                bell,
                "indices lending n={n}"
            );
            let v: Vec<usize> = (0..n).collect();
            assert_eq!(SetPartitions::new(&v).count(), bell, "partitions n={n}");
            assert_eq!(
                SetPartitionsLending::new(&v).count(),
                bell,
                "partitions lending n={n}"
            );
        }
    }

    #[test]
    fn indices_n3() {
        let expected: Vec<Vec<usize>> = vec![
            vec![0, 0, 0],
            vec![0, 0, 1],
            vec![0, 1, 0],
            vec![0, 1, 1],
            vec![0, 1, 2],
        ];
        assert_eq!(SetPartitionIndices::new(3).collect::<Vec<_>>(), expected);
        assert_eq!(collect_indices_lending(3), expected);
    }

    #[test]
    fn indices_match_brute_force() {
        for n in 0..=7 {
            let expected = brute_force_rgs(n);
            assert_eq!(collect_indices_lending(n), expected, "n={n}");
            assert_eq!(
                SetPartitionIndices::new(n).collect::<Vec<_>>(),
                expected,
                "n={n}"
            );
        }
    }

    #[test]
    fn num_blocks_is_max_plus_one() {
        for n in 0..=8 {
            let mut it = SetPartitionIndicesLending::new(n);
            while let Some(a) = it.next() {
                let expected = a.iter().max().map_or(0, |m| m + 1);
                let a = a.to_vec();
                assert_eq!(it.num_blocks(), expected, "{a:?}");
            }
        }
    }

    #[test]
    fn empty_input_has_one_empty_partition() {
        let empty: Vec<i32> = Vec::new();
        assert_eq!(
            SetPartitionIndices::new(0).collect::<Vec<_>>(),
            vec![vec![]]
        );
        let mut it = SetPartitionsLending::new(&empty);
        let p = it.next().unwrap();
        assert!(p.is_empty());
        assert_eq!(p.len(), 0);
        assert_eq!(p.blocks().count(), 0);
        assert!(it.next().is_none());
        assert_eq!(SetPartitions::new(&empty).count(), 1);
    }

    #[test]
    fn single_element() {
        let v = vec!['x'];
        assert_eq!(collect_lending(&v), vec![vec![vec!['x']]]);
    }

    #[test]
    fn abc_partitions() {
        let v = vec!['a', 'b', 'c'];
        let expected = vec![
            vec![vec!['a', 'b', 'c']],
            vec![vec!['a', 'b'], vec!['c']],
            vec![vec!['a', 'c'], vec!['b']],
            vec![vec!['a'], vec!['b', 'c']],
            vec![vec!['a'], vec!['b'], vec!['c']],
        ];
        assert_eq!(collect_lending(&v), expected);
        let plain: Vec<Vec<Vec<char>>> = SetPartitions::new(&v)
            .map(|p| {
                p.into_iter()
                    .map(|b| b.into_iter().copied().collect())
                    .collect()
            })
            .collect();
        assert_eq!(plain, expected);
    }

    #[test]
    fn rgs_0102_is_ac_b_d() {
        let v = vec!['a', 'b', 'c', 'd'];
        let mut it = SetPartitionsLending::new(&v);
        let mut found = false;
        while let Some(p) = it.next() {
            if p.rgs() == [0, 1, 0, 2] {
                found = true;
                assert_eq!(p.len(), 3);
                assert_eq!(p.block(0), [&'a', &'c']);
                assert_eq!(p.block(1), [&'b']);
                assert_eq!(p.block(2), [&'d']);
                assert_eq!(p.get(3), None);
                assert_eq!(format!("{p:?}"), "[['a', 'c'], ['b'], ['d']]");
            }
        }
        assert!(found);
    }

    // Every structural promise of the Partition view, checked against the RGS.
    #[test]
    fn view_invariants() {
        for n in 0..=8 {
            let v: Vec<usize> = (10..10 + n).collect();
            let mut it = SetPartitionsLending::new(&v);
            while let Some(p) = it.next() {
                let rgs = p.rgs();
                assert_eq!(rgs.len(), n);
                assert!(is_rgs(rgs), "{rgs:?}");
                assert_eq!(p.len(), rgs.iter().max().map_or(0, |m| m + 1));
                assert_eq!(p.blocks().len(), p.len());

                let mut seen = vec![false; n];
                let mut prev_min = None;
                for (b, block) in p.blocks().enumerate() {
                    assert!(!block.is_empty(), "empty block in {rgs:?}");
                    assert!(block.windows(2).all(|w| *w[0] < *w[1]));
                    // Blocks are ordered by smallest element.
                    let min = **block.first().unwrap();
                    assert!(prev_min.is_none_or(|m| m < min));
                    prev_min = Some(min);
                    for &&x in block {
                        let i = x - 10;
                        assert!(!seen[i], "element {i} repeated in {rgs:?}");
                        seen[i] = true;
                        assert_eq!(p.block_of(i), b);
                        assert_eq!(rgs[i], b);
                    }
                }
                assert!(seen.iter().all(|&s| s), "element missing in {rgs:?}");
            }
        }
    }

    #[test]
    fn blocks_borrow_the_original_elements() {
        let v = vec![String::from("x"), String::from("y"), String::from("z")];
        let mut it = SetPartitionsLending::new(&v);
        while let Some(p) = it.next() {
            for block in &p {
                for &s in block {
                    assert!(v.iter().any(|orig| std::ptr::eq(orig, s)));
                }
            }
        }
    }

    #[test]
    fn blocks_iterator_is_double_ended() {
        let v = vec![1, 2, 3, 4];
        let mut it = SetPartitionsLending::new(&v);
        while let Some(p) = it.next() {
            let fwd: Vec<_> = p.blocks().collect();
            let mut rev: Vec<_> = p.blocks().rev().collect();
            rev.reverse();
            assert_eq!(fwd, rev);
        }
    }

    // The point of the second lifetime: a copied-out partition outlives the step.
    #[test]
    fn to_vecs_outlives_the_step() {
        let v = vec![1, 2, 3, 4];
        let mut it = SetPartitionsLending::new(&v);
        let mut best: Vec<Vec<&i32>> = Vec::new();
        while let Some(p) = it.next() {
            if p.len() > best.len() {
                best = p.to_vecs();
            }
        }
        assert_eq!(best, vec![vec![&1], vec![&2], vec![&3], vec![&4]]);
    }

    #[test]
    fn plain_and_lending_agree() {
        let v: Vec<u32> = (0..7).collect();
        let plain: Vec<Vec<Vec<u32>>> = SetPartitions::new(&v)
            .map(|p| {
                p.into_iter()
                    .map(|b| b.into_iter().copied().collect())
                    .collect()
            })
            .collect();
        assert_eq!(plain, collect_lending(&v));
    }

    #[test]
    fn duplicate_values_are_distinct_elements() {
        let v = vec!['x', 'x'];
        assert_eq!(SetPartitions::new(&v).count(), 2);
    }

    #[test]
    fn stays_exhausted() {
        let v = vec![1, 2, 3];
        let mut it = SetPartitions::new(&v);
        assert_eq!(it.by_ref().count(), 5);
        assert!(it.next().is_none());

        let mut lit = SetPartitionsLending::new(&v);
        while lit.next().is_some() {}
        assert!(lit.next().is_none());
        assert!(lit.next().is_none());

        let mut idx = SetPartitionIndicesLending::new(3);
        while idx.next().is_some() {}
        assert!(idx.next().is_none());
        assert!(idx.next().is_none());
    }
}
