// DPLL with watched literals and an active variable ring (Algorithm D)
// from TAOCP 4B §7.2.2.2.

use super::super::SatProblem;

const MOVE_TRY_TRUE_FIRST: u8 = 0;
const MOVE_TRY_FALSE_FIRST: u8 = 1;
const MOVE_FORCED_TRUE: u8 = 4;
const MOVE_FORCED_FALSE: u8 = 5;

pub fn solve_via_dpll_codex(problem: &SatProblem) -> Option<Vec<bool>> {
    DpllCodex::new(problem).solve()
}

#[derive(Debug)]
struct DpllCodexData {
    n: usize,
    l: Box<[u32]>,
    start: Box<[u32]>,
    link: Box<[u32]>,
    w: Box<[u32]>,
}

#[derive(Debug)]
enum DpllCodex {
    Trivial,
    Unsatisfiable,
    Active(DpllCodexData),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ForcedChoice {
    None,
    ForceTrue,
    ForceFalse,
    Conflict,
}

#[derive(Debug, Clone, Copy)]
enum Undo {
    L(usize, u32),
    Link(usize, u32),
    W(usize, u32),
    Next(usize, usize),
    T(usize),
    X(usize, i8),
}

#[derive(Debug, Clone, Copy)]
struct Choice {
    predecessor: usize,
    var: usize,
    first_move: u8,
    second_move: Option<u8>,
}

impl DpllCodex {
    fn solve(self) -> Option<Vec<bool>> {
        match self {
            DpllCodex::Trivial => Some(vec![]),
            DpllCodex::Unsatisfiable => None,
            DpllCodex::Active(data) => data.solve(),
        }
    }

    fn new(problem: &SatProblem) -> Self {
        let clauses = problem.clauses();
        let m = clauses.len();

        if m == 0 {
            return DpllCodex::Trivial;
        }

        if clauses.iter().any(|c| c.is_empty()) {
            return DpllCodex::Unsatisfiable;
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
            // literal starts at start[j].
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

        DpllCodex::Active(DpllCodexData {
            n,
            l: l.into_boxed_slice(),
            start: start.into_boxed_slice(),
            link: link.into_boxed_slice(),
            w: w.into_boxed_slice(),
        })
    }
}

impl DpllCodexData {
    #[inline]
    fn literal_is_false(lit: u32, x: &[i8]) -> bool {
        let var = (lit / 2) as usize;
        x[var] >= 0 && (x[var] as u32) == (lit & 1)
    }

    fn init_active_ring(&self) -> (Vec<usize>, usize) {
        let mut next = vec![0usize; self.n + 1];
        let mut head = 0usize;
        let mut tail = 0usize;

        for k in (1..=self.n).rev() {
            if self.w[2 * k] != 0 || self.w[2 * k + 1] != 0 {
                next[k] = head;
                head = k;
                if tail == 0 {
                    tail = k;
                }
            }
        }

        if tail != 0 {
            next[tail] = head;
        }

        (next, tail)
    }

    fn check_unit_literal(&self, lit: u32, x: &[i8]) -> bool {
        let mut j = self.w[lit as usize];
        while j != 0 {
            let clause = j as usize;
            let i = self.start[clause] as usize;
            let end = self.start[clause - 1] as usize;

            let mut p = i + 1;
            while p < end && Self::literal_is_false(self.l[p], x) {
                p += 1;
            }
            if p == end {
                return true;
            }

            j = self.link[clause];
        }
        false
    }

    fn forced_choice_for_var(&self, k: usize, x: &[i8]) -> ForcedChoice {
        let pos_unit = self.check_unit_literal((2 * k) as u32, x);
        let neg_unit = self.check_unit_literal((2 * k + 1) as u32, x);

        match (pos_unit, neg_unit) {
            (true, true) => ForcedChoice::Conflict,
            (true, false) => ForcedChoice::ForceTrue,
            (false, true) => ForcedChoice::ForceFalse,
            (false, false) => ForcedChoice::None,
        }
    }

    // Clears the watch list for a literal known to be false under the current
    // assignment. Clauses are rewired to watch another non-false literal.
    fn clear_watch_list(
        &mut self,
        false_lit: u32,
        x: &[i8],
        next: &mut [usize],
        t: &mut usize,
        trail: &mut Vec<Undo>,
    ) -> bool {
        fn set_l(this: &mut DpllCodexData, idx: usize, val: u32, trail: &mut Vec<Undo>) {
            trail.push(Undo::L(idx, this.l[idx]));
            this.l[idx] = val;
        }
        fn set_link(this: &mut DpllCodexData, idx: usize, val: u32, trail: &mut Vec<Undo>) {
            trail.push(Undo::Link(idx, this.link[idx]));
            this.link[idx] = val;
        }
        fn set_w(this: &mut DpllCodexData, idx: usize, val: u32, trail: &mut Vec<Undo>) {
            trail.push(Undo::W(idx, this.w[idx]));
            this.w[idx] = val;
        }
        fn set_next(next: &mut [usize], idx: usize, val: usize, trail: &mut Vec<Undo>) {
            trail.push(Undo::Next(idx, next[idx]));
            next[idx] = val;
        }
        fn set_t(t: &mut usize, val: usize, trail: &mut Vec<Undo>) {
            trail.push(Undo::T(*t));
            *t = val;
        }

        let idx = false_lit as usize;
        let mut j = self.w[idx];
        set_w(self, idx, 0, trail);

        while j != 0 {
            let clause = j as usize;
            let j_next = self.link[clause];
            let i = self.start[clause] as usize;
            let end = self.start[clause - 1] as usize;

            let mut p = i + 1;
            while p < end && Self::literal_is_false(self.l[p], x) {
                p += 1;
            }
            if p == end {
                set_w(self, idx, j, trail);
                return false;
            }

            let lit_prime = self.l[p];
            set_l(self, p, false_lit, trail);
            set_l(self, i, lit_prime, trail);

            let lit_prime_idx = lit_prime as usize;
            let old_head = self.w[lit_prime_idx];
            let other_head = self.w[lit_prime_idx ^ 1];
            let var = (lit_prime / 2) as usize;

            // Variable becomes newly active when it is unset and both watch
            // lists were previously empty.
            if old_head == 0 && other_head == 0 && x[var] < 0 {
                if *t == 0 {
                    set_t(t, var, trail);
                    set_next(next, var, var, trail);
                } else {
                    let h = next[*t];
                    set_next(next, var, h, trail);
                    set_next(next, *t, var, trail);
                }
            }

            set_link(self, clause, old_head, trail);
            set_w(self, lit_prime_idx, j, trail);
            j = j_next;
        }

        true
    }

    fn undo_to(
        &mut self,
        trail: &mut Vec<Undo>,
        checkpoint: usize,
        x: &mut [i8],
        next: &mut [usize],
        t: &mut usize,
    ) {
        while trail.len() > checkpoint {
            match trail.pop().expect("trail underflow") {
                Undo::L(idx, old) => self.l[idx] = old,
                Undo::Link(idx, old) => self.link[idx] = old,
                Undo::W(idx, old) => self.w[idx] = old,
                Undo::Next(idx, old) => next[idx] = old,
                Undo::T(old) => *t = old,
                Undo::X(idx, old) => x[idx] = old,
            }
        }
    }

    fn choose(&self, x: &[i8], next: &[usize], t: usize) -> Option<Choice> {
        let mut predecessor = t;
        let mut h = next[predecessor];
        loop {
            match self.forced_choice_for_var(h, x) {
                ForcedChoice::Conflict => return None,
                ForcedChoice::ForceTrue => {
                    return Some(Choice {
                        predecessor,
                        var: h,
                        first_move: MOVE_FORCED_TRUE,
                        second_move: None,
                    });
                }
                ForcedChoice::ForceFalse => {
                    return Some(Choice {
                        predecessor,
                        var: h,
                        first_move: MOVE_FORCED_FALSE,
                        second_move: None,
                    });
                }
                ForcedChoice::None => {
                    if h == t {
                        break;
                    }
                    predecessor = h;
                    h = next[predecessor];
                }
            }
        }

        let var = next[t];
        let first = if self.w[2 * var] == 0 || self.w[2 * var + 1] != 0 {
            MOVE_TRY_FALSE_FIRST
        } else {
            MOVE_TRY_TRUE_FIRST
        };

        Some(Choice {
            predecessor: t,
            var,
            first_move: first,
            second_move: Some(3 - first),
        })
    }

    fn assignment_satisfies_formula(&self, assignment: &[bool]) -> bool {
        let m = self.start.len() - 1;
        for clause in 1..=m {
            let start = self.start[clause] as usize;
            let end = self.start[clause - 1] as usize;
            let mut satisfied = false;
            for p in start..end {
                let lit = self.l[p];
                let var = (lit / 2) as usize;
                if var == 0 || var > assignment.len() {
                    continue;
                }
                let val = assignment[var - 1];
                if (lit & 1) == 0 {
                    if val {
                        satisfied = true;
                        break;
                    }
                } else if !val {
                    satisfied = true;
                    break;
                }
            }
            if !satisfied {
                return false;
            }
        }
        true
    }

    fn assignment_from_x(&self, x: &[i8]) -> Vec<bool> {
        let mut assignment = vec![false; self.n];
        for var in 1..=self.n {
            assignment[var - 1] = x[var] == 1;
        }
        assignment
    }

    fn apply_move(
        &mut self,
        x: &mut [i8],
        next: &mut [usize],
        t: &mut usize,
        trail: &mut Vec<Undo>,
        predecessor: usize,
        var: usize,
        mv: u8,
    ) -> bool {
        if next[var] == var {
            trail.push(Undo::T(*t));
            *t = 0;
        } else {
            trail.push(Undo::Next(predecessor, next[predecessor]));
            next[predecessor] = next[var];
            trail.push(Undo::T(*t));
            *t = predecessor;
        }

        let b = ((mv + 1) & 1) as i8;
        trail.push(Undo::X(var, x[var]));
        x[var] = b;
        let false_lit = (2 * var) as u32 + b as u32;
        self.clear_watch_list(false_lit, x, next, t, trail)
    }

    fn solve(mut self) -> Option<Vec<bool>> {
        let n = self.n;
        let mut x = vec![-1i8; n + 1];
        let (mut next, mut t) = self.init_active_ring();

        let mut trail = Vec::new();
        let mut depth = 0usize;
        let mut choice_by_depth = vec![None::<Choice>; n + 1];
        let mut checkpoint_by_depth = vec![0usize; n + 1];
        let mut used_second_by_depth = vec![false; n + 1];

        'search: loop {
            if t == 0 {
                let assignment = self.assignment_from_x(&x);
                if self.assignment_satisfies_formula(&assignment) {
                    return Some(assignment);
                }
            }

            let choice = if t == 0 {
                None
            } else {
                self.choose(&x, &next, t)
            };

            if let Some(choice) = choice {
                depth += 1;
                choice_by_depth[depth] = Some(choice);
                checkpoint_by_depth[depth] = trail.len();
                used_second_by_depth[depth] = false;

                if self.apply_move(
                    &mut x,
                    &mut next,
                    &mut t,
                    &mut trail,
                    choice.predecessor,
                    choice.var,
                    choice.first_move,
                ) {
                    continue 'search;
                }

                self.undo_to(
                    &mut trail,
                    checkpoint_by_depth[depth],
                    &mut x,
                    &mut next,
                    &mut t,
                );
            }

            loop {
                if depth == 0 {
                    return None;
                }

                let choice = choice_by_depth[depth].expect("choice must exist");
                self.undo_to(
                    &mut trail,
                    checkpoint_by_depth[depth],
                    &mut x,
                    &mut next,
                    &mut t,
                );

                if !used_second_by_depth[depth] {
                    if let Some(second) = choice.second_move {
                        used_second_by_depth[depth] = true;
                        if self.apply_move(
                            &mut x,
                            &mut next,
                            &mut t,
                            &mut trail,
                            choice.predecessor,
                            choice.var,
                            second,
                        ) {
                            continue 'search;
                        }

                        self.undo_to(
                            &mut trail,
                            checkpoint_by_depth[depth],
                            &mut x,
                            &mut next,
                            &mut t,
                        );
                    }
                }

                choice_by_depth[depth] = None;
                depth -= 1;
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
        assert!(matches!(DpllCodex::new(&p), DpllCodex::Trivial));
    }

    #[test]
    fn test_new_unsatisfiable() {
        use crate::sat::Clause;
        let empty = Clause::new(&[]).unwrap();
        let p = SatProblem::from_clauses(&[empty]);
        assert!(matches!(DpllCodex::new(&p), DpllCodex::Unsatisfiable));
    }

    #[test]
    fn test_new_r_prime_layout() {
        let data = match DpllCodex::new(&r_prime()) {
            DpllCodex::Active(data) => data,
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
    }

    #[test]
    fn test_init_active_ring_r_prime() {
        let data = match DpllCodex::new(&r_prime()) {
            DpllCodex::Active(data) => data,
            _ => panic!("expected active"),
        };

        let (next, t) = data.init_active_ring();
        assert_eq!(t, 4);
        assert_eq!(next[4], 3);
        assert_eq!(next[3], 4);
    }

    #[test]
    fn test_check_unit_literal_detects_unit() {
        let data = match DpllCodex::new(&make(&[vec![2, 4], vec![3]])) {
            DpllCodex::Active(data) => data,
            _ => panic!("expected active"),
        };

        let x = vec![-1, -1, -1];
        assert!(data.check_unit_literal(3, &x));
        assert!(!data.check_unit_literal(2, &x));
    }

    #[test]
    fn test_forced_choice_conflict() {
        let data = match DpllCodex::new(&make(&[vec![2], vec![3]])) {
            DpllCodex::Active(data) => data,
            _ => panic!("expected active"),
        };

        let x = vec![-1, -1];
        assert_eq!(data.forced_choice_for_var(1, &x), ForcedChoice::Conflict);
    }

    #[test]
    fn test_solve_r_prime() {
        let p = r_prime();
        let assignment = solve_via_dpll_codex(&p).expect("R' is satisfiable");
        assert!(p.is_satisfied(&assignment));
    }

    #[test]
    fn test_solve_r_prime_unsat() {
        let p = r_prime_unsat();
        assert_eq!(solve_via_dpll_codex(&p), None);
    }

    #[test]
    fn test_solve_prefers_forced_move_when_unit_exists() {
        let p = make(&[vec![2], vec![2, 4]]);
        let assignment = solve_via_dpll_codex(&p).expect("expected satisfiable");
        assert!(assignment[0]);
        assert!(p.is_satisfied(&assignment));
    }

    #[test]
    fn test_apply_move_keeps_ring_when_removing_tail() {
        let mut data = match DpllCodex::new(&r_prime()) {
            DpllCodex::Active(data) => data,
            _ => panic!("expected active"),
        };

        let mut x = vec![-1i8; data.n + 1];
        let (mut next, mut t) = data.init_active_ring();
        let mut trail = Vec::new();

        assert_eq!(t, 4);
        assert_eq!(next[4], 3);
        assert_eq!(next[3], 4);

        assert!(data.apply_move(&mut x, &mut next, &mut t, &mut trail, 3, 4, MOVE_TRY_TRUE_FIRST));
        assert_eq!(t, 3);
        assert_eq!(next[3], 3);
    }

    // W(3,3) = 9: waerden(3,3,n) is SAT for n < 9, UNSAT for n >= 9.
    #[test]
    fn test_solve_waerden_sat() {
        use crate::sat::sample_problems::waerden;
        for n in 1u8..=8 {
            let p = waerden(3, 3, n).unwrap();
            let assignment = solve_via_dpll_codex(&p)
                .unwrap_or_else(|| panic!("waerden(3,3,{n}) should be satisfiable"));
            assert!(p.is_satisfied(&assignment), "invalid solution at n={n}");
        }
    }

    #[test]
    fn test_solve_waerden_unsat() {
        use crate::sat::sample_problems::waerden;
        for n in 9u8..=11 {
            let p = waerden(3, 3, n).unwrap();
            assert!(
                solve_via_dpll_codex(&p).is_none(),
                "waerden(3,3,{n}) should be unsatisfiable"
            );
        }
    }

    #[test]
    fn test_solve_langford_sat() {
        use crate::sat::sample_problems::langford;
        for n in [3, 4, 7] {
            let p = langford(n).unwrap();
            let res = solve_via_dpll_codex(&p);
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
            let res = solve_via_dpll_codex(&p);
            assert!(res.is_none(), "langford({n}) should be unsatisfiable");
        }
    }
}
