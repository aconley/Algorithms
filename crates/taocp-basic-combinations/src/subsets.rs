// Generate all subsets of a slice.
//
// Three generators share one stepping rule:
//
//   - SubsetsRefLendingIter lends each subset as &[&T], references into the
//     input.  Nothing is ever cloned and T needs no bounds.
//   - SubsetsLendingIter lends each subset as &[T] from an internal buffer of
//     clones, cloning one element per step.  Handier when callers need a
//     plain &[T] and T is cheap to clone.
//   - SubsetsIterator yields an owned Vec<T> per subset.
//
// Subsets are visited in lexicographic order of their index sequences; for
// [a, b, c] that is
//
//   {}, {a}, {a, b}, {a, b, c}, {a, c}, {b}, {b, c}, {c}
//
// The current subset's indices are kept as an increasing stack, and every
// step only touches its top:
//
//   - if the top index k is less than n - 1, push k + 1;
//   - otherwise pop it and replace the new top k with k + 1 (finishing once
//     the stack is empty).
//
// So each step does at most two pops and one push -- O(1) in the worst case,
// not merely amortized -- and the subset is always a contiguous prefix of a
// buffer, which is what lets the LendingIterator lend it out as a slice.
//
// Every input, including the empty one, has at least one subset (the empty
// set), so unlike the other generators in this crate the constructors here
// cannot fail.

use lending_iterator::prelude::*;

// Advance the increasing index stack `idx` (over n elements) to the next
// subset, returning the index now on top -- the only entry that changed; all
// those below it are untouched.  Returns None, leaving `idx` empty, once the
// last subset has already been visited.
#[inline]
fn advance(idx: &mut Vec<usize>, n: usize) -> Option<usize> {
    let next = match idx.last() {
        // Room to grow: append the next index.
        Some(&k) if k + 1 < n => k + 1,
        // {} -> {0}
        None if n > 0 => 0,
        // The top is n - 1 (or n == 0): drop it, then bump the new top.
        _ => {
            idx.pop();
            idx.pop()? + 1
        }
    };
    idx.push(next);
    Some(next)
}

// An Iterator that yields every subset as a freshly allocated Vec<T>.
#[derive(Debug)]
pub struct SubsetsIterator<'a, T: 'a> {
    values: &'a [T],
    indices: Vec<usize>,
    started: bool,
    done: bool,
}

impl<'a, T: Clone> SubsetsIterator<'a, T> {
    pub fn new(v: &'a [T]) -> Self {
        SubsetsIterator {
            values: v,
            indices: Vec::with_capacity(v.len()),
            started: false,
            done: false,
        }
    }
}

impl<'a, T: Clone> Iterator for SubsetsIterator<'a, T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        if self.started && advance(&mut self.indices, self.values.len()).is_none() {
            self.done = true;
            return None;
        }
        self.started = true;
        Some(
            self.indices
                .iter()
                .map(|&i| self.values[i].clone())
                .collect(),
        )
    }
}

// LendingIterator that yields every subset as a borrowed view into an internal
// buffer.  Each step clones at most one element, never a whole subset.
#[derive(Debug)]
pub struct SubsetsLendingIter<'a, T: 'a> {
    values: &'a [T],
    indices: Vec<usize>,
    scratch: Vec<T>,
    started: bool,
    done: bool,
}

impl<'a, T: Clone> SubsetsLendingIter<'a, T> {
    pub fn new(v: &'a [T]) -> Self {
        SubsetsLendingIter {
            values: v,
            indices: Vec::with_capacity(v.len()),
            scratch: Vec::with_capacity(v.len()),
            started: false,
            done: false,
        }
    }

    // Positions in the input of the elements of the most recently lent subset,
    // in increasing order.
    pub fn indices(&self) -> &[usize] {
        &self.indices
    }
}

#[gat]
impl<'a, T: Clone> LendingIterator for SubsetsLendingIter<'a, T> {
    type Item<'next>
    where
        Self: 'next,
    = &'next [T];

    fn next(&'_ mut self) -> Option<&'_ [T]> {
        if self.done {
            return None;
        }
        if self.started {
            let Some(top) = advance(&mut self.indices, self.values.len()) else {
                self.done = true;
                return None;
            };
            // Only the top entry changed, so resync just that one element.
            self.scratch.truncate(self.indices.len() - 1);
            self.scratch.push(self.values[top].clone());
        }
        self.started = true;
        Some(&self.scratch[..])
    }
}

// LendingIterator that yields every subset as a slice of references into the
// input.  Nothing is cloned, so this works for any T and costs the same
// regardless of how large or expensive to clone the elements are.  The
// references themselves live as long as the input, so callers may hold on to
// individual elements across calls to next().
#[derive(Debug)]
pub struct SubsetsRefLendingIter<'a, T: 'a> {
    values: &'a [T],
    indices: Vec<usize>,
    refs: Vec<&'a T>,
    started: bool,
    done: bool,
}

impl<'a, T> SubsetsRefLendingIter<'a, T> {
    pub fn new(v: &'a [T]) -> Self {
        SubsetsRefLendingIter {
            values: v,
            indices: Vec::with_capacity(v.len()),
            refs: Vec::with_capacity(v.len()),
            started: false,
            done: false,
        }
    }

    // Positions in the input of the elements of the most recently lent subset,
    // in increasing order.
    pub fn indices(&self) -> &[usize] {
        &self.indices
    }
}

#[gat]
impl<'a, T> LendingIterator for SubsetsRefLendingIter<'a, T> {
    type Item<'next>
    where
        Self: 'next,
    = &'next [&'a T];

    fn next(&'_ mut self) -> Option<&'_ [&'a T]> {
        if self.done {
            return None;
        }
        if self.started {
            let Some(top) = advance(&mut self.indices, self.values.len()) else {
                self.done = true;
                return None;
            };
            // Only the top entry changed, so resync just that one reference.
            self.refs.truncate(self.indices.len() - 1);
            self.refs.push(&self.values[top]);
        }
        self.started = true;
        Some(&self.refs[..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expected_abc() -> Vec<Vec<char>> {
        vec![
            vec![],
            vec!['a'],
            vec!['a', 'b'],
            vec!['a', 'b', 'c'],
            vec!['a', 'c'],
            vec!['b'],
            vec!['b', 'c'],
            vec!['c'],
        ]
    }

    fn collect_lending<T: Clone>(v: &[T]) -> Vec<Vec<T>> {
        let mut iter = SubsetsLendingIter::new(v);
        let mut actual = Vec::new();
        while let Some(s) = iter.next() {
            actual.push(s.to_vec());
        }
        actual
    }

    fn collect_ref_lending<T: Clone>(v: &[T]) -> Vec<Vec<T>> {
        let mut iter = SubsetsRefLendingIter::new(v);
        let mut actual = Vec::new();
        while let Some(s) = iter.next() {
            actual.push(s.iter().map(|&x| x.clone()).collect());
        }
        actual
    }

    #[test]
    fn subsets_ref_lending_iter_3() {
        let v = vec!['a', 'b', 'c'];
        assert_eq!(collect_ref_lending(&v), expected_abc());
    }

    #[test]
    fn ref_lending_needs_no_clone() {
        // A type that is deliberately not Clone.
        #[derive(Debug, PartialEq)]
        struct NoClone(u32);

        let v = vec![NoClone(1), NoClone(2)];
        let mut iter = SubsetsRefLendingIter::new(&v);
        let mut seen: Vec<Vec<u32>> = Vec::new();
        while let Some(s) = iter.next() {
            seen.push(s.iter().map(|x| x.0).collect());
        }
        assert_eq!(seen, vec![vec![], vec![1], vec![1, 2], vec![2]]);
    }

    #[test]
    fn ref_lending_points_into_input() {
        let v = vec![10, 20, 30];
        let mut iter = SubsetsRefLendingIter::new(&v);
        while let Some(s) = iter.next() {
            // The lent slice borrows `iter`, but its elements are &'a T, so
            // copying them out ends that borrow before indices() is called.
            let s: Vec<&i32> = s.to_vec();
            for (&r, &i) in s.iter().zip(iter.indices()) {
                assert!(std::ptr::eq(r, &v[i]));
            }
        }
    }

    #[test]
    fn ref_lending_elements_outlive_the_step() {
        // Elements are &'a T, so they can be kept across calls to next().
        let v = vec!['a', 'b', 'c'];
        let mut kept: Vec<&char> = Vec::new();
        let mut iter = SubsetsRefLendingIter::new(&v);
        while let Some(s) = iter.next() {
            if let Some(&last) = s.last() {
                kept.push(last);
            }
        }
        assert_eq!(kept.len(), 7);
        assert_eq!(*kept[0], 'a');
    }

    #[test]
    fn subsets_iterator_3() {
        let v = vec!['a', 'b', 'c'];
        let actual: Vec<Vec<char>> = SubsetsIterator::new(&v).collect();
        assert_eq!(actual, expected_abc());
    }

    #[test]
    fn subsets_lending_iter_3() {
        let v = vec!['a', 'b', 'c'];
        assert_eq!(collect_lending(&v), expected_abc());
    }

    #[test]
    fn empty_input_has_one_subset() {
        let empty: Vec<i32> = Vec::new();
        let expected: Vec<Vec<i32>> = vec![vec![]];
        assert_eq!(SubsetsIterator::new(&empty).collect::<Vec<_>>(), expected);
        assert_eq!(collect_lending(&empty), expected);
        assert_eq!(collect_ref_lending(&empty), expected);
    }

    #[test]
    fn single_element() {
        let v = vec![7];
        let expected = vec![vec![], vec![7]];
        assert_eq!(SubsetsIterator::new(&v).collect::<Vec<_>>(), expected);
        assert_eq!(collect_lending(&v), expected);
        assert_eq!(collect_ref_lending(&v), expected);
    }

    #[test]
    fn count_is_power_of_two() {
        for n in 0..=12 {
            let v: Vec<u32> = (0..n).collect();
            assert_eq!(SubsetsIterator::new(&v).count(), 1 << n, "Iterator n={n}");
            assert_eq!(SubsetsLendingIter::new(&v).count(), 1 << n, "Lending n={n}");
            assert_eq!(
                SubsetsRefLendingIter::new(&v).count(),
                1 << n,
                "RefLending n={n}"
            );
        }
    }

    #[test]
    fn iterator_and_lending_agree() {
        let v: Vec<u32> = (0..6).collect();
        let owned: Vec<Vec<u32>> = SubsetsIterator::new(&v).collect();
        assert_eq!(owned, collect_lending(&v));
        assert_eq!(owned, collect_ref_lending(&v));
    }

    #[test]
    fn subsets_are_distinct_and_increasing() {
        let v: Vec<u32> = (0..8).collect();
        let all = collect_lending(&v);
        for s in &all {
            assert!(s.windows(2).all(|w| w[0] < w[1]), "{s:?} not increasing");
        }
        let mut sorted = all.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), all.len());
    }

    #[test]
    fn indices_track_lent_subset() {
        let v = vec!['a', 'b', 'c', 'd'];
        let mut iter = SubsetsLendingIter::new(&v);
        while let Some(s) = iter.next() {
            let s = s.to_vec();
            let from_indices: Vec<char> = iter.indices().iter().map(|&i| v[i]).collect();
            assert_eq!(s, from_indices);
        }
    }

    #[test]
    fn stays_exhausted() {
        let v = vec![1, 2];
        let mut it = SubsetsIterator::new(&v);
        assert_eq!(it.by_ref().count(), 4);
        assert_eq!(it.next(), None);

        let mut lit = SubsetsLendingIter::new(&v);
        while lit.next().is_some() {}
        assert!(lit.next().is_none());
        assert!(lit.next().is_none());

        let mut rit = SubsetsRefLendingIter::new(&v);
        while rit.next().is_some() {}
        assert!(rit.next().is_none());
        assert!(rit.next().is_none());
    }
}
