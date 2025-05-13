mod parser;
mod cnf;
mod solver;
mod trail;

use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use crate::cnf::Lit;

#[wasm_bindgen]
pub fn solve_cnf(str: String) -> String {
    let mut cnf;
    match parser::parse(str) {
        Some(f) => { cnf = f; },
        _ => { // TODO: Proper error handling
            return String::from("Syntax error in CNF");
        },
    }

    get_solution(solver::solve(&mut cnf), cnf.num_vars())
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
