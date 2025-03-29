mod cnf;
mod cnf_parser;
mod solver;
mod assignments;

use std::env;
use std::fs;
use crate::assignments::Assignments;

fn main() {
    // Get filename
    let filename = env::args().skip(1).next();
    if matches!(filename, None) {
        println!("usage: saguaro <cnf file>");
        return;
    }

    // Load and parse CNF file
    let contents = fs::read_to_string(filename.unwrap())
        .expect("Failed to read cnf file");
    let (prob_def, cnf) = cnf_parser::parse(contents);

    // Solve
    let solution = get_solution(solver::dpll(&cnf, prob_def.num_vars));
    println!("{solution}");
}

/**
 * Given a result, get a string representation of the solution. The output format is according to
 * the SAT Competition requirements: https://satcompetition.github.io/2024/output.html
 */
pub fn get_solution(result: Result<Assignments, ()>) -> String {
    match result {
        Ok(assign) => format!("s SATISFIABLE\nv {} 0", assign.to_string()),
        Err(()) => String::from("s UNSATISFIABLE"),
    }
}
