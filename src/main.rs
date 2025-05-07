mod cnf;
mod parser;
mod solver;
mod trail;

use std::collections::HashSet;
use std::env;
use std::fs;
use crate::cnf::Lit;

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
    let mut cnf = parser::parse(contents)
        .expect("Failed to parse cnf");

    // Solve
    let solution = get_solution(solver::solve(&mut cnf), cnf.num_vars());
    println!("{solution}");
}

/**
 * Given a result, get a string representation of the solution. The output is
 * in the format expected by the SAT Competition:
 * https://satcompetition.github.io/2024/output.html
 */
fn get_solution(result: Result<HashSet<Lit>, ()>, num_vars: usize) -> String {
    match result {
        Ok(assign) =>
            format!("s SATISFIABLE\nv {}0",
                    get_assignments_str(&assign, num_vars as i32)),
        Err(()) => String::from("s UNSATISFIABLE"),
    }
}

fn get_assignments_str(assignments: &HashSet<Lit>, num_vars: i32) -> String {
    (1..num_vars + 1).fold(String::new(), |acc, var| {
        let lit = if assignments.contains(&var) {
            var.to_string()
        }
        else {
            (-var).to_string()
        };

        format!("{}{} ", acc, lit)
    })
}