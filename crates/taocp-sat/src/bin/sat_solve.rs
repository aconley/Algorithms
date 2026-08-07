//! Command line solver for sample SAT problems.
//!
//! Usage:
//!   sat_solve <solver> <problem> <problem-params>
//!   sat_solve --help
//!   sat_solve -h
//!
//! Solvers:
//!   backtracking
//!   lazy_backtracking
//!   dpll
//!
//! Problems:
//!   langford <n>
//!   waerden <j> <k> <n>
//!
//! Examples:
//!   cargo run --bin sat_solve -- backtracking langford 4
//!   cargo run --bin sat_solve -- lazy_backtracking waerden 3 3 9
//!
use std::env;
use std::fmt;
use std::process;

use taocp_sat::{
    langford, langford_solution_arrangement, solve_via_backtracking, solve_via_lazy_backtracking,
    solve_via_dpll, waerden, waerden_solution_string, SampleProblemError, SatProblem,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Solver {
    Backtracking,
    LazyBacktracking,
    Dpll,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Problem {
    Langford { n: u8 },
    Waerden { j: u8, k: u8, n: u8 },
}

const USAGE: &str = "Usage:\n  sat_solve <solver> <problem> <problem-params>\n\nSolvers:\n  backtracking\n  lazy_backtracking\n  dpll\n\nProblems:\n  langford <n>\n  waerden <j> <k> <n>";

fn is_help_flag(arg: &str) -> bool {
    matches!(arg, "-h" | "--help")
}

fn parse_u8(name: &str, value: &str) -> Result<u8, String> {
    value
        .parse::<u8>()
        .map_err(|_| format!("invalid {name}: '{value}' is not a valid u8"))
}

fn parse_solver(raw: &str) -> Result<Solver, String> {
    match raw {
        "backtracking" => Ok(Solver::Backtracking),
        "lazy_backtracking" => Ok(Solver::LazyBacktracking),
        "dpll" => Ok(Solver::Dpll),
        _ => Err(format!("unknown solver: '{raw}'")),
    }
}

fn parse_problem(args: &[String]) -> Result<Problem, String> {
    match args[2].as_str() {
        "langford" => {
            if args.len() != 4 {
                return Err("langford expects exactly 1 parameter: <n>".to_string());
            }
            let n = parse_u8("langford n", &args[3])?;
            Ok(Problem::Langford { n })
        }
        "waerden" => {
            if args.len() != 6 {
                return Err("waerden expects exactly 3 parameters: <j> <k> <n>".to_string());
            }
            let j = parse_u8("waerden j", &args[3])?;
            let k = parse_u8("waerden k", &args[4])?;
            let n = parse_u8("waerden n", &args[5])?;
            Ok(Problem::Waerden { j, k, n })
        }
        other => Err(format!("unknown problem: '{other}'")),
    }
}

fn parse_cli(args: &[String]) -> Result<(Solver, Problem), String> {
    if args.len() < 3 {
        return Err("expected at least 2 arguments: <solver> <problem>".to_string());
    }

    let solver = parse_solver(&args[1])?;
    let problem = parse_problem(args)?;
    Ok((solver, problem))
}

fn build_problem(problem: Problem) -> Result<SatProblem, SampleProblemError> {
    match problem {
        Problem::Langford { n } => langford(n),
        Problem::Waerden { j, k, n } => waerden(j, k, n),
    }
}

fn solve(problem: &SatProblem, solver: Solver) -> Option<Vec<bool>> {
    match solver {
        Solver::Backtracking => solve_via_backtracking(problem),
        Solver::LazyBacktracking => solve_via_lazy_backtracking(problem),
        Solver::Dpll => solve_via_dpll(problem),
    }
}

fn format_solution(problem: Problem, assignment: &[bool]) -> Result<String, SampleProblemError> {
    match problem {
        Problem::Langford { n } => {
            let arrangement = langford_solution_arrangement(n, Some(assignment))
                .map(|opt| opt.unwrap_or_default())?;
            let text = arrangement
                .iter()
                .map(u8::to_string)
                .collect::<Vec<_>>()
                .join(" ");
            Ok(format!("Langford arrangement: [{text}]"))
        }
        Problem::Waerden { n, .. } => {
            let sequence = waerden_solution_string(n, Some(assignment))?.unwrap_or_default();
            Ok(format!("Waerden bitstring: {sequence}"))
        }
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 && is_help_flag(&args[1]) {
        println!("{USAGE}");
        return Ok(());
    }

    let (solver, problem_kind) = parse_cli(&args)?;

    let problem = build_problem(problem_kind).map_err(|e| e.to_string())?;
    let assignment = solve(&problem, solver);

    match assignment {
        Some(values) => {
            let formatted = format_solution(problem_kind, &values).map_err(|e| e.to_string())?;
            println!("Solution found.");
            println!("{formatted}");
        }
        None => println!("No solution exists."),
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {err}");
        eprintln!("\n{USAGE}");
        process::exit(1);
    }
}

impl fmt::Display for Solver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Solver::Backtracking => write!(f, "backtracking"),
            Solver::LazyBacktracking => write!(f, "lazy_backtracking"),
            Solver::Dpll => write!(f, "dpll"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cli(parts: &[&str]) -> Vec<String> {
        parts.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn parse_cli_langford_ok() {
        let args = cli(&["sat_solve", "backtracking", "langford", "4"]);
        assert_eq!(
            parse_cli(&args),
            Ok((Solver::Backtracking, Problem::Langford { n: 4 }))
        );
    }

    #[test]
    fn parse_cli_waerden_ok() {
        let args = cli(&["sat_solve", "lazy_backtracking", "waerden", "3", "3", "8"]);
        assert_eq!(
            parse_cli(&args),
            Ok((
                Solver::LazyBacktracking,
                Problem::Waerden { j: 3, k: 3, n: 8 }
            ))
        );
    }

    #[test]
    fn parse_cli_rejects_unknown_solver() {
        let args = cli(&["sat_solve", "foo", "langford", "4"]);
        assert!(parse_cli(&args).is_err());
    }

    #[test]
    fn parse_cli_rejects_bad_waerden_arity() {
        let args = cli(&["sat_solve", "backtracking", "waerden", "3", "8"]);
        assert!(parse_cli(&args).is_err());
    }

    #[test]
    fn help_flag_short_is_recognized() {
        assert!(is_help_flag("-h"));
    }

    #[test]
    fn help_flag_long_is_recognized() {
        assert!(is_help_flag("--help"));
    }
}
