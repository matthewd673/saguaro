mod cnf;
mod cnf_parser;

use std::env;
use std::fs;

fn main() {
    println!("saguaro");

    // Get filename
    let filename = env::args().skip(1).next();
    if matches!(filename, None) {
        println!("usage: saguaro <cnf file>");
        return;
    }

    let contents = fs::read_to_string(filename.unwrap())
        .expect("Failed to read cnf file");

    let (prob_def, cnf) = cnf_parser::parse(contents);
    println!("Solving problem with: vars={}, clauses={}", prob_def.num_vars, prob_def.num_clauses);

    println!("{:?}", cnf);
}
