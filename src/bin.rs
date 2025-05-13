mod cnf;
mod parser;
mod solver;
mod trail;

use std::env;
use std::fs;

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

    println!("{}", saguaro::solve_cnf(contents));
}