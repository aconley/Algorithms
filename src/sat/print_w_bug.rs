use sat::sat_problem::SatProblem;
use sat::sample_problems::waerden;
use sat::dpll_alternatives::dpll_gemini::solve_via_dpll;

fn main() {
    let p = waerden(3, 3, 10).unwrap();
    let assignment = solve_via_dpll(&p).unwrap();
    println!("Found assignment: {:?}", assignment);
    
    // Evaluate which clauses are satisfied/unsatisfied
    for (idx, clause) in p.clauses().iter().enumerate() {
        let mut sat = false;
        for &lit in clause.literals() {
            let var = lit / 2;
            let val = if lit % 2 == 0 { true } else { false };
            if assignment[var as usize - 1] == val {
                sat = true;
                break;
            }
        }
        if !sat {
            println!("Clause {} is unsatisfied: {:?}", idx, clause);
        }
    }
}
