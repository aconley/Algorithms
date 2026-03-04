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
                return SatProblem { clauses: vec![clause.clone()] };
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
                return Ok(SatProblem { clauses: vec![clause] });
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
    use claim::{assert_err, assert_ok};
    use std::collections::HashSet;

    // --- Literal helpers ---

    #[test]
    fn test_literal_display_positive_single_digit() {
        assert_eq!(literal_display_str(2), "1");
        assert_eq!(literal_display_str(4), "2");
        assert_eq!(literal_display_str(10), "5");
    }

    #[test]
    fn test_literal_display_negative_single_digit() {
        // ¬x_1 = lit 3 → "1̅"
        assert_eq!(literal_display_str(3), "1\u{0305}");
        assert_eq!(literal_display_str(5), "2\u{0305}");
    }

    #[test]
    fn test_literal_display_positive_multi_digit() {
        // x_12 = lit 24
        assert_eq!(literal_display_str(24), "12");
    }

    #[test]
    fn test_literal_display_negative_multi_digit() {
        // ¬x_12 = lit 25 → "1̅2̅"
        assert_eq!(literal_display_str(25), "1\u{0305}2\u{0305}");
    }

    // --- Clause::new success ---

    #[test]
    fn test_clause_new_empty() {
        let c = assert_ok!(Clause::new(&[]));
        assert!(c.is_empty());
        assert_eq!(c.len(), 0);
    }

    #[test]
    fn test_clause_new_single_positive() {
        let c = assert_ok!(Clause::new(&[2]));
        assert_eq!(c.literals(), &[2]);
    }

    #[test]
    fn test_clause_new_single_negative() {
        let c = assert_ok!(Clause::new(&[3]));
        assert_eq!(c.literals(), &[3]);
    }

    #[test]
    fn test_clause_new_unsorted_gets_sorted() {
        let c = assert_ok!(Clause::new(&[6, 2, 4]));
        assert_eq!(c.literals(), &[2, 4, 6]);
    }

    // --- Clause::new errors ---

    #[test]
    fn test_clause_new_literal_zero() {
        let err = assert_err!(Clause::new(&[0]));
        assert_eq!(
            err,
            ClauseError::new("invalid literal 0: literals must be >= 2 (variables are 1-indexed)")
        );
    }

    #[test]
    fn test_clause_new_literal_one() {
        let err = assert_err!(Clause::new(&[1]));
        assert_eq!(
            err,
            ClauseError::new("invalid literal 1: literals must be >= 2 (variables are 1-indexed)")
        );
    }

    #[test]
    fn test_clause_new_duplicate_literal() {
        let err = assert_err!(Clause::new(&[2, 2]));
        assert_eq!(err, ClauseError::new("duplicate literal 2 in clause"));
    }

    // --- Clause equality ---

    #[test]
    fn test_clause_equality_same_order() {
        let a = assert_ok!(Clause::new(&[2, 4, 6]));
        let b = assert_ok!(Clause::new(&[2, 4, 6]));
        assert_eq!(a, b);
    }

    #[test]
    fn test_clause_equality_different_order() {
        let a = assert_ok!(Clause::new(&[6, 2, 4]));
        let b = assert_ok!(Clause::new(&[2, 4, 6]));
        assert_eq!(a, b);
    }

    #[test]
    fn test_clause_inequality() {
        let a = assert_ok!(Clause::new(&[2, 4]));
        let b = assert_ok!(Clause::new(&[2, 6]));
        assert_ne!(a, b);
    }

    // --- Clause in HashSet ---

    #[test]
    fn test_clause_hashset_deduplication() {
        let a = assert_ok!(Clause::new(&[6, 2, 4]));
        let b = assert_ok!(Clause::new(&[2, 4, 6]));
        let mut set = HashSet::new();
        set.insert(a);
        set.insert(b);
        assert_eq!(set.len(), 1);
    }

    // --- Clause Display ---

    #[test]
    fn test_clause_display_empty() {
        let c = assert_ok!(Clause::new(&[]));
        assert_eq!(format!("{}", c), "{}");
    }

    #[test]
    fn test_clause_display_single() {
        let c = assert_ok!(Clause::new(&[2]));
        assert_eq!(format!("{}", c), "{1}");
    }

    #[test]
    fn test_clause_display_single_negative() {
        let c = assert_ok!(Clause::new(&[3]));
        assert_eq!(format!("{}", c), "{1\u{0305}}");
    }

    #[test]
    fn test_clause_display_multi() {
        // Spec example: {1̅ 2 3}  → lits: ¬x_1=3, x_2=4, x_3=6
        let c = assert_ok!(Clause::new(&[3, 4, 6]));
        assert_eq!(format!("{}", c), "{1\u{0305} 2 3}");
    }

    // --- Clause Debug ---

    #[test]
    fn test_clause_debug_equals_display() {
        let c = assert_ok!(Clause::new(&[3, 4, 6]));
        assert_eq!(format!("{:?}", c), format!("{}", c));
    }

    // --- SatProblem constructors ---

    #[test]
    fn test_sat_problem_empty_from_clauses() {
        let p = SatProblem::from_clauses(&[]);
        assert_eq!(p.clause_count(), 0);
    }

    #[test]
    fn test_sat_problem_new_empty_clause_collapses() {
        let empty = assert_ok!(Clause::new(&[]));
        let other = assert_ok!(Clause::new(&[2, 3]));
        let p = SatProblem::new(vec![other, empty]);
        assert_eq!(p.clause_count(), 1);
        assert!(p.clauses()[0].is_empty());
    }

    #[test]
    fn test_sat_problem_from_clauses_empty_clause_collapses() {
        let empty = assert_ok!(Clause::new(&[]));
        let other = assert_ok!(Clause::new(&[2, 3]));
        let p = SatProblem::from_clauses(&[other, empty]);
        assert_eq!(p.clause_count(), 1);
        assert!(p.clauses()[0].is_empty());
    }

    #[test]
    fn test_sat_problem_from_literals_empty_clause_collapses() {
        let p = assert_ok!(SatProblem::from_literals(&[vec![2, 3], vec![]]));
        assert_eq!(p.clause_count(), 1);
        assert!(p.clauses()[0].is_empty());
    }

    #[test]
    fn test_sat_problem_from_clauses() {
        let c1 = assert_ok!(Clause::new(&[2, 3]));
        let c2 = assert_ok!(Clause::new(&[4, 5]));
        let p = SatProblem::from_clauses(&[c1, c2]);
        assert_eq!(p.clause_count(), 2);
    }

    #[test]
    fn test_sat_problem_from_literals_success() {
        let p = assert_ok!(SatProblem::from_literals(&[vec![2, 3], vec![4, 5]]));
        assert_eq!(p.clause_count(), 2);
    }

    #[test]
    fn test_sat_problem_from_literals_error_propagation() {
        let err = assert_err!(SatProblem::from_literals(&[vec![2, 3], vec![0]]));
        assert_eq!(
            err,
            SatProblemError::from(
                "invalid literal 0: literals must be >= 2 (variables are 1-indexed)"
            )
        );
    }

    // --- SatProblem Display ---

    #[test]
    fn test_sat_problem_display_empty() {
        let p = SatProblem::from_clauses(&[]);
        assert_eq!(format!("{}", p), "");
    }

    #[test]
    fn test_sat_problem_display_one_clause() {
        let p = assert_ok!(SatProblem::from_literals(&[vec![2, 3]]));
        assert_eq!(format!("{}", p), "{1 1\u{0305}}");
    }

    #[test]
    fn test_sat_problem_display_two_clauses() {
        let p = assert_ok!(SatProblem::from_literals(&[vec![2], vec![4]]));
        assert_eq!(format!("{}", p), "{1} {2}");
    }

    // --- display_latex ---

    #[test]
    fn test_display_latex_empty() {
        let p = SatProblem::from_clauses(&[]);
        assert_eq!(p.display_latex(), r"$\emptyset$");
    }

    #[test]
    fn test_display_latex_unsat() {
        let p = assert_ok!(SatProblem::from_literals(&[vec![]]));
        assert_eq!(p.display_latex(), r"$\bot$");
    }

    #[test]
    fn test_display_latex_single_positive_literal() {
        let p = assert_ok!(SatProblem::from_literals(&[vec![2]]));
        assert_eq!(p.display_latex(), r"$\left(x_{1}\right)$");
    }

    #[test]
    fn test_display_latex_single_negative_literal() {
        let p = assert_ok!(SatProblem::from_literals(&[vec![3]]));
        assert_eq!(p.display_latex(), r"$\left({\bar x}_{1}\right)$");
    }

    #[test]
    fn test_display_latex_spec_example() {
        // (x_1 ∨ ¬x_2 ∨ x_4): lits 2, 5, 8
        let p = assert_ok!(SatProblem::from_literals(&[vec![2, 5, 8]]));
        assert_eq!(
            p.display_latex(),
            r"$\left(x_{1} \vee {\bar x}_{2} \vee x_{4}\right)$"
        );
    }

    #[test]
    fn test_display_latex_multi_digit_variable() {
        // x_12 = lit 24
        let p = assert_ok!(SatProblem::from_literals(&[vec![24]]));
        assert_eq!(p.display_latex(), r"$\left(x_{12}\right)$");
    }

    #[test]
    fn test_display_latex_two_clauses() {
        let p = assert_ok!(SatProblem::from_literals(&[vec![2], vec![4]]));
        assert_eq!(
            p.display_latex(),
            r"$\left(x_{1}\right) \wedge \left(x_{2}\right)$"
        );
    }
}
