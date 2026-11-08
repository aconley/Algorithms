// SAT problem data structures for input and output.
//
// Literal encoding:
//   x_i  → 2 * i       (e.g. x_1 → 2)
//   ¬x_i → 2 * i + 1   (e.g. ¬x_1 → 3)
// Variables are 1-indexed; literals 0 and 1 are invalid.

use std::fmt;

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq)]
pub struct ClauseError(pub String);

impl ClauseError {
    fn new<S: Into<String>>(message: S) -> Self {
        ClauseError(message.into())
    }
}

impl From<&str> for ClauseError {
    fn from(value: &str) -> Self {
        ClauseError(value.into())
    }
}

impl From<String> for ClauseError {
    fn from(value: String) -> Self {
        ClauseError(value)
    }
}

impl fmt::Display for ClauseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SatProblemError(pub String);

impl From<&str> for SatProblemError {
    fn from(value: &str) -> Self {
        SatProblemError(value.into())
    }
}

impl From<String> for SatProblemError {
    fn from(value: String) -> Self {
        SatProblemError(value)
    }
}

impl From<ClauseError> for SatProblemError {
    fn from(e: ClauseError) -> Self {
        SatProblemError(e.0)
    }
}

impl fmt::Display for SatProblemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ---------------------------------------------------------------------------
// Literal display helpers
// ---------------------------------------------------------------------------

/// Returns the display string for a literal.
/// Positive literal (even): variable number as digits.
/// Negative literal (odd): variable number digits each followed by COMBINING OVERLINE (U+0305).
fn literal_display_str(lit: u32) -> String {
    let var = lit / 2;
    let var_str = var.to_string();
    if lit.is_multiple_of(2) {
        var_str
    } else {
        // Append U+0305 COMBINING OVERLINE after each digit
        var_str.chars().flat_map(|c| [c, '\u{0305}']).collect()
    }
}

/// Returns the LaTeX string for a literal.
/// Positive: `x_{var}`, Negative: `{\bar x}_{var}`.
fn literal_latex_str(lit: u32) -> String {
    let var = lit / 2;
    if lit.is_multiple_of(2) {
        format!("x_{{{}}}", var)
    } else {
        format!("{{\\bar x}}_{{{}}}", var)
    }
}

// ---------------------------------------------------------------------------
// Clause
// ---------------------------------------------------------------------------

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Clause {
    literals: Box<[u32]>,
}

impl Clause {
    /// Constructs a clause from a slice of literals.
    ///
    /// Rejects literals < 2 (invalid), deduplicates detection (error on duplicate),
    /// and stores literals in sorted order.
    pub fn new(literals: &[u32]) -> Result<Self, ClauseError> {
        for &lit in literals {
            if lit < 2 {
                return Err(ClauseError::new(format!(
                    "invalid literal {}: literals must be >= 2 (variables are 1-indexed)",
                    lit
                )));
            }
        }

        let mut sorted = literals.to_vec();
        sorted.sort_unstable();

        for window in sorted.windows(2) {
            if window[0] == window[1] {
                return Err(ClauseError::new(format!(
                    "duplicate literal {} in clause",
                    window[0]
                )));
            }
        }

        Ok(Clause {
            literals: sorted.into_boxed_slice(),
        })
    }

    pub fn len(&self) -> usize {
        self.literals.len()
    }

    pub fn is_empty(&self) -> bool {
        self.literals.is_empty()
    }

    pub fn literals(&self) -> &[u32] {
        &self.literals
    }

    /// Returns true if this clause is satisfied by `assignment`.
    ///
    /// `assignment[i]` corresponds to variable `x_{i+1}`.
    /// If `assignment` is too short for a literal's variable index, that
    /// literal is treated as not satisfied.
    pub fn is_satisfied(&self, assignment: &[bool]) -> bool {
        self.literals.iter().any(|&lit| {
            let var = (lit / 2) as usize;
            if var == 0 || var > assignment.len() {
                return false;
            }

            let value = assignment[var - 1];
            if lit.is_multiple_of(2) { value } else { !value }
        })
    }
}

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{")?;
        for (i, &lit) in self.literals.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", literal_display_str(lit))?;
        }
        write!(f, "}}")
    }
}

impl fmt::Debug for Clause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

// ---------------------------------------------------------------------------
// SatProblem
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq)]
pub struct SatProblem {
    clauses: Vec<Clause>,
}

impl SatProblem {
    /// Constructs a SAT problem taking ownership of the provided clauses.
    ///
    /// If any clause is empty the problem is immediately unsatisfiable, so only
    /// a single empty clause is stored and the rest are discarded.
    pub fn new(clauses: Vec<Clause>) -> Self {
        if clauses.iter().any(|c| c.is_empty()) {
            SatProblem {
                clauses: vec![Clause::new(&[]).expect("empty clause is always valid")],
            }
        } else {
            SatProblem { clauses }
        }
    }

    /// Constructs a SAT problem from a slice of already-constructed clauses.
    ///
    /// If any clause is empty the problem is immediately unsatisfiable, so only
    /// a single empty clause is stored and the rest are discarded.
    pub fn from_clauses(clauses: &[Clause]) -> Self {
        let mut result = Vec::with_capacity(clauses.len());
        for clause in clauses {
            if clause.is_empty() {
                return SatProblem {
                    clauses: vec![clause.clone()],
                };
            }
            result.push(clause.clone());
        }
        SatProblem { clauses: result }
    }

    /// Constructs a SAT problem from a slice of literal vecs, building each clause.
    ///
    /// If any clause is empty the problem is immediately unsatisfiable, so only
    /// a single empty clause is stored and the rest are discarded.
    pub fn from_literals(clauses: &[Vec<u32>]) -> Result<Self, SatProblemError> {
        let mut result = Vec::with_capacity(clauses.len());
        for lits in clauses {
            let clause = Clause::new(lits)?;
            if clause.is_empty() {
                return Ok(SatProblem {
                    clauses: vec![clause],
                });
            }
            result.push(clause);
        }
        Ok(SatProblem { clauses: result })
    }

    pub fn clause_count(&self) -> usize {
        self.clauses.len()
    }

    pub fn clauses(&self) -> &[Clause] {
        &self.clauses
    }

    /// Returns true if `assignment` satisfies every clause in this problem.
    ///
    /// `assignment[i]` corresponds to variable `x_{i+1}`.
    /// If `assignment` is too short for a literal's variable index, that
    /// literal is treated as not satisfied.
    pub fn is_satisfied(&self, assignment: &[bool]) -> bool {
        self.clauses
            .iter()
            .all(|clause| clause.is_satisfied(assignment))
    }

    /// Returns a LaTeX string for the problem.
    pub fn display_latex(&self) -> String {
        if self.clauses.is_empty() {
            return r"$\emptyset$".to_string();
        }
        if self.clauses.len() == 1 && self.clauses[0].is_empty() {
            return r"$\bot$".to_string();
        }
        let clause_strs: Vec<String> = self
            .clauses
            .iter()
            .map(|clause| {
                let lits: Vec<String> = clause
                    .literals()
                    .iter()
                    .map(|&lit| literal_latex_str(lit))
                    .collect();
                format!("\\left({}\\right)", lits.join(" \\vee "))
            })
            .collect();
        format!("${}$", clause_strs.join(" \\wedge "))
    }
}

impl fmt::Display for SatProblem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, clause) in self.clauses.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", clause)?;
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use test_that::prelude::*;

    // --- Literal helpers ---

    #[test]
    fn test_literal_display_positive_single_digit() {
        assert_that!(literal_display_str(2), eq("1"));
        assert_that!(literal_display_str(4), eq("2"));
        assert_that!(literal_display_str(10), eq("5"));
    }

    #[test]
    fn test_literal_display_negative_single_digit() {
        // ¬x_1 = lit 3 → "1̅"
        assert_that!(literal_display_str(3), eq("1\u{0305}"));
        assert_that!(literal_display_str(5), eq("2\u{0305}"));
    }

    #[test]
    fn test_literal_display_positive_multi_digit() {
        // x_12 = lit 24
        assert_that!(literal_display_str(24), eq("12"));
    }

    #[test]
    fn test_literal_display_negative_multi_digit() {
        // ¬x_12 = lit 25 → "1̅2̅"
        assert_that!(literal_display_str(25), eq("1\u{0305}2\u{0305}"));
    }

    // --- Clause::new success ---

    #[test]
    fn test_clause_new_empty() {
        let c = Clause::new(&[]).unwrap();
        assert_that!(c.is_empty(), eq(true));
        assert_that!(c.len(), eq(0));
    }

    #[test]
    fn test_clause_new_single_positive() {
        let c = Clause::new(&[2]).unwrap();
        assert_that!(c.literals(), eq(&[2]));
    }

    #[test]
    fn test_clause_new_single_negative() {
        let c = Clause::new(&[3]).unwrap();
        assert_that!(c.literals(), eq(&[3]));
    }

    #[test]
    fn test_clause_new_unsorted_gets_sorted() {
        let c = Clause::new(&[6, 2, 4]).unwrap();
        assert_that!(c.literals(), eq(&[2, 4, 6]));
    }

    // --- Clause::new errors ---

    #[test]
    fn test_clause_new_literal_zero() {
        let err = Clause::new(&[0]).unwrap_err();
        assert_that!(
            err,
            eq(ClauseError::new(
                "invalid literal 0: literals must be >= 2 (variables are 1-indexed)"
            ))
        );
    }

    #[test]
    fn test_clause_new_literal_one() {
        let err = Clause::new(&[1]).unwrap_err();
        assert_that!(
            err,
            eq(ClauseError::new(
                "invalid literal 1: literals must be >= 2 (variables are 1-indexed)"
            ))
        );
    }

    #[test]
    fn test_clause_new_duplicate_literal() {
        let err = Clause::new(&[2, 2]).unwrap_err();
        assert_that!(err, eq(ClauseError::new("duplicate literal 2 in clause")));
    }

    // --- Clause equality ---

    #[test]
    fn test_clause_equality_same_order() {
        let a = Clause::new(&[2, 4, 6]).unwrap();
        let b = Clause::new(&[2, 4, 6]).unwrap();
        assert_that!(a, eq(b));
    }

    #[test]
    fn test_clause_equality_different_order() {
        let a = Clause::new(&[6, 2, 4]).unwrap();
        let b = Clause::new(&[2, 4, 6]).unwrap();
        assert_that!(a, eq(b));
    }

    #[test]
    fn test_clause_inequality() {
        let a = Clause::new(&[2, 4]).unwrap();
        let b = Clause::new(&[2, 6]).unwrap();
        assert_that!(a, not(eq(b)));
    }

    // --- Clause in HashSet ---

    #[test]
    fn test_clause_hashset_deduplication() {
        let a = Clause::new(&[6, 2, 4]).unwrap();
        let b = Clause::new(&[2, 4, 6]).unwrap();
        let mut set = HashSet::new();
        set.insert(a);
        set.insert(b);
        assert_that!(set, len(eq(1)));
    }

    // --- Clause Display ---

    #[test]
    fn test_clause_display_empty() {
        let c = Clause::new(&[]).unwrap();
        assert_that!(format!("{}", c), eq("{}"));
    }

    #[test]
    fn test_clause_display_single() {
        let c = Clause::new(&[2]).unwrap();
        assert_that!(format!("{}", c), eq("{1}"));
    }

    #[test]
    fn test_clause_display_single_negative() {
        let c = Clause::new(&[3]).unwrap();
        assert_that!(format!("{}", c), eq("{1\u{0305}}"));
    }

    #[test]
    fn test_clause_display_multi() {
        // Spec example: {1̅ 2 3}  → lits: ¬x_1=3, x_2=4, x_3=6
        let c = Clause::new(&[3, 4, 6]).unwrap();
        assert_that!(format!("{}", c), eq("{1\u{0305} 2 3}"));
    }

    // --- Clause Debug ---

    #[test]
    fn test_clause_debug_equals_display() {
        let c = Clause::new(&[3, 4, 6]).unwrap();
        assert_that!(format!("{:?}", c), eq(format!("{}", c)));
    }

    // --- Clause::is_satisfied ---

    #[test]
    fn test_clause_is_satisfied_true() {
        let c = Clause::new(&[2, 5]).unwrap(); // x1 ∨ ¬x2
        assert_that!(c.is_satisfied(&[true, true]), eq(true));
    }

    #[test]
    fn test_clause_is_satisfied_false() {
        let c = Clause::new(&[2, 5]).unwrap(); // x1 ∨ ¬x2
        assert_that!(c.is_satisfied(&[false, true]), eq(false));
    }

    #[test]
    fn test_clause_is_satisfied_short_assignment() {
        let c = Clause::new(&[6]).unwrap(); // x3
        assert_that!(c.is_satisfied(&[true, true]), eq(false));
    }

    // --- SatProblem constructors ---

    #[test]
    fn test_sat_problem_empty_from_clauses() {
        let p = SatProblem::from_clauses(&[]);
        assert_that!(p.clause_count(), eq(0));
    }

    #[test]
    fn test_sat_problem_new_empty_clause_collapses() {
        let empty = Clause::new(&[]).unwrap();
        let other = Clause::new(&[2, 3]).unwrap();
        let p = SatProblem::new(vec![other, empty]);
        assert_that!(p.clause_count(), eq(1));
        assert_that!(p.clauses()[0].is_empty(), eq(true));
    }

    #[test]
    fn test_sat_problem_from_clauses_empty_clause_collapses() {
        let empty = Clause::new(&[]).unwrap();
        let other = Clause::new(&[2, 3]).unwrap();
        let p = SatProblem::from_clauses(&[other, empty]);
        assert_that!(p.clause_count(), eq(1));
        assert_that!(p.clauses()[0].is_empty(), eq(true));
    }

    #[test]
    fn test_sat_problem_from_literals_empty_clause_collapses() {
        let p = SatProblem::from_literals(&[vec![2, 3], vec![]]).unwrap();
        assert_that!(p.clause_count(), eq(1));
        assert_that!(p.clauses()[0].is_empty(), eq(true));
    }

    #[test]
    fn test_sat_problem_from_clauses() {
        let c1 = Clause::new(&[2, 3]).unwrap();
        let c2 = Clause::new(&[4, 5]).unwrap();
        let p = SatProblem::from_clauses(&[c1, c2]);
        assert_that!(p.clause_count(), eq(2));
    }

    #[test]
    fn test_sat_problem_from_literals_success() {
        let p = SatProblem::from_literals(&[vec![2, 3], vec![4, 5]]).unwrap();
        assert_that!(p.clause_count(), eq(2));
    }

    #[test]
    fn test_sat_problem_from_literals_error_propagation() {
        let err = SatProblem::from_literals(&[vec![2, 3], vec![0]]).unwrap_err();
        assert_that!(
            err,
            eq(SatProblemError::from(
                "invalid literal 0: literals must be >= 2 (variables are 1-indexed)"
            ))
        );
    }

    // --- SatProblem Display ---

    #[test]
    fn test_sat_problem_display_empty() {
        let p = SatProblem::from_clauses(&[]);
        assert_that!(format!("{}", p), eq(""));
    }

    #[test]
    fn test_sat_problem_display_one_clause() {
        let p = SatProblem::from_literals(&[vec![2, 3]]).unwrap();
        assert_that!(format!("{}", p), eq("{1 1\u{0305}}"));
    }

    #[test]
    fn test_sat_problem_display_two_clauses() {
        let p = SatProblem::from_literals(&[vec![2], vec![4]]).unwrap();
        assert_that!(format!("{}", p), eq("{1} {2}"));
    }

    // --- is_satisfied ---

    #[test]
    fn test_is_satisfied_true() {
        let p = SatProblem::from_literals(&[vec![2, 3], vec![4]]).unwrap();
        assert_that!(p.is_satisfied(&[true, true]), eq(true));
    }

    #[test]
    fn test_is_satisfied_false() {
        let p = SatProblem::from_literals(&[vec![2], vec![5]]).unwrap();
        assert_that!(p.is_satisfied(&[false, true]), eq(false));
    }

    #[test]
    fn test_is_satisfied_empty_problem() {
        let p = SatProblem::from_clauses(&[]);
        assert_that!(p.is_satisfied(&[]), eq(true));
    }

    #[test]
    fn test_is_satisfied_empty_clause_problem() {
        let p = SatProblem::from_literals(&[vec![]]).unwrap();
        assert_that!(p.is_satisfied(&[]), eq(false));
    }

    #[test]
    fn test_is_satisfied_short_assignment() {
        let p = SatProblem::from_literals(&[vec![6]]).unwrap(); // x_3
        assert_that!(p.is_satisfied(&[true, true]), eq(false));
    }

    // --- display_latex ---

    #[test]
    fn test_display_latex_empty() {
        let p = SatProblem::from_clauses(&[]);
        assert_that!(p.display_latex(), eq(r"$\emptyset$"));
    }

    #[test]
    fn test_display_latex_unsat() {
        let p = SatProblem::from_literals(&[vec![]]).unwrap();
        assert_that!(p.display_latex(), eq(r"$\bot$"));
    }

    #[test]
    fn test_display_latex_single_positive_literal() {
        let p = SatProblem::from_literals(&[vec![2]]).unwrap();
        assert_that!(p.display_latex(), eq(r"$\left(x_{1}\right)$"));
    }

    #[test]
    fn test_display_latex_single_negative_literal() {
        let p = SatProblem::from_literals(&[vec![3]]).unwrap();
        assert_that!(p.display_latex(), eq(r"$\left({\bar x}_{1}\right)$"));
    }

    #[test]
    fn test_display_latex_spec_example() {
        // (x_1 ∨ ¬x_2 ∨ x_4): lits 2, 5, 8
        let p = SatProblem::from_literals(&[vec![2, 5, 8]]).unwrap();
        assert_that!(
            p.display_latex(),
            eq(r"$\left(x_{1} \vee {\bar x}_{2} \vee x_{4}\right)$")
        );
    }

    #[test]
    fn test_display_latex_multi_digit_variable() {
        // x_12 = lit 24
        let p = SatProblem::from_literals(&[vec![24]]).unwrap();
        assert_that!(p.display_latex(), eq(r"$\left(x_{12}\right)$"));
    }

    #[test]
    fn test_display_latex_two_clauses() {
        let p = SatProblem::from_literals(&[vec![2], vec![4]]).unwrap();
        assert_that!(
            p.display_latex(),
            eq(r"$\left(x_{1}\right) \wedge \left(x_{2}\right)$")
        );
    }
}
