// Solves the N-queens problem using Dancing Links.

use crate::backtracking::dancing_links::{
    DancingLinksError, DancingLinksIterator, ProblemOption, ProblemOptionBuilder,
};
use std::fmt;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
enum NQueensItem {
    // Primary.
    Row(u8),
    Column(u8),
    // Secondary.
    UpDiagonal(u8),
    DownDiagonal(i8),
}

struct NQueensOption {
    row: u8,
    column: u8,
}

impl ProblemOption<NQueensItem> for NQueensOption {
    type PrimaryIteratorType = std::array::IntoIter<NQueensItem, 2>;
    type SecondaryIteratorType = std::array::IntoIter<NQueensItem, 2>;
    type BuilderType = Self;

    fn primary_items(&self) -> Self::PrimaryIteratorType {
        [NQueensItem::Row(self.row), NQueensItem::Column(self.column)].into_iter()
    }

    fn secondary_items(&self) -> Self::SecondaryIteratorType {
        [
            NQueensItem::UpDiagonal(self.row + self.column),
            NQueensItem::DownDiagonal(self.row as i8 - self.column as i8),
        ]
        .into_iter()
    }

    fn builder() -> Self::BuilderType {
        NQueensOption { row: 0, column: 0 }
    }
}

impl ProblemOptionBuilder<NQueensItem> for NQueensOption {
    type ProblemOptionType = Self;

    fn add_primary(&mut self, item: &NQueensItem) -> &mut Self {
        match item {
            NQueensItem::Row(r) => self.row = *r,
            NQueensItem::Column(c) => self.column = *c,
            _ => (),
        }
        self
    }

    fn add_secondary(&mut self, _item: &NQueensItem) -> &mut Self {
        self
    }

    fn build(self) -> Self::ProblemOptionType {
        self
    }
}

// Return type from iterator.
#[derive(PartialEq, Eq, Debug)]
pub struct NQueensSolution {
    // For each row, the column number.
    rows: Vec<u8>,
}

impl NQueensSolution {
    fn new(n: u8, options: Vec<NQueensOption>) -> Self {
        let mut result = NQueensSolution {
            rows: vec![0; n as usize],
        };
        for option in options {
            result.rows[option.row as usize] = option.column;
        }
        result
    }
}

impl fmt::Display for NQueensSolution {
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

pub struct DancingQueensIterator {
    n: u8,
    inner: DancingLinksIterator<NQueensItem, NQueensOption>,
}

impl DancingQueensIterator {
    pub fn new(n: u8) -> Result<Self, DancingLinksError> {
        if n == 0 || n > 32u8 {
            return Err("n not in valid range (1, 32]".into());
        }
        let mut options = Vec::with_capacity((n as usize) * (n as usize));
        for row_idx in 0..n {
            for col_idx in 0..n {
                options.push(NQueensOption {
                    row: row_idx,
                    column: col_idx,
                });
            }
        }
        Ok(DancingQueensIterator {
            n,
            inner: DancingLinksIterator::new(options)?,
        })
    }
}

impl Iterator for DancingQueensIterator {
    type Item = NQueensSolution;
    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(options) => Some(NQueensSolution::new(self.n, options)),
            None => None,
        }
    }
}

impl std::iter::FusedIterator for DancingQueensIterator {}

#[cfg(test)]
mod tests {
    use crate::backtracking::dancing_queens::NQueensSolution;

    #[test]
    fn formatter_works() {
        let sol = NQueensSolution {
            rows: vec![2, 0, 3, 1],
        };
        assert_eq!(
            format!("{}", sol),
            ". . Q . \nQ . . . \n. . . Q \n. Q . . \n"
        );
    }

    mod dancing_queens_iterator_test {
        use crate::backtracking::dancing_queens::{DancingQueensIterator, NQueensSolution};
        use claim::assert_ok;

        #[test]
        fn count_n1() {
            assert_eq!(assert_ok!(DancingQueensIterator::new(1)).count(), 1);
        }

        #[test]
        fn count_n2() {
            assert_eq!(assert_ok!(DancingQueensIterator::new(2)).count(), 0);
        }

        #[test]
        fn count_n4() {
            assert_eq!(assert_ok!(DancingQueensIterator::new(4)).count(), 2);
        }

        #[test]
        fn count_n5() {
            assert_eq!(assert_ok!(DancingQueensIterator::new(5)).count(), 10);
        }

        #[test]
        fn count_n8() {
            assert_eq!(assert_ok!(DancingQueensIterator::new(8)).count(), 92);
        }

        #[test]
        fn count_n10() {
            assert_eq!(assert_ok!(DancingQueensIterator::new(10)).count(), 724);
        }

        #[test]
        fn values_n4() {
            let mut q = assert_ok!(DancingQueensIterator::new(4));
            assert_eq!(
                q.next(),
                Some(NQueensSolution {
                    rows: vec![1, 3, 0, 2]
                })
            );
            assert_eq!(
                q.next(),
                Some(NQueensSolution {
                    rows: vec![2, 0, 3, 1]
                })
            );
            assert_eq!(q.next(), None);
            assert_eq!(q.next(), None);
        }
    }
}
