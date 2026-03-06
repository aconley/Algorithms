// Basic Backtracking (Algorithm A) from TAOCP 4B §7.2.2.2.

use super::SatProblem;

// Public interface.

/// Attempts to solve the specified SAT problem using a basic backtracking
/// approach.
///
/// If a solution is found, returns an assignment (as a Vec<bool>) that
/// satisfies the problem.  If no solution is found, returns None.
/// The returned `Vec<bool>` is 0-indexed: `assignment[i]` is the value of
/// x_{i+1}.  Variables not constrained by the active search (depth < n) are
/// set to `false`.
pub fn solve_via_backtracking(problem: &SatProblem) -> Option<Vec<bool>> {
    BasicBacktracking::new(problem).solve()
}

// ---------------------------------------------------------------------------
// Data structure
// ---------------------------------------------------------------------------

#[derive(Debug)]
struct BasicBacktrackingData {
    n: u32,            // number of variables
    l: Box<[u32]>,     // literal at cell p          (len = 2n+2+total_lits)
    f: Box<[u32]>,     // forward pointer at cell p
    b: Box<[u32]>,     // backward pointer at cell p
    c: Box<[u32]>,     // clause# (p>2n+1) or count (p≤2n+1)
    start: Box<[u32]>, // start[i] = first cell of clause i (1-indexed, len=m+1)
    size: Box<[u32]>,  // size[i]  = literal count of clause i (1-indexed, len=m+1)
    a: usize,          // number of active (unsatisfied) clauses
    d: usize,          // decision depth
}

#[derive(Debug)]
enum BasicBacktracking {
    Trivial,       // no clauses → always satisfiable
    Unsatisfiable, // contains an empty clause → immediately unsat
    Active(BasicBacktrackingData),
}

impl BasicBacktracking {
    /// Runs Algorithm A (TAOCP 4B §7.2.2.2) and returns a satisfying variable
    /// assignment, or `None` if the problem is unsatisfiable.
    ///
    /// The returned `Vec<bool>` is 0-indexed: `assignment[i]` is the value of
    /// x_{i+1}.  Variables not constrained by the active search (depth < n) are
    /// set to `false`.
    fn solve(self) -> Option<Vec<bool>> {
        match self {
            BasicBacktracking::Trivial => Some(vec![]),
            BasicBacktracking::Unsatisfiable => None,
            BasicBacktracking::Active(data) => data.solve(),
        }
    }

    fn new(problem: &SatProblem) -> Self {
        let clauses = problem.clauses();
        let m = clauses.len();

        // Step 1: no clauses → trivially satisfiable.
        if m == 0 {
            return BasicBacktracking::Trivial;
        }

        // Step 2: any empty clause → immediately unsatisfiable.
        if clauses.iter().any(|c| c.is_empty()) {
            return BasicBacktracking::Unsatisfiable;
        }

        // Step 3: n = max variable index across all literals.
        let n = clauses
            .iter()
            .flat_map(|c| c.literals().iter())
            .map(|&lit| lit / 2)
            .max()
            .unwrap_or(0);

        let total_lits: usize = clauses.iter().map(|c| c.len()).sum();
        let total_cells = (2 * n as usize + 2) + total_lits;

        let mut l = vec![0u32; total_cells];
        let mut f = vec![0u32; total_cells];
        let mut b = vec![0u32; total_cells];
        let mut c = vec![0u32; total_cells];

        // Step 5: compute start[i] and size[i] (1-indexed).
        // Clause 1 lands at the high end, clause m at the low end.
        let mut start = vec![0u32; m + 1];
        let mut size = vec![0u32; m + 1];
        let mut pos = total_cells;
        for i in 1..=m {
            size[i] = clauses[i - 1].len() as u32;
            pos -= size[i] as usize;
            start[i] = pos as u32;
        }

        // Step 6: fill l[p] and c[p] for clause cells.
        // Literals are stored in decreasing order within each clause.
        for i in 1..=m {
            for (j, &lit) in clauses[i - 1].literals().iter().rev().enumerate() {
                let p = start[i] as usize + j;
                l[p] = lit;
                c[p] = i as u32;
            }
        }

        // Step 7: initialise header cells (p = 2..=2n+1) as empty circular lists.
        for p in 2..=(2 * n as usize + 1) {
            f[p] = p as u32;
            b[p] = p as u32;
            // c[p] already 0
        }

        // Step 8: build linked lists by iterating p downward from total_cells-1 to 2n+2.
        // Appending p to the back of the circular list headed by l[p] ensures the
        // list reads clause 1 → clause 2 → … → clause m in forward order.
        for p in (2 * n as usize + 2..total_cells).rev() {
            let lit = l[p] as usize;
            let back = b[lit] as usize;
            f[back] = p as u32;
            b[p] = back as u32;
            f[p] = lit as u32;
            b[lit] = p as u32;
            c[lit] += 1;
        }

        BasicBacktracking::Active(BasicBacktrackingData {
            n,
            l: l.into_boxed_slice(),
            f: f.into_boxed_slice(),
            b: b.into_boxed_slice(),
            c: c.into_boxed_slice(),
            start: start.into_boxed_slice(),
            size: size.into_boxed_slice(),
            a: m,
            d: 0,
        })
    }
}

impl BasicBacktrackingData {
    /// Index of the first clause cell; header cells occupy indices 0..threshold.
    #[inline]
    fn threshold(&self) -> usize {
        2 * self.n as usize + 2
    }

    /// Removes `¬lit` from all active clauses by decrementing their sizes.
    ///
    /// Returns `true` if successful. Returns `false` if any clause would become
    /// empty; in that case all size changes are rolled back.
    fn remove_false_literal(&mut self, lit: u32) -> bool {
        let not_lit = (lit ^ 1) as usize;
        let threshold = self.threshold();

        let mut p = self.f[not_lit] as usize;
        while p >= threshold {
            let j = self.c[p] as usize;
            if self.size[j] == 1 {
                // Would empty clause j — roll back all prior decrements.
                let mut q = self.b[p] as usize;
                while q >= threshold {
                    self.size[self.c[q] as usize] += 1;
                    q = self.b[q] as usize;
                }
                return false;
            }
            self.size[j] -= 1;
            p = self.f[p] as usize;
        }
        true
    }

    /// Covers all clauses containing `lit`: splices their other literals out of
    /// occurrence lists, decrements occurrence counts, and reduces `a` by `c[lit]`.
    fn deactivate_literal(&mut self, lit: u32) {
        let threshold = self.threshold();
        let lit_idx = lit as usize;

        let mut p = self.f[lit_idx] as usize;
        while p >= threshold {
            let j = self.c[p] as usize;
            let start = self.start[j] as usize;
            let size = self.size[j] as usize;
            p = self.f[p] as usize;
            for s in start..start + size - 1 {
                let q = self.f[s] as usize;
                let r = self.b[s] as usize;
                self.b[q] = r as u32;
                self.f[r] = q as u32;
                self.c[self.l[s] as usize] -= 1;
            }
        }
        self.a -= self.c[lit_idx] as usize;
    }

    /// Reverses `deactivate_literal`: splices literals back into occurrence
    /// lists, restores occurrence counts, and increments `a` by `c[lit]`.
    /// Does not modify `d`.
    fn reactivate_literal(&mut self, lit: u32) {
        let threshold = self.threshold();
        let lit_idx = lit as usize;

        self.a += self.c[lit_idx] as usize;
        let mut p = self.b[lit_idx] as usize;
        while p >= threshold {
            let j = self.c[p] as usize;
            let start = self.start[j] as usize;
            let size = self.size[j] as usize;
            p = self.b[p] as usize;
            for s in start..start + size - 1 {
                let q = self.f[s] as usize;
                let r = self.b[s] as usize;
                self.b[q] = s as u32;
                self.f[r] = s as u32;
                self.c[self.l[s] as usize] += 1;
            }
        }
    }

    /// Reverses `remove_false_literal`: walks the occurrence list of `¬lit`
    /// and increments the size of each clause it appears in.
    fn unremove_false_literal(&mut self, lit: u32) {
        let not_lit = (lit ^ 1) as usize;
        let threshold = self.threshold();

        let mut p = self.f[not_lit] as usize;
        while p >= threshold {
            let j = self.c[p] as usize;
            self.size[j] += 1;
            p = self.f[p] as usize;
        }
    }

    /// Runs Algorithm A (TAOCP 4B §7.2.2.2).
    ///
    /// The goto-based algorithm is restructured into three nested loops:
    ///
    /// - `'choose` (outer): re-enters A2 after each successful A4.
    /// - Inner unnamed: retries A3 with a flipped literal (A5) after failure.
    /// - Innermost unnamed: climbs back through exhausted depths (A6-A8).
    fn solve(mut self) -> Option<Vec<bool>> {
        let n = self.n as usize;
        // Move codes m_1..m_n (1-indexed; index 0 unused).
        // Values 0-5 encode the polarity tried and whether the literal is pure.
        let mut moves = vec![0u32; n + 1];

        // A1: Initialize.
        self.d = 1;

        'choose: loop {
            // A2: Choose literal for the current depth.
            let d = self.d;
            let mut l = (2 * d) as u32;
            if self.c[l as usize] <= self.c[(l + 1) as usize] {
                l += 1;
            }
            let is_pure = self.c[(l ^ 1) as usize] == 0;
            moves[d] = (l & 1) + if is_pure { 4 } else { 0 };

            if self.c[l as usize] as usize == self.a {
                // l appears in every active clause → the current partial
                // assignment (plus setting x_d = (l is positive)) satisfies all.
                let mut assignment = vec![false; n];
                for j in 1..=d {
                    assignment[j - 1] = (moves[j] & 1) == 0;
                }
                return Some(assignment);
            }

            // Try to commit literal l, backtracking as needed.
            loop {
                // A3: Remove ¬l from active clauses; fail if any would empty.
                if self.remove_false_literal(l) {
                    // A4: Deactivate clauses satisfied by l, then advance depth.
                    self.deactivate_literal(l);
                    self.d += 1;
                    continue 'choose; // → A2 at the next depth
                }

                // A5: Removal failed.  Try the complementary literal if this
                // depth has not yet exhausted both polarities.
                let d = self.d;
                if moves[d] < 2 {
                    moves[d] = 3 - moves[d];
                    l = (2 * d) as u32 + (moves[d] & 1);
                    continue; // → A3 with the flipped literal
                }

                // Both polarities exhausted at depth d; unwind the stack.
                loop {
                    // A6: If already at the root, the formula is unsatisfiable.
                    if self.d == 1 {
                        return None;
                    }
                    self.d -= 1;
                    let d = self.d;
                    l = (2 * d) as u32 + (moves[d] & 1);

                    // A7: Restore clauses that were satisfied by l.
                    self.reactivate_literal(l);
                    // A8: Restore ¬l in the active clause sizes.
                    self.unremove_false_literal(l);

                    // A5 for this depth: try the other polarity if available.
                    if moves[d] < 2 {
                        moves[d] = 3 - moves[d];
                        l = (2 * d) as u32 + (moves[d] & 1);
                        break; // exit backtrack loop → A3 with the new literal
                    }
                    // This depth is also exhausted; keep climbing.
                }
                // After breaking from the backtrack loop, retry A3 with the
                // new literal chosen at the restored depth.
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make(clauses: &[Vec<u32>]) -> SatProblem {
        SatProblem::from_literals(clauses).unwrap()
    }

    #[test]
    fn test_trivial() {
        let p = SatProblem::from_clauses(&[]);
        assert!(matches!(BasicBacktracking::new(&p), BasicBacktracking::Trivial));
    }

    #[test]
    fn test_unsatisfiable() {
        // Clause::new(&[]) succeeds but gives an empty clause.
        use super::super::Clause;
        let empty = Clause::new(&[]).unwrap();
        let p = SatProblem::from_clauses(&[empty]);
        assert!(matches!(BasicBacktracking::new(&p), BasicBacktracking::Unsatisfiable));
    }

    #[test]
    fn test_single_clause_single_literal() {
        // {x_1} → literals [2], n=1, m=1, total_cells=5
        let p = make(&[vec![2]]);
        let data = match BasicBacktracking::new(&p) {
            BasicBacktracking::Active(d) => d,
            other => panic!("expected Active, got {:?}", other),
        };

        assert_eq!(data.n, 1);
        // total_cells = 2*1+2 + 1 = 5
        assert_eq!(data.l.len(), 5);

        // Clause cell at p=4: l[4]=2, c[4]=1
        assert_eq!(data.l[4], 2);
        assert_eq!(data.c[4], 1);

        // Literal header at p=2: f[2]=4, b[2]=4, c[2]=1
        assert_eq!(data.f[2], 4);
        assert_eq!(data.b[2], 4);
        assert_eq!(data.c[2], 1);

        // Clause cell linked back: f[4]=2, b[4]=2
        assert_eq!(data.f[4], 2);
        assert_eq!(data.b[4], 2);

        assert_eq!(data.start[1], 4);
        assert_eq!(data.size[1], 1);
    }

    fn spec_example_data() -> BasicBacktrackingData {
        let p = make(&[
            vec![2, 4, 7], // {x_1, x_2, ¬x_3}
            vec![4, 6, 9], // {x_2, x_3, ¬x_4}
            vec![2, 6, 8], // {x_1, x_3, x_4}
            vec![3, 4, 8], // {x_2, ¬x_1, x_4}
            vec![3, 5, 6], // {x_3, ¬x_1, ¬x_2}
            vec![5, 7, 8], // {x_4, ¬x_2, ¬x_3}
            vec![3, 7, 9], // {¬x_1, ¬x_3, ¬x_4}
        ]);
        match BasicBacktracking::new(&p) {
            BasicBacktracking::Active(d) => d,
            other => panic!("expected Active, got {:?}", other),
        }
    }

    #[test]
    fn test_spec_example() {
        let data = spec_example_data();

        assert_eq!(data.n, 4);
        assert_eq!(data.l.len(), 31);

        // start(i) = 31 - 3i, size(i) = 3
        for i in 1..=7usize {
            assert_eq!(data.start[i], (31 - 3 * i) as u32, "start[{i}]");
            assert_eq!(data.size[i], 3u32, "size[{i}]");
        }

        let expected_l: &[u32] = &[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0..9
            9, 7, 3, // clause 7  (cells 10-12)
            8, 7, 5, // clause 6  (cells 13-15)
            6, 5, 3, // clause 5  (cells 16-18)
            8, 4, 3, // clause 4  (cells 19-21)
            8, 6, 2, // clause 3  (cells 22-24)
            9, 6, 4, // clause 2  (cells 25-27)
            7, 4, 2, // clause 1  (cells 28-30)
        ];
        assert_eq!(&data.l[10..=30], &expected_l[10..=30], "l[10..=30]");

        let expected_f: &[u32] = &[
            0, 0, 30, 21, 29, 17, 26, 28, 22, 25, // 0..9
            9, 7, 3, 8, 11, 5, 6, 15, 12, 13, 4, 18, 19, 16, 2, 10, 23, 20, 14, 27, 24,
        ];
        assert_eq!(&*data.f, expected_f, "f");

        let expected_b: &[u32] = &[
            0, 0, 24, 12, 20, 15, 16, 11, 13, 10, // 0..9
            25, 14, 18, 19, 28, 17, 23, 5, 21, 22, 27, 3, 8, 26, 30, 9, 6, 29, 7, 4, 2,
        ];
        assert_eq!(&*data.b, expected_b, "b");

        let expected_c: &[u32] = &[
            0, 0, 2, 3, 3, 2, 3, 3, 3, 2, // 0..9
            7, 7, 7, 6, 6, 6, 5, 5, 5, 4, 4, 4, 3, 3, 3, 2, 2, 2, 1, 1, 1,
        ];
        assert_eq!(&*data.c, expected_c, "c");
    }

    #[test]
    fn test_initial_active_clauses_and_depth() {
        let data = spec_example_data();
        assert_eq!(data.a, 7);
        assert_eq!(data.d, 0);
    }

    #[test]
    fn test_remove_false_literal_success() {
        // R′: set x_1=true → remove ¬x_1 (lit 3) from clauses 4, 5, 7.
        // All have initial size 3, so none become empty.
        let mut data = spec_example_data();
        assert!(data.remove_false_literal(2));
        assert_eq!(data.size[4], 2, "size[4]");
        assert_eq!(data.size[5], 2, "size[5]");
        assert_eq!(data.size[7], 2, "size[7]");
        // Unaffected clauses stay at 3.
        assert_eq!(data.size[1], 3, "size[1]");
        assert_eq!(data.size[2], 3, "size[2]");
        assert_eq!(data.size[3], 3, "size[3]");
        assert_eq!(data.size[6], 3, "size[6]");
    }

    #[test]
    fn test_remove_false_literal_no_occurrences() {
        // In R′, literal 2 (x_1) appears in clauses 1, 3; its negation ¬x_1
        // (lit 3) appears in clauses 4, 5, 7.  But if we instead call
        // remove_false_literal on a literal whose negation never appears (e.g.
        // build a problem where that literal has an empty list), all sizes are
        // unchanged and we return true.
        //
        // Simplest: a single-literal problem {x_1}.  Removing ¬x_1 means
        // removing lit 3, which has no clause cells at all in this problem.
        let p = make(&[vec![2]]); // {x_1}
        let mut data = match BasicBacktracking::new(&p) {
            BasicBacktracking::Active(d) => d,
            other => panic!("expected Active, got {:?}", other),
        };
        // ¬x_1 = lit 3 does not appear in any clause → f[3] == 3 (self-loop).
        assert!(data.remove_false_literal(2));
        assert_eq!(data.size[1], 1); // unchanged
    }

    #[test]
    fn test_remove_false_literal_rollback() {
        // Clauses [{¬x_1, x_2}, {¬x_1}] = [{3,4}, {3}].
        // Clause 1 has size 2, clause 2 has size 1.
        // Forward pass visits clause 1 first (higher cell index → earlier in list).
        // Clause 1: size 2 → decremented to 1.
        // Clause 2: size 1 → conflict; rollback restores clause 1 to 2.
        let p = make(&[vec![3, 4], vec![3]]);
        let mut data = match BasicBacktracking::new(&p) {
            BasicBacktracking::Active(d) => d,
            other => panic!("expected Active, got {:?}", other),
        };
        assert!(!data.remove_false_literal(2)); // setting x_1=true fails
        assert_eq!(data.size[1], 2, "size[1] should be restored");
        assert_eq!(data.size[2], 1, "size[2] unchanged");
    }

    #[test]
    fn test_deactivate_literal() {
        // Set x_1=true (lit 2). Clauses 1 and 3 contain x_1.
        // Their other literals are: clause 1 → {7,4}, clause 3 → {8,6}.
        // c[4], c[6], c[7], c[8] should each drop by 1 (from 3 to 2).
        // a drops by c[2]=2 (x_1 appears in 2 clauses); d increments.
        let mut data = spec_example_data();
        data.deactivate_literal(2);

        assert_eq!(data.a, 5, "a");
        assert_eq!(data.d, 0, "d unchanged by deactivate_literal");
        assert_eq!(data.c[4], 2, "c[4] (x_2)");
        assert_eq!(data.c[6], 2, "c[6] (x_3)");
        assert_eq!(data.c[7], 2, "c[7] (¬x_3)");
        assert_eq!(data.c[8], 2, "c[8] (x_4)");

        // Cells 28 (l=7) and 29 (l=4) of clause 1 must be spliced out.
        // For cell 28: b[f[28]] and f[b[28]] no longer point to 28.
        assert_ne!(data.b[data.f[28] as usize], 28, "cell 28 still in list");
        assert_ne!(data.f[data.b[28] as usize], 28, "cell 28 still in list");
        // For cell 29: same check.
        assert_ne!(data.b[data.f[29] as usize], 29, "cell 29 still in list");
        assert_ne!(data.f[data.b[29] as usize], 29, "cell 29 still in list");
        // Cells 22 (l=8) and 23 (l=6) of clause 3 must be spliced out.
        assert_ne!(data.b[data.f[22] as usize], 22, "cell 22 still in list");
        assert_ne!(data.f[data.b[22] as usize], 22, "cell 22 still in list");
        assert_ne!(data.b[data.f[23] as usize], 23, "cell 23 still in list");
        assert_ne!(data.f[data.b[23] as usize], 23, "cell 23 still in list");
    }

    #[test]
    fn test_reactivate_restores_state() {
        // Deactivate then immediately reactivate should restore all state.
        let original = spec_example_data();
        let mut data = spec_example_data();
        data.deactivate_literal(2);
        data.reactivate_literal(2);

        assert_eq!(data.a, original.a, "a");
        assert_eq!(data.c[4], original.c[4], "c[4]");
        assert_eq!(data.c[6], original.c[6], "c[6]");
        assert_eq!(data.c[7], original.c[7], "c[7]");
        assert_eq!(data.c[8], original.c[8], "c[8]");
        // Cells 28, 29, 22, 23 must be back in their lists.
        assert_eq!(data.b[data.f[28] as usize], 28, "cell 28 not restored");
        assert_eq!(data.f[data.b[28] as usize], 28, "cell 28 not restored");
        assert_eq!(data.b[data.f[29] as usize], 29, "cell 29 not restored");
        assert_eq!(data.f[data.b[29] as usize], 29, "cell 29 not restored");
        assert_eq!(data.b[data.f[22] as usize], 22, "cell 22 not restored");
        assert_eq!(data.f[data.b[22] as usize], 22, "cell 22 not restored");
        assert_eq!(data.b[data.f[23] as usize], 23, "cell 23 not restored");
        assert_eq!(data.f[data.b[23] as usize], 23, "cell 23 not restored");
    }

    #[test]
    fn test_unremove_false_literal() {
        // After remove_false_literal(2) sizes of clauses 4, 5, 7 are 2.
        // unremove_false_literal(2) should restore them to 3.
        let mut data = spec_example_data();
        assert!(data.remove_false_literal(2));
        data.unremove_false_literal(2);
        assert_eq!(data.size[4], 3, "size[4]");
        assert_eq!(data.size[5], 3, "size[5]");
        assert_eq!(data.size[7], 3, "size[7]");
        // Unaffected clauses unchanged.
        assert_eq!(data.size[1], 3, "size[1]");
        assert_eq!(data.size[2], 3, "size[2]");
        assert_eq!(data.size[3], 3, "size[3]");
        assert_eq!(data.size[6], 3, "size[6]");
    }

    // ---------------------------------------------------------------------------
    // solve() — Algorithm A
    // ---------------------------------------------------------------------------

    #[test]
    fn test_solve_trivial() {
        let p = SatProblem::from_clauses(&[]);
        let bb = BasicBacktracking::new(&p);
        assert_eq!(bb.solve(), Some(vec![]));
    }

    #[test]
    fn test_solve_empty_clause() {
        use super::super::Clause;
        let empty = Clause::new(&[]).unwrap();
        let p = SatProblem::from_clauses(&[empty]);
        let bb = BasicBacktracking::new(&p);
        assert_eq!(bb.solve(), None);
    }

    #[test]
    fn test_solve_single_variable_sat() {
        // {x_1}: one clause, one variable.
        let p = make(&[vec![2]]);
        let bb = BasicBacktracking::new(&p);
        let assignment = bb.solve().expect("satisfiable");
        assert_eq!(assignment.len(), 1);
        assert!(p.is_satisfied(&assignment));
    }

    #[test]
    fn test_solve_r_prime() {
        // R' is satisfiable.  Algorithm A finds m_1m_2m_3m_4 = 1014, giving
        // x_1x_2x_3x_4 = 0101 = [false, true, false, true].
        let p = make(&[
            vec![2, 4, 7], // x_1 ∨  x_2 ∨ ¬x_3
            vec![4, 6, 9], // x_2 ∨  x_3 ∨ ¬x_4
            vec![2, 6, 8], // x_1 ∨  x_3 ∨  x_4
            vec![3, 4, 8], // ¬x_1 ∨  x_2 ∨  x_4
            vec![3, 5, 6], // ¬x_1 ∨ ¬x_2 ∨  x_3
            vec![5, 7, 8], // ¬x_2 ∨ ¬x_3 ∨  x_4
            vec![3, 7, 9], // ¬x_1 ∨ ¬x_3 ∨ ¬x_4
        ]);
        let bb = BasicBacktracking::new(&p);
        let assignment = bb.solve().expect("R' is satisfiable");
        assert_eq!(assignment, vec![false, true, false, true]);
        assert!(p.is_satisfied(&assignment));
    }

    #[test]
    fn test_solve_r_prime_unsat() {
        // R' augmented with {¬x_4, x_1, ¬x_2} is unsatisfiable (Knuth ex. 58).
        let p = make(&[
            vec![2, 4, 7],
            vec![4, 6, 9],
            vec![2, 6, 8],
            vec![3, 4, 8],
            vec![3, 5, 6],
            vec![5, 7, 8],
            vec![3, 7, 9],
            vec![2, 5, 9], // x_1 ∨ ¬x_2 ∨ ¬x_4
        ]);
        let bb = BasicBacktracking::new(&p);
        assert_eq!(bb.solve(), None);
    }
}
