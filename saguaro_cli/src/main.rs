use std::collections::HashSet;
use std::env;
use std::fs;
use saguaro::{parser, solver};
use saguaro::cnf::Lit;

fn main() {
    // Get filename
    let filename;
    match env::args().skip(1).next() {
        Some(str) => { filename = str; },
        None => {
            println!("usage: saguaro <cnf file>");
            return;
        }
    }

    // Load and parse CNF file
    let contents;
    match fs::read_to_string(&filename) {
        Ok(str) => { contents = str; },
        Err(_) => {
            println!("Failed to load file \"{}\"", &filename);
            std::process::exit(1);
        },
    }

    let mut cnf;
    match parser::parse(contents) {
        Some(f) => { cnf = f; },
        None => {
            println!("Syntax error in CNF");
            std::process::exit(1);
        }
    }

    let solution = solver::solve(&mut cnf);
    println!("{}", fmt_solution(solution, cnf.num_vars()));
}

fn fmt_solution(result: Result<HashSet<Lit>, ()>, num_vars: usize) -> String {
    match result {
        Ok(assign) =>
            format!("s SATISFIABLE\nv {}0",
                    fmt_assignments(&assign, num_vars as i32)),
        Err(()) => String::from("s UNSATISFIABLE"),
    }
}

fn fmt_assignments(assignments: &HashSet<Lit>, num_vars: i32) -> String {
    (1..num_vars + 1).fold(String::new(), |acc, var| {
        let mul = if assignments.contains(&var) { 1 } else { -1 };
        let lit = (var * mul).to_string();

        format!("{}{} ", acc, lit)
    })
}