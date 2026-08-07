// Lazy Backtracking (Algorithm B) from TAOCP 4B §7.2.2.2.

use super::SatProblem;

// Algorithm B move codes (TAOCP 4B §7.2.2.2):
// 0: trying x_d = 1 first
// 1: trying x_d = 0 first
// 2: trying x_d = 1 after x_d = 0 failed
// 3: trying x_d = 0 after x_d = 1 failed
const MOVE_TRY_TRUE_FIRST: u8 = 0;
const MOVE_TRY_FALSE_FIRST: u8 = 1;
const MOVE_TRY_TRUE_SECOND: u8 = 2;

/// Attempts to solve the specified SAT problem using lazy backtracking.
///
/// If a solution is found, returns an assignment (as a Vec<bool>) that
/// satisfies the problem. If no solution is found, returns None.
/// The returned `Vec<bool>` is 0-indexed: `assignment[i]` is the value of
/// x_{i+1}.
pub fn solve_via_lazy_backtracking(problem: &SatProblem) -> Option<Vec<bool>> {
    LazyBacktracking::new(problem).solve()
}

#[derive(Debug)]
struct LazyBacktrackingData {
    n: usize,
    l: Box<[u32]>,
    start: Box<[u32]>,
    link: Box<[u32]>,
    w: Box<[u32]>,
}

#[derive(Debug)]
enum LazyBacktracking {
    Trivial,
    Unsatisfiable,
    Active(LazyBacktrackingData),
}

impl LazyBacktracking {
    fn solve(self) -> Option<Vec<bool>> {
        match self {
            LazyBacktracking::Trivial => Some(vec![]),
            LazyBacktracking::Unsatisfiable => None,
            LazyBacktracking::Active(data) => data.solve(),
        }
    }

    fn new(problem: &SatProblem) -> Self {
        let clauses = problem.clauses();
        let m = clauses.len();

        if m == 0 {
            return LazyBacktracking::Trivial;
        }

        if clauses.iter().any(|c| c.is_empty()) {
            return LazyBacktracking::Unsatisfiable;
        }

        let n = clauses
            .iter()
            .flat_map(|c| c.literals().iter())
            .map(|&lit| lit / 2)
            .max()
            .unwrap_or(0) as usize;

        let total_lits: usize = clauses.iter().map(|c| c.len()).sum();

        let mut l = vec![0u32; total_lits];
        let mut start = vec![0u32; m + 1];
        start[0] = total_lits as u32;

        let mut pos = total_lits;
        for j in 1..=m {
            pos -= clauses[j - 1].len();
            start[j] = pos as u32;

            // Store clause literals in reverse sorted order so the watched
            // literal starts at start[j]. This preserves Algorithm B
            // invariants; the exact `l`/`w` layout differs from the prose
            // example that assumes forward literal order.
            for (offset, &lit) in clauses[j - 1].literals().iter().rev().enumerate() {
                l[pos + offset] = lit;
            }
        }

        let mut link = vec![0u32; m + 1];
        let mut w = vec![0u32; 2 * n + 2];

        for j in 1..=m {
            let watched = l[start[j] as usize] as usize;
            link[j] = w[watched];
            w[watched] = j as u32;
        }

        LazyBacktracking::Active(LazyBacktrackingData {
            n,
            l: l.into_boxed_slice(),
            start: start.into_boxed_slice(),
            link: link.into_boxed_slice(),
            w: w.into_boxed_slice(),
        })
    }
}

impl LazyBacktrackingData {
    #[inline]
    fn literal_is_false(lit: u32, d: usize, moves: &[u8]) -> bool {
        let var = (lit / 2) as usize;
        var <= d && ((lit + moves[var] as u32) & 1) == 1
    }

    /// Reassigns watched clauses that currently watch `false_lit`.
    ///
    /// Returns `true` if all such clauses can watch a different literal.
    /// Returns `false` if at least one clause has no alternative non-false
    /// literal, in which case `w[false_lit]` points to the first failing clause.
    fn watch_false_literal(&mut self, false_lit: u32, d: usize, moves: &[u8]) -> bool {
        let idx = false_lit as usize;
        let mut j = self.w[idx];

        while j != 0 {
            let clause = j as usize;
            let i = self.start[clause] as usize;
            let i_prime = self.start[clause - 1] as usize;
            let next_j = self.link[clause];

            let mut moved = false;
            for k in i + 1..i_prime {
                let lit = self.l[k];
                let lit_idx = lit as usize;
                if !Self::literal_is_false(lit, d, moves) {
                    self.l[i] = lit;
                    self.l[k] = false_lit;
                    self.link[clause] = self.w[lit_idx];
                    self.w[lit_idx] = j;
                    j = next_j;
                    moved = true;
                    break;
                }
            }

            if !moved {
                self.w[idx] = j;
                return false;
            }
        }

        self.w[idx] = 0;
        true
    }

    fn solve(mut self) -> Option<Vec<bool>> {
        let n = self.n;
        let mut moves = vec![0u8; n + 1];
        let mut d = 1usize;

        'choose: loop {
            if d > n {
                let mut assignment = vec![false; n];
                for j in 1..=n {
                    assignment[j - 1] = (moves[j] & 1) == 0;
                }
                return Some(assignment);
            }

            let pos = 2 * d;
            let neg = pos + 1;
            moves[d] = if self.w[pos] == 0 || self.w[neg] != 0 {
                MOVE_TRY_FALSE_FIRST
            } else {
                MOVE_TRY_TRUE_FIRST
            };

            let mut lit = (2 * d) as u32 + moves[d] as u32;

            loop {
                if self.watch_false_literal(lit ^ 1, d, &moves) {
                    d += 1;
                    continue 'choose;
                }

                if moves[d] < MOVE_TRY_TRUE_SECOND {
                    moves[d] = 3 - moves[d];
                    lit = (2 * d) as u32 + (moves[d] & 1) as u32;
                    continue;
                }

                loop {
                    if d == 1 {
                        return None;
                    }

                    d -= 1;
                    if moves[d] < MOVE_TRY_TRUE_SECOND {
                        moves[d] = 3 - moves[d];
                        lit = (2 * d) as u32 + (moves[d] & 1) as u32;
                        break;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make(clauses: &[Vec<u32>]) -> SatProblem {
        SatProblem::from_literals(clauses).unwrap()
    }

    fn r_prime() -> SatProblem {
        make(&[
            vec![2, 4, 7],
            vec![4, 6, 9],
            vec![2, 6, 8],
            vec![3, 4, 8],
            vec![3, 5, 6],
            vec![5, 7, 8],
            vec![3, 7, 9],
        ])
    }

    fn r_prime_unsat() -> SatProblem {
        make(&[
            vec![2, 4, 7],
            vec![4, 6, 9],
            vec![2, 6, 8],
            vec![3, 4, 8],
            vec![3, 5, 6],
            vec![5, 7, 8],
            vec![3, 7, 9],
            vec![2, 5, 9],
        ])
    }

    #[test]
    fn test_new_trivial() {
        let p = SatProblem::from_clauses(&[]);
        assert!(matches!(
            LazyBacktracking::new(&p),
            LazyBacktracking::Trivial
        ));
    }

    #[test]
    fn test_new_unsatisfiable() {
        use super::super::Clause;
        let empty = Clause::new(&[]).unwrap();
        let p = SatProblem::from_clauses(&[empty]);
        assert!(matches!(
            LazyBacktracking::new(&p),
            LazyBacktracking::Unsatisfiable
        ));
    }

    #[test]
    fn test_new_r_prime_layout() {
        let data = match LazyBacktracking::new(&r_prime()) {
            LazyBacktracking::Active(data) => data,
            other => panic!("expected active, got {other:?}"),
        };

        assert_eq!(data.n, 4);
        assert_eq!(data.start.len() - 1, 7);
        assert_eq!(data.start[0], 21);
        for j in 1..=7 {
            assert_eq!(data.start[j], (21 - 3 * j) as u32, "start[{j}]");
        }

        let expected_l = vec![
            9, 7, 3, 8, 7, 5, 6, 5, 3, 8, 4, 3, 8, 6, 2, 9, 6, 4, 7, 4, 2,
        ];
        assert_eq!(&*data.l, &expected_l);

        assert_eq!(data.w[2], 0);
        assert_eq!(data.w[3], 0);
        assert_eq!(data.w[4], 0);
        assert_eq!(data.w[5], 0);
        assert_eq!(data.w[6], 5);
        assert_eq!(data.w[7], 1);
        assert_eq!(data.w[8], 6);
        assert_eq!(data.w[9], 7);

        assert_eq!(data.link[6], 4);
        assert_eq!(data.link[4], 3);
        assert_eq!(data.link[3], 0);
        assert_eq!(data.link[7], 2);
        assert_eq!(data.link[2], 0);
    }

    #[test]
    fn test_watch_false_literal_reassigns_all() {
        let mut data = match LazyBacktracking::new(&r_prime()) {
            LazyBacktracking::Active(data) => data,
            _ => panic!("expected active"),
        };

        let moves = vec![0u8, 0, 0, 0, 0];

        assert!(data.watch_false_literal(8, 1, &moves));
        assert_eq!(data.w[8], 0);

        let clause = 6usize;
        let start = data.start[clause] as usize;
        assert_ne!(data.l[start], 8);
        assert!(data.l[start + 1..data.start[clause - 1] as usize].contains(&8));
    }

    #[test]
    fn test_watch_false_literal_reports_failure() {
        let p = make(&[vec![2], vec![3]]);
        let mut data = match LazyBacktracking::new(&p) {
            LazyBacktracking::Active(data) => data,
            _ => panic!("expected active"),
        };

        let moves = vec![0u8, 0];
        assert!(!data.watch_false_literal(3, 1, &moves));
        assert_eq!(data.w[3], 2);
    }

    #[test]
    fn test_solve_r_prime() {
        let p = r_prime();
        let assignment = solve_via_lazy_backtracking(&p).expect("R' is satisfiable");
        assert!(p.is_satisfied(&assignment));
    }

    #[test]
    fn test_solve_r_prime_unsat() {
        let p = r_prime_unsat();
        assert_eq!(solve_via_lazy_backtracking(&p), None);
    }
}
