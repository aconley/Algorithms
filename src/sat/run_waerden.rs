use sat::sat_problem::SatProblem;
use sat::sample_problems::waerden;
use sat::dpll_alternatives::dpll_gemini::solve_via_dpll;

fn main() {
    let p = waerden(3, 3, 10).unwrap();
    let assignment = solve_via_dpll(&p).unwrap();
    println!("Found assignment: {:?}", assignment);
    println!("Is satisfied? {}", p.is_satisfied(&assignment));
}
