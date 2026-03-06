use std::fmt;

use super::{Clause, ClauseError, SatProblem, SatProblemError};

#[derive(Debug, PartialEq, Eq)]
pub struct SampleProblemError(pub String);

impl SampleProblemError {
    fn invalid<S: Into<String>>(message: S) -> Self {
        SampleProblemError(message.into())
    }
}

impl From<&str> for SampleProblemError {
    fn from(value: &str) -> Self {
        SampleProblemError(value.into())
    }
}

impl From<String> for SampleProblemError {
    fn from(value: String) -> Self {
        SampleProblemError(value)
    }
}

impl From<SatProblemError> for SampleProblemError {
    fn from(value: SatProblemError) -> Self {
        SampleProblemError(value.0)
    }
}

impl From<ClauseError> for SampleProblemError {
    fn from(value: ClauseError) -> Self {
        SampleProblemError(value.0)
    }
}

impl fmt::Display for SampleProblemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn exact_one(vars: &[u32], clauses: &mut Vec<Clause>) -> Result<(), SampleProblemError> {
    clauses.push(Clause::new(vars)?);
    for i in 0..vars.len() {
        for j in i + 1..vars.len() {
            clauses.push(Clause::new(&[vars[i] ^ 1, vars[j] ^ 1])?);
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LangfordOption {
    digit: u8,
    p: u16, // 1-indexed position
    q: u16, // 1-indexed position
}

fn langford_options(n: u8) -> Vec<LangfordOption> {
    let n_u16 = n as u16;
    let two_n = 2 * n_u16;
    let mut options = Vec::new();

    let n_sym = n - if n & 1 == 0 { 1 } else { 0 };
    for k in 1..=n {
        let full_max_start = two_n.saturating_sub(k as u16 + 1);
        let max_start = if k == n_sym {
            n_u16 / 2
        } else {
            full_max_start
        };

        if max_start == 0 {
            continue;
        }

        for p in 1..=max_start {
            let q = p + k as u16 + 1;
            if q <= two_n {
                options.push(LangfordOption { digit: k, p, q });
            }
        }
    }

    options
}

pub fn waerden(j: u8, k: u8, n: u8) -> Result<SatProblem, SampleProblemError> {
    if j == 0 || k == 0 || n == 0 {
        return Err(SampleProblemError::invalid(format!(
            "invalid waerden parameters: j={j}, k={k}, n={n}; all must be > 0"
        )));
    }

    let mut clauses = Vec::new();
    let n_u16 = n as u16;

    // The formal set definition uses d >= 1. For length-1 progressions this
    // would generate infinitely many duplicate unit clauses, so we canonicalize
    // to one clause per starting index.
    if j == 1 {
        for i in 1..=n_u16 {
            clauses.push(Clause::new(&[2 * i as u32])?);
        }
    } else {
        let j_u16 = j as u16;
        for d in 1..=n_u16 {
            let span = (j_u16 - 1) * d;
            if span >= n_u16 {
                break;
            }
            for i in 1..=(n_u16 - span) {
                let lits: Vec<u32> = (0..j_u16)
                    .map(|t| 2 * (i + t * d) as u32)
                    .collect();
                clauses.push(Clause::new(&lits)?);
            }
        }
    }

    if k == 1 {
        for i in 1..=n_u16 {
            clauses.push(Clause::new(&[2 * i as u32 + 1])?);
        }
    } else {
        let k_u16 = k as u16;
        for d in 1..=n_u16 {
            let span = (k_u16 - 1) * d;
            if span >= n_u16 {
                break;
            }
            for i in 1..=(n_u16 - span) {
                let lits: Vec<u32> = (0..k_u16)
                    .map(|t| 2 * (i + t * d) as u32 + 1)
                    .collect();
                clauses.push(Clause::new(&lits)?);
            }
        }
    }

    Ok(SatProblem::new(clauses))
}

pub fn langford(n: u8) -> Result<SatProblem, SampleProblemError> {
    if n == 0 {
        return Err(SampleProblemError::invalid(
            "invalid langford parameter: n must be > 0",
        ));
    }

    let two_n = 2 * n as usize;
    let mut clauses = Vec::new();
    let mut by_digit: Vec<Vec<u32>> = vec![Vec::new(); n as usize + 1];
    let mut by_slot: Vec<Vec<u32>> = vec![Vec::new(); two_n + 1];

    for (idx, opt) in langford_options(n).iter().enumerate() {
        let var_lit = 2 * (idx as u32 + 1);
        by_digit[opt.digit as usize].push(var_lit);
        by_slot[opt.p as usize].push(var_lit);
        by_slot[opt.q as usize].push(var_lit);
    }

    for vars in by_digit.iter().skip(1) {
        exact_one(vars, &mut clauses)?;
    }

    for vars in by_slot.iter().skip(1) {
        exact_one(vars, &mut clauses)?;
    }

    Ok(SatProblem::new(clauses))
}

pub fn waerden_solution_string(
    n: u8,
    solution: Option<&[bool]>,
) -> Result<Option<String>, SampleProblemError> {
    if n == 0 {
        return Err(SampleProblemError::invalid(
            "invalid waerden parameter: n must be > 0",
        ));
    }

    let Some(assignment) = solution else {
        return Ok(None);
    };

    if assignment.len() < n as usize {
        return Err(SampleProblemError::invalid(format!(
            "assignment too short for waerden: expected at least {n}, got {}",
            assignment.len()
        )));
    }

    let mut out = String::with_capacity(n as usize);
    for &bit in assignment.iter().take(n as usize) {
        out.push(if bit { '1' } else { '0' });
    }

    Ok(Some(out))
}

pub fn langford_solution_arrangement(
    n: u8,
    solution: Option<&[bool]>,
) -> Result<Option<Vec<u8>>, SampleProblemError> {
    if n == 0 {
        return Err(SampleProblemError::invalid(
            "invalid langford parameter: n must be > 0",
        ));
    }

    let Some(assignment) = solution else {
        return Ok(None);
    };

    let options = langford_options(n);
    if assignment.len() < options.len() {
        return Err(SampleProblemError::invalid(format!(
            "assignment too short for langford: expected at least {}, got {}",
            options.len(),
            assignment.len()
        )));
    }

    let mut arrangement = vec![0u8; 2 * n as usize];
    let mut chosen_for_digit = vec![0u8; n as usize + 1];
    for (idx, &is_true) in assignment.iter().enumerate().take(options.len()) {
        if !is_true {
            continue;
        }

        let opt = options[idx];
        if chosen_for_digit[opt.digit as usize] != 0 {
            return Err(SampleProblemError::invalid(format!(
                "invalid langford solution: digit {} chosen multiple times",
                opt.digit
            )));
        }
        chosen_for_digit[opt.digit as usize] = 1;

        let p = opt.p as usize - 1;
        let q = opt.q as usize - 1;
        if arrangement[p] != 0 || arrangement[q] != 0 {
            return Err(SampleProblemError::invalid(
                "invalid langford solution: overlapping placements",
            ));
        }
        arrangement[p] = opt.digit;
        arrangement[q] = opt.digit;
    }

    for digit in 1..=n as usize {
        if chosen_for_digit[digit] == 0 {
            return Err(SampleProblemError::invalid(format!(
                "invalid langford solution: digit {} not placed",
                digit
            )));
        }
    }

    if arrangement.contains(&0) {
        return Err(SampleProblemError::invalid(
            "invalid langford solution: at least one slot is unfilled",
        ));
    }

    Ok(Some(arrangement))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sat::solve_via_backtracking;

    #[test]
    fn waerden_rejects_zero_inputs() {
        assert!(waerden(0, 3, 8).is_err());
        assert!(waerden(3, 0, 8).is_err());
        assert!(waerden(3, 3, 0).is_err());
    }

    #[test]
    fn waerden_338_is_satisfiable_and_matches_known_assignment() {
        let problem = waerden(3, 3, 8).expect("valid input");
        assert_eq!(problem.clause_count(), 24);

        let known = vec![false, false, true, true, false, false, true, true];
        assert!(problem.is_satisfied(&known));
        assert!(solve_via_backtracking(&problem).is_some());
    }

    #[test]
    fn waerden_339_is_unsatisfiable() {
        let problem = waerden(3, 3, 9).expect("valid input");
        assert_eq!(problem.clause_count(), 32);
        assert_eq!(solve_via_backtracking(&problem), None);
    }

    #[test]
    fn langford_rejects_zero() {
        assert!(langford(0).is_err());
    }

    #[test]
    fn langford_n3_clause_count_with_symmetry_break() {
        let problem = langford(3).expect("valid input");
        assert_eq!(problem.clause_count(), 32);
    }

    #[test]
    fn langford_n3_sat() {
        let problem = langford(3).expect("valid input");
        assert!(solve_via_backtracking(&problem).is_some());
    }

    #[test]
    fn langford_n4_sat() {
        let problem = langford(4).expect("valid input");
        assert!(solve_via_backtracking(&problem).is_some());
    }

    #[test]
    fn langford_n5_unsat() {
        let problem = langford(5).expect("valid input");
        assert_eq!(solve_via_backtracking(&problem), None);
    }

    #[test]
    fn waerden_solution_string_none() {
        assert_eq!(waerden_solution_string(8, None).unwrap(), None);
    }

    #[test]
    fn waerden_solution_string_known_sequence() {
        let known = vec![false, false, true, true, false, false, true, true];
        assert_eq!(
            waerden_solution_string(8, Some(&known)).unwrap(),
            Some("00110011".to_string())
        );
    }

    #[test]
    fn waerden_solution_string_rejects_short_assignment() {
        let short = vec![true, false];
        assert!(waerden_solution_string(3, Some(&short)).is_err());
    }

    fn is_valid_langford(arrangement: &[u8], n: u8) -> bool {
        if arrangement.len() != 2 * n as usize {
            return false;
        }
        for k in 1..=n {
            let positions: Vec<usize> = arrangement
                .iter()
                .enumerate()
                .filter_map(|(idx, &v)| if v == k { Some(idx) } else { None })
                .collect();
            if positions.len() != 2 {
                return false;
            }
            if positions[1] - positions[0] != k as usize + 1 {
                return false;
            }
        }
        true
    }

    #[test]
    fn langford_solution_arrangement_none() {
        assert_eq!(langford_solution_arrangement(3, None).unwrap(), None);
    }

    #[test]
    fn langford_solution_arrangement_from_solver() {
        let problem = langford(3).expect("valid input");
        let solution = solve_via_backtracking(&problem).expect("sat");
        let arrangement = langford_solution_arrangement(3, Some(&solution))
            .unwrap()
            .expect("present");
        assert!(is_valid_langford(&arrangement, 3));
    }

    #[test]
    fn langford_solution_arrangement_rejects_malformed_solution() {
        let malformed = vec![false; langford_options(3).len()];
        assert!(langford_solution_arrangement(3, Some(&malformed)).is_err());
    }
}
