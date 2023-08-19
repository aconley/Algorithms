// Finds Langford pairs using Dancing Links.

use crate::backtracking::dancing_links::{DancingLinksError, DancingLinksIterator, ProblemOption};

#[derive(Debug)]
pub struct DancingLangfordIterator {
    n: u8,
    inner: DancingLinksIterator,
}

impl DancingLangfordIterator {
    pub fn new(n: u8) -> Result<Self, DancingLinksError> {
        if n == 0 || n > 32u8 {
            return Err("n not in valid range (1, 32]".into());
        }
        Ok(DancingLangfordIterator {
            n: n,
            inner: DancingLinksIterator::new(create_options(n)?)?,
        })
    }
}

impl Iterator for DancingLangfordIterator {
    type Item = Vec<u8>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(options) => {
                let mut result = vec![0; 2 * self.n as usize];
                for option in options {
                    // Dancing links guarantees the items are in lexicographic
                    // order, which puts the i ahead of s_j,s_k.
                    let i: u8 = option.primary_items[0].parse().unwrap();
                    let j: usize = option.primary_items[1]
                        .strip_prefix("s_")
                        .unwrap()
                        .parse()
                        .unwrap();
                    let k: usize = option.primary_items[2]
                        .strip_prefix("s_")
                        .unwrap()
                        .parse()
                        .unwrap();
                    result[j] = i;
                    result[k] = i;
                }
                Some(result)
            }
            None => None,
        }
    }
}

fn create_options(n: u8) -> Result<Vec<ProblemOption>, DancingLinksError> {
    if n == 0 || n > 32u8 {
        return Err(DancingLinksError::new("n not in valid range (0, 32]"));
    }
    // Options are triplets { i s_j s_k } which means positions j and k
    // should have value i, where k = i + j + 1.
    //  Following Knuth 7.2.2.1 problem 15, options with i = n - [n even] and
    // j > n/2 - 1 are omitted to remove symmetrical solutions.
    let mut options = Vec::new();
    let n_sym = n - (if n & 1 == 0 { 1 } else { 0 });
    for i in 1..=n {
        if i == n_sym {
            for j in 0..(n / 2) {
                options.push(ProblemOption::new(
                    vec![
                        format!("{}", i),
                        format!("s_{}", j),
                        format!("s_{}", i + j + 1),
                    ],
                    vec![],
                ));
            }
        } else {
            for j in 0..(2 * n - i - 1) {
                options.push(ProblemOption::new(
                    vec![
                        format!("{}", i),
                        format!("s_{}", j),
                        format!("s_{}", i + j + 1),
                    ],
                    vec![],
                ));
            }
        }
    }
    // Convert Vec<Result<E>> -> Result<V<E>>.
    options.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use claim::assert_ok;

    #[test]
    fn count_no_solutions() {
        assert_eq!(assert_ok!(DancingLangfordIterator::new(1)).count(), 0);
        assert_eq!(assert_ok!(DancingLangfordIterator::new(2)).count(), 0);
        assert_eq!(assert_ok!(DancingLangfordIterator::new(5)).count(), 0);
        assert_eq!(assert_ok!(DancingLangfordIterator::new(6)).count(), 0);
        assert_eq!(assert_ok!(DancingLangfordIterator::new(9)).count(), 0);
        assert_eq!(assert_ok!(DancingLangfordIterator::new(10)).count(), 0);
    }

    #[test]
    fn count_small_with_solutions() {
        // Recall that we don't include the mirror solutions.
        assert_eq!(assert_ok!(DancingLangfordIterator::new(3)).count(), 1);
        assert_eq!(assert_ok!(DancingLangfordIterator::new(4)).count(), 1);
    }

    #[test]
    fn count_medium_with_solutions() {
        assert_eq!(assert_ok!(DancingLangfordIterator::new(7)).count(), 26);
        assert_eq!(assert_ok!(DancingLangfordIterator::new(8)).count(), 150);
    }

    #[test]
    fn count_large_number_solutions() {
        assert_eq!(assert_ok!(DancingLangfordIterator::new(11)).count(), 17792);
    }

    #[test]
    fn expected_solutions_n3() {
        assert_eq!(
            assert_ok!(DancingLangfordIterator::new(3)).collect::<Vec<_>>(),
            vec![vec![3, 1, 2, 1, 3, 2]]
        );
    }

    #[test]
    fn expected_solutions_n4() {
        assert_eq!(
            assert_ok!(DancingLangfordIterator::new(4)).collect::<Vec<_>>(),
            vec![vec![2, 3, 4, 2, 1, 3, 1, 4]]
        );
    }
}
