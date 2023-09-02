// Finds Langford pairs using Dancing Links.

use crate::backtracking::dancing_links::{
    DancingLinksError, DancingLinksIterator, ProblemOption, ProblemOptionBuilder,
};

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
enum LangfordItem {
    Value(u8),
    Position(u8),
}

// Corresponds to the number value in positions p0 and p1.
// Is both itself and it's own builder.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct LangfordOption {
    value: u8,
    p0: u8,
    p1: u8,
}

impl ProblemOption<LangfordItem> for LangfordOption {
    type IteratorType = std::vec::IntoIter<LangfordItem>;
    type BuilderType = Self;

    fn primary_items(&self) -> Self::IteratorType {
        vec![
            LangfordItem::Value(self.value),
            LangfordItem::Position(self.p0),
            LangfordItem::Position(self.p1),
        ]
        .into_iter()
    }

    fn secondary_items(&self) -> Self::IteratorType {
        vec![].into_iter()
    }

    fn builder() -> Self::BuilderType {
        LangfordOption {
            value: 0,
            p0: u8::MAX,
            p1: u8::MAX,
        }
    }
}

impl ProblemOptionBuilder<LangfordItem> for LangfordOption {
    type ProblemOptionType = Self;

    fn add_primary(&mut self, item: &LangfordItem) -> &mut Self {
        match item {
            LangfordItem::Value(v) => self.value = *v,
            LangfordItem::Position(p) => {
                if self.p0 == u8::MAX {
                    self.p1 = self.p0;
                    self.p0 = *p;
                } else {
                    self.p1 = *p;
                }
            }
        }
        self
    }

    fn add_secondary(&mut self, _item: &LangfordItem) -> &mut Self {
        // There are no secondary langford items.
        self
    }

    fn build(self) -> Self::ProblemOptionType {
        self
    }
}

#[derive(Debug)]
pub struct DancingLangfordIterator {
    n: u8,
    inner: DancingLinksIterator<LangfordItem, LangfordOption>,
}

impl DancingLangfordIterator {
    pub fn new(n: u8) -> Result<Self, DancingLinksError> {
        if n == 0 || n > 32u8 {
            return Err("n not in valid range (1, 32]".into());
        }
        Ok(DancingLangfordIterator {
            n,
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
                    result[option.p0 as usize] = option.value;
                    result[option.p1 as usize] = option.value;
                }
                Some(result)
            }
            None => None,
        }
    }
}

impl std::iter::FusedIterator for DancingLangfordIterator {}

fn create_options(n: u8) -> Result<Vec<LangfordOption>, DancingLinksError> {
    if n == 0 || n > 32u8 {
        return Err(DancingLinksError::new("n not in valid range (0, 32]"));
    }
    //  Following Knuth 7.2.2.1 problem 15, options with i = n - [n even] and
    // j > n/2 - 1 are omitted to remove symmetrical solutions.
    let mut options = Vec::new();
    let n_sym = n - (if n & 1 == 0 { 1 } else { 0 });
    for i in 1..=n {
        if i == n_sym {
            for j in 0..(n / 2) {
                options.push(LangfordOption {
                    value: i,
                    p0: j,
                    p1: i + j + 1,
                });
            }
        } else {
            for j in 0..(2 * n - i - 1) {
                options.push(LangfordOption {
                    value: i,
                    p0: j,
                    p1: i + j + 1,
                });
            }
        }
    }
    Ok(options)
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
