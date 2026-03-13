use super::super::SatProblem;

// Algorithm D move codes:
// 0: trying x = 1 first
// 1: trying x = 0 first
// 2: trying x = 1 after x = 0 failed
// 3: trying x = 0 after x = 1 failed
// 4: trying x = 1 forced
// 5: trying x = 0 forced

/// Attempts to solve the specified SAT problem using DPLL (Algorithm D).
///
/// If a solution is found, returns an assignment (as a Vec<bool>) that
/// satisfies the problem. If no solution is found, returns None.
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
    next: Box<[u32]>,
    h: u32,
    t: u32,
    x: Box<[Option<bool>]>,
}

#[derive(Debug)]
enum Dpll {
    Trivial,
    Unsatisfiable,
    Active(DpllData),
}

impl Dpll {
    fn solve(self) -> Option<Vec<bool>> {
        match self {
            Dpll::Trivial => Some(vec![]),
            Dpll::Unsatisfiable => None,
            Dpll::Active(data) => data.solve(),
        }
    }

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

        let mut next = vec![0u32; n + 1];
        let x = vec![None; n + 1];
        let mut h = 0u32;
        let mut t = 0u32;

        for k in (1..=n).rev() {
            if w[2 * k] != 0 || w[2 * k + 1] != 0 {
                next[k] = h;
                h = k as u32;
                if t == 0 {
                    t = k as u32;
                }
            }
        }
        if t != 0 {
            next[t as usize] = h;
        }

        Dpll::Active(DpllData {
            n,
            l: l.into_boxed_slice(),
            start: start.into_boxed_slice(),
            link: link.into_boxed_slice(),
            w: w.into_boxed_slice(),
            next: next.into_boxed_slice(),
            h,
            t,
            x: x.into_boxed_slice(),
        })
    }
}

impl DpllData {
    #[inline]
    fn is_literal_false(&self, lit: u32) -> bool {
        let var = (lit / 2) as usize;
        if let Some(val) = self.x[var] {
            (val as u32) == (lit & 1)
        } else {
            false
        }
    }

    fn check_unit_clauses(&self, l: u32) -> bool {
        let mut j = self.w[l as usize];
        while j != 0 {
            let i = self.start[j as usize] as usize;
            let end = self.start[(j - 1) as usize] as usize;
            let mut p = i + 1;
            let mut all_false = true;
            while p < end {
                if !self.is_literal_false(self.l[p]) {
                    all_false = false;
                    break;
                }
                p += 1;
            }
            if all_false {
                return true;
            }
            j = self.link[j as usize];
        }
        false
    }

    fn clear_watch_lists(&mut self, k: usize, b: u32) {
        let l = (2 * k) as u32 + b;
        let mut j = self.w[l as usize];
        self.w[l as usize] = 0;

        while j != 0 {
            let j_prime = self.link[j as usize];
            let i = self.start[j as usize] as usize;
            let end = self.start[(j - 1) as usize] as usize;
            let mut p = i + 1;

            while p < end && self.is_literal_false(self.l[p]) {
                p += 1;
            }

            if p >= end {
                panic!("Algorithm D invariant violated: clause {} became empty", j);
            }

            let l_prime = self.l[p];
            self.l[p] = l;
            self.l[i] = l_prime;

            let p_val = self.w[l_prime as usize];
            let q_val = self.w[(l_prime ^ 1) as usize];
            let var_l_prime = (l_prime / 2) as usize;

            if p_val == 0 && q_val == 0 && self.x[var_l_prime].is_none() {
                let lp_var = var_l_prime as u32;
                if self.t == 0 {
                    self.h = lp_var;
                    self.t = lp_var;
                    self.next[self.t as usize] = self.h;
                } else {
                    self.next[lp_var as usize] = self.h;
                    self.h = lp_var;
                    self.next[self.t as usize] = self.h;
                }
            }

            self.link[j as usize] = p_val;
            self.w[l_prime as usize] = j;

            j = j_prime;
        }
    }

    fn solve(mut self) -> Option<Vec<bool>> {
        let n = self.n;
        let mut m = vec![0u8; n + 1];
        let mut h_hist = vec![0usize; n + 1];
        let mut d = 0usize;

        loop {
            // D2. [Success?]
            if self.t == 0 {
                let mut assignment = vec![false; n];
                for i in 1..=n {
                    assignment[i - 1] = self.x[i].unwrap_or(false);
                }
                return Some(assignment);
            }

            // D3. [Look for unit clauses.]
            let mut k_search = self.t as usize;
            let m_next = loop {
                let h_var = self.next[k_search] as usize;
                let unit_pos = self.check_unit_clauses(2 * h_var as u32);
                let unit_neg = self.check_unit_clauses(2 * h_var as u32 + 1);

                let f = (if unit_pos { 1 } else { 0 }) + (if unit_neg { 2 } else { 0 });

                if f == 3 {
                    // Conflict found
                    break Err(k_search);
                }
                if f != 0 {
                    // Forced move
                    self.h = h_var as u32;
                    self.t = k_search as u32;
                    break Ok(f + 3);
                }

                if h_var == self.t as usize {
                    // D4. [Two-way branch.]
                    self.h = self.next[self.t as usize];
                    let hv = self.h as usize;
                    break Ok(if self.w[2 * hv] == 0 || self.w[2 * hv + 1] != 0 {
                        1
                    } else {
                        0
                    });
                }
                k_search = h_var;
            };

            match m_next {
                Ok(mc) => {
                    // D5. [Move on.]
                    d += 1;
                    m[d] = mc;
                    h_hist[d] = self.h as usize;

                    // Remove current_k from ring
                    let current_k = self.h as usize;
                    if self.t as usize == current_k {
                        self.t = 0;
                    } else {
                        self.h = self.next[current_k];
                        self.next[self.t as usize] = self.h;
                    }
                }
                Err(k_conflict) => {
                    // D7. [Backtrack.]
                    self.t = k_conflict as u32;
                    self.h = self.next[self.t as usize];
                    while d > 0 && m[d] >= 2 {
                        let prev_k = h_hist[d];
                        self.x[prev_k] = None;
                        if self.w[2 * prev_k] != 0 || self.w[2 * prev_k + 1] != 0 {
                            self.next[prev_k] = self.h;
                            self.h = prev_k as u32;
                            self.next[self.t as usize] = self.h;
                        }
                        d -= 1;
                    }

                    if d > 0 {
                        m[d] = 3 - m[d];
                    } else {
                        return None;
                    }
                }
            }

            // D6. [Update watches.]
            let current_k = h_hist[d];
            let b = (m[d] + 1) % 2;
            self.x[current_k] = Some(b == 1);
            self.clear_watch_lists(current_k, b as u32);
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
        assert!(matches!(Dpll::new(&p), Dpll::Trivial));
    }

    #[test]
    fn test_new_unsatisfiable() {
        use crate::sat::Clause;
        let empty = Clause::new(&[]).unwrap();
        let p = SatProblem::from_clauses(&[empty]);
        assert!(matches!(Dpll::new(&p), Dpll::Unsatisfiable));
    }

    #[test]
    fn test_solve_r_prime() {
        let p = r_prime();
        let assignment = solve_via_dpll(&p).expect("R' is satisfiable");
        assert!(p.is_satisfied(&assignment));
    }

    #[test]
    fn test_solve_r_prime_unsat() {
        let p = r_prime_unsat();
        assert_eq!(solve_via_dpll(&p), None);
    }

    // W(3,3) = 9: waerden(3,3,n) is SAT for n < 9, UNSAT for n >= 9.
    #[test]
    fn test_solve_waerden_sat() {
        use crate::sat::sample_problems::waerden;
        for n in 1u8..=8 {
            let p = waerden(3, 3, n).unwrap();
            let assignment = solve_via_dpll(&p)
                .unwrap_or_else(|| panic!("waerden(3,3,{n}) should be satisfiable"));
            assert!(p.is_satisfied(&assignment), "invalid solution at n={n}");
        }
    }

    #[test]
    fn test_solve_waerden_unsat() {
        use crate::sat::sample_problems::waerden;
        for n in 9u8..=11 {
            let p = waerden(3, 3, n).unwrap();
            let res = solve_via_dpll(&p);
            if let Some(ref assignment) = res {
                println!("n={} assignment: {:?}", n, assignment);
                assert!(p.is_satisfied(assignment), "invalid solution at n={n}");
            }
            assert!(
                res.is_none(),
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
