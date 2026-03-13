// DPLL (Algorithm D) from TAOCP 4B §7.2.2.2.
//
// Extends lazy backtracking (Algorithm B) with unit propagation via an
// "active ring" that tracks unset variables with non-empty watch lists.

use super::SatProblem;

/// Attempts to solve the specified SAT problem using DPLL.
pub fn solve_via_dpll(problem: &SatProblem) -> Option<Vec<bool>> {
    Dpll::new(problem).solve()
}

#[derive(Debug)]
struct DpllData {
    n: usize,
    l: Box<[u32]>,
    start: Box<[u32]>,
    link: Box<[u32]>,
    w: Box<[u32]>,
    next: Box<[usize]>,
}

#[derive(Debug)]
enum Dpll {
    Trivial,
    Unsatisfiable,
    Active(DpllData),
}

impl Dpll {
    fn new(problem: &SatProblem) -> Self {
        let clauses = problem.clauses();
        let m = clauses.len();

        if m == 0 {
            return Dpll::Trivial;
        }
        if clauses.iter().any(|c| c.is_empty()) {
            return Dpll::Unsatisfiable;
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

        let next = vec![0usize; n + 1].into_boxed_slice();

        Dpll::Active(DpllData {
            n,
            l: l.into_boxed_slice(),
            start: start.into_boxed_slice(),
            link: link.into_boxed_slice(),
            w: w.into_boxed_slice(),
            next,
        })
    }

    fn solve(self) -> Option<Vec<bool>> {
        match self {
            Dpll::Trivial => Some(vec![]),
            Dpll::Unsatisfiable => None,
            Dpll::Active(data) => data.solve(),
        }
    }
}

impl DpllData {
    /// Returns true if `lit` is currently assigned false.
    ///
    /// Even move codes (0,2,4) assign x=1, so odd literal (neg) is false.
    /// Odd move codes (1,3,5) assign x=0, so even literal (pos) is false.
    #[inline]
    fn literal_is_false(lit: u32, depth_of: &[usize], moves: &[u8]) -> bool {
        let var = (lit / 2) as usize;
        let d = depth_of[var];
        d != 0 && ((lit + moves[d] as u32) & 1) == 1
    }

    /// Returns true if `lit` is the forced literal of some unit clause:
    /// i.e. `lit` is watched in a clause whose all other literals are false.
    fn is_unit(&self, lit: u32, depth_of: &[usize], moves: &[u8]) -> bool {
        let mut j = self.w[lit as usize] as usize;
        while j != 0 {
            let i = self.start[j] as usize;
            let i_end = self.start[j - 1] as usize;
            // p starts after the watched literal
            let mut p = i + 1;
            // scan rest; if all false → unit
            loop {
                if p == i_end {
                    return true;
                }
                if !Self::literal_is_false(self.l[p], depth_of, moves) {
                    break;
                }
                p += 1;
            }
            j = self.link[j] as usize;
        }
        false
    }

    /// Moves all clauses watching `false_lit` to watch some other literal.
    ///
    /// For each such clause, swaps a non-false alternative into the watched
    /// position.  If the new watched variable had no watches and is unset,
    /// it is inserted at the front of the active ring.
    fn clear_watch_list(
        &mut self,
        false_lit: u32,
        depth_of: &[usize],
        moves: &[u8],
        head: &mut usize,
        tail: &mut usize,
    ) {
        let idx = false_lit as usize;
        let mut j = self.w[idx] as usize;
        self.w[idx] = 0;

        while j != 0 {
            let j_next = self.link[j] as usize;
            let i = self.start[j] as usize;

            // Find first non-false literal in positions i+1..start[j-1].
            // This loop always terminates before i_end because we only call
            // clear_watch_list when no conflict has been detected (D3 passed).
            let mut p = i + 1;
            while Self::literal_is_false(self.l[p], depth_of, moves) {
                p += 1;
            }

            let l_prime = self.l[p];
            // Swap l' into watched position
            self.l[p] = false_lit;
            self.l[i] = l_prime;

            let var_prime = (l_prime / 2) as usize;
            // If var_prime has no watches at all and is unset, add to ring
            if self.w[2 * var_prime] == 0
                && self.w[2 * var_prime + 1] == 0
                && depth_of[var_prime] == 0
            {
                if *tail == 0 {
                    self.next[var_prime] = var_prime;
                    *head = var_prime;
                    *tail = var_prime;
                } else {
                    self.next[var_prime] = *head;
                    *head = var_prime;
                    self.next[*tail] = *head;
                }
            }

            // Prepend clause j to watch list of l'
            self.link[j] = self.w[l_prime as usize];
            self.w[l_prime as usize] = j as u32;

            j = j_next;
        }
    }

    fn solve(mut self) -> Option<Vec<bool>> {
        let n = self.n;
        let mut moves = vec![0u8; n + 1];
        let mut h_arr = vec![0usize; n + 1];
        let mut depth_of = vec![0usize; n + 1];
        let mut d = 0usize;

        // D1: Initialize active ring
        let mut head = 0usize;
        let mut tail = 0usize;
        for k in (1..=n).rev() {
            if self.w[2 * k] != 0 || self.w[2 * k + 1] != 0 {
                self.next[k] = head;
                head = k;
                if tail == 0 {
                    tail = k;
                }
            }
        }
        if tail != 0 {
            self.next[tail] = head;
        }

        'main: loop {
            // D2: Success check
            if tail == 0 {
                let mut assignment = vec![false; n];
                for j in 1..=n {
                    let dj = depth_of[j];
                    if dj > 0 {
                        // even move code → x=1, odd → x=0
                        assignment[j - 1] = (moves[dj] & 1) == 0;
                    }
                }
                return Some(assignment);
            }

            // D3: Scan ring for unit clauses
            let mut k_scan = tail;
            let mut conflict = false;
            let (k, move_code) = 'scan: loop {
                let h = self.next[k_scan];
                let f_pos = self.is_unit(2 * h as u32, &depth_of, &moves);
                let f_neg = self.is_unit(2 * h as u32 + 1, &depth_of, &moves);
                match (f_pos, f_neg) {
                    (true, true) => {
                        tail = k_scan;
                        head = h;
                        conflict = true;
                        break 'scan (h, 0); // dummy
                    }
                    (true, false) => {
                        tail = k_scan;
                        head = h;
                        break 'scan (h, 4);
                    }
                    (false, true) => {
                        tail = k_scan;
                        head = h;
                        break 'scan (h, 5);
                    }
                    (false, false) => {
                        if h != tail {
                            k_scan = h;
                        } else {
                            // D4: Free choice
                            head = self.next[tail];
                            let h2 = head;
                            let mc = if self.w[2 * h2] == 0 || self.w[2 * h2 + 1] != 0 {
                                1u8
                            } else {
                                0u8
                            };
                            break 'scan (h2, mc);
                        }
                    }
                }
            };

            if !conflict {
                // D5: Move on
                d += 1;
                h_arr[d] = k;
                let kk = k;
                if tail == kk {
                    tail = 0;
                } else {
                    self.next[tail] = self.next[kk];
                    head = self.next[kk];
                }
                moves[d] = move_code;

                // D6: Update watches
                let b = (moves[d] + 1) % 2;
                depth_of[kk] = d;
                let false_lit = (2 * kk + b as usize) as u32;
                self.clear_watch_list(false_lit, &depth_of, &moves, &mut head, &mut tail);
                continue 'main;
            }

            // D7: Backtrack
            while moves[d] >= 2 {
                let kd = h_arr[d];
                depth_of[kd] = 0;
                if self.w[2 * kd] != 0 || self.w[2 * kd + 1] != 0 {
                    if tail == 0 {
                        self.next[kd] = kd;
                        head = kd;
                        tail = kd;
                    } else {
                        self.next[kd] = head;
                        head = kd;
                        self.next[tail] = head;
                    }
                }
                d -= 1;
            }

            // D8: Failure?
            if d == 0 {
                return None;
            }
            moves[d] = 3 - moves[d];
            let k = h_arr[d];

            // D6 again with flipped polarity
            let b = (moves[d] + 1) % 2;
            depth_of[k] = d;
            let false_lit = (2 * k + b as usize) as u32;
            self.clear_watch_list(false_lit, &depth_of, &moves, &mut head, &mut tail);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sat::{sample_problems::waerden, Clause};

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

    // --- Construction tests ---

    #[test]
    fn test_new_trivial() {
        let p = SatProblem::from_clauses(&[]);
        assert!(matches!(Dpll::new(&p), Dpll::Trivial));
    }

    #[test]
    fn test_new_unsatisfiable() {
        let empty = Clause::new(&[]).unwrap();
        let p = SatProblem::from_clauses(&[empty]);
        assert!(matches!(Dpll::new(&p), Dpll::Unsatisfiable));
    }

    #[test]
    fn test_new_r_prime_layout() {
        let data = match Dpll::new(&r_prime()) {
            Dpll::Active(data) => data,
            other => panic!("expected active, got {other:?}"),
        };

        assert_eq!(data.n, 4);
        assert_eq!(data.start.len() - 1, 7);
        assert_eq!(data.start[0], 21);
        for j in 1..=7 {
            assert_eq!(data.start[j], (21 - 3 * j) as u32, "start[{j}]");
        }

        let expected_l: Vec<u32> = vec![
            9, 7, 3, 8, 7, 5, 6, 5, 3, 8, 4, 3, 8, 6, 2, 9, 6, 4, 7, 4, 2,
        ];
        assert_eq!(&*data.l, &expected_l[..]);

        // next should be all zeros before solve()
        assert!(data.next.iter().all(|&x| x == 0));
    }

    // --- literal_is_false tests ---

    #[test]
    fn test_literal_is_false_unset() {
        let depth_of = vec![0usize; 3];
        let moves = vec![0u8; 3];
        // var 1, depth_of[1]=0 → always false
        assert!(!DpllData::literal_is_false(2, &depth_of, &moves));
        assert!(!DpllData::literal_is_false(3, &depth_of, &moves));
    }

    #[test]
    fn test_literal_is_false_set_true() {
        // move 0 means x=1: pos lit (even) is true, neg lit (odd) is false
        let mut depth_of = vec![0usize; 3];
        let mut moves = vec![0u8; 3];
        depth_of[1] = 1;
        moves[1] = 0;
        assert!(!DpllData::literal_is_false(2, &depth_of, &moves)); // pos x1
        assert!(DpllData::literal_is_false(3, &depth_of, &moves));  // neg x1
    }

    #[test]
    fn test_literal_is_false_forced() {
        // move 4 behaves like 0 (x=1)
        let mut depth_of = vec![0usize; 3];
        let mut moves = vec![0u8; 3];
        depth_of[1] = 1;
        moves[1] = 4;
        assert!(!DpllData::literal_is_false(2, &depth_of, &moves));
        assert!(DpllData::literal_is_false(3, &depth_of, &moves));

        // move 5 behaves like 1 (x=0)
        moves[1] = 5;
        assert!(DpllData::literal_is_false(2, &depth_of, &moves));
        assert!(!DpllData::literal_is_false(3, &depth_of, &moves));
    }

    // --- is_unit tests ---

    #[test]
    fn test_is_unit_empty_watchlist() {
        let data = match Dpll::new(&r_prime()) {
            Dpll::Active(d) => d,
            _ => panic!(),
        };
        let depth_of = vec![0usize; data.n + 1];
        let moves = vec![0u8; data.n + 1];
        // w[2]=0, so lit 2 has no watch list
        assert!(!data.is_unit(2, &depth_of, &moves));
    }

    #[test]
    fn test_is_unit_unit_clause() {
        // single-literal clause {x1}: lit 2, clause 1
        let p = make(&[vec![2]]);
        let data = match Dpll::new(&p) {
            Dpll::Active(d) => d,
            _ => panic!(),
        };
        let depth_of = vec![0usize; data.n + 1];
        let moves = vec![0u8; data.n + 1];
        // The clause has only one literal, so p starts at i+1 == i_end → unit
        assert!(data.is_unit(2, &depth_of, &moves));
    }

    #[test]
    fn test_is_unit_with_false_other_lit() {
        // clause {x1, x2}: if x2 is set false, x1 should be unit
        let p = make(&[vec![2, 4]]);
        let data = match Dpll::new(&p) {
            Dpll::Active(d) => d,
            _ => panic!(),
        };
        let mut depth_of = vec![0usize; data.n + 1];
        let mut moves = vec![0u8; data.n + 1];
        // Set x2=0 (move 1 at depth 1)
        depth_of[2] = 1;
        moves[1] = 1; // x2=0 means lit 4 (pos x2) is false
        // Set x1=0 instead: depth_of[1]=1, moves[1]=1 (x1=0), so lit 2 is false.
        depth_of[1] = 1;
        depth_of[2] = 0;
        moves[1] = 1; // x1=0
        // Now is_unit(4): clause 1, scan l[i+1]=2, literal_is_false(2,...) → depth_of[1]=1, (2+1)&1=1 → true
        // p→p+1=i_end → return true
        assert!(data.is_unit(4, &depth_of, &moves));
    }

    #[test]
    fn test_is_unit_other_lit_not_false() {
        let p = make(&[vec![2, 4]]);
        let data = match Dpll::new(&p) {
            Dpll::Active(d) => d,
            _ => panic!(),
        };
        let depth_of = vec![0usize; data.n + 1];
        let moves = vec![0u8; data.n + 1];
        // Neither x1 nor x2 set → clause not unit
        assert!(!data.is_unit(4, &depth_of, &moves));
    }

    // --- End-to-end solve tests ---

    #[test]
    fn test_solve_trivial() {
        let p = SatProblem::from_clauses(&[]);
        assert_eq!(solve_via_dpll(&p), Some(vec![]));
    }

    #[test]
    fn test_solve_unsatisfiable_empty_clause() {
        let p = make(&[vec![]]);
        assert_eq!(solve_via_dpll(&p), None);
    }

    #[test]
    fn test_solve_unit_clause() {
        let p = make(&[vec![2]]); // {x1}
        let result = solve_via_dpll(&p).expect("satisfiable");
        assert!(p.is_satisfied(&result));
        assert!(result[0]); // x1 = true
    }

    #[test]
    fn test_solve_contradiction() {
        let p = make(&[vec![2], vec![3]]); // {x1} ∧ {¬x1}
        assert_eq!(solve_via_dpll(&p), None);
    }

    #[test]
    fn test_solve_r_prime_sat() {
        let p = r_prime();
        let result = solve_via_dpll(&p).expect("R' is satisfiable");
        assert!(p.is_satisfied(&result));
    }

    #[test]
    fn test_solve_r_prime_unsat() {
        let p = r_prime_unsat();
        assert_eq!(solve_via_dpll(&p), None);
    }

    // W(3,3) = 9: waerden(3,3,n) is SAT for n < 9, UNSAT for n >= 9.
    #[test]
    fn test_solve_waerden_sat() {
        for n in 1u8..=8 {
            let p = waerden(3, 3, n).unwrap();
            let assignment = solve_via_dpll(&p)
                .unwrap_or_else(|| panic!("waerden(3,3,{n}) should be satisfiable"));
            assert!(p.is_satisfied(&assignment), "invalid solution at n={n}");
        }
    }

    #[test]
    fn test_solve_waerden_unsat() {
        for n in 9u8..=11 {
            let p = waerden(3, 3, n).unwrap();
            assert!(
                solve_via_dpll(&p).is_none(),
                "waerden(3,3,{n}) should be unsatisfiable"
            );
        }
    }

    #[test]
    fn test_solve_langford_sat() {
        use crate::sat::sample_problems::langford;
        for n in [3, 4, 7] {
            let p = langford(n).unwrap();
            let res = solve_via_dpll(&p);
            assert!(res.is_some(), "langford({n}) should be satisfiable");
            if let Some(ref assignment) = res {
                assert!(p.is_satisfied(assignment), "invalid solution at n={n}");
            }
        }
    }

    #[test]
    fn test_solve_langford_unsat() {
        use crate::sat::sample_problems::langford;
        for n in [1, 2, 5, 6] {
            let p = langford(n).unwrap();
            let res = solve_via_dpll(&p);
            assert!(res.is_none(), "langford({n}) should be unsatisfiable");
        }
    }
}
