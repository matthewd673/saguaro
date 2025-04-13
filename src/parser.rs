use std::iter::Peekable;
use std::str::Chars;
use crate::cnf::{Cnf, Clause};

pub fn parse(str: String) -> Option<Cnf> {
    let mut chars = str.chars().peekable();
    skip_irrelevant(&mut chars);

    let (num_vars, num_clauses);
    match parse_problem_def(&mut chars) {
        Some(p) => { (num_vars, num_clauses) = p; },
        None => { return None; },
    }

    let clauses;
    match parse_clauses(&mut chars, num_clauses) {
        Some(c) => { clauses = c; },
        None => { return None; },
    }

    Some(Cnf::new(clauses, num_vars))
}

fn eat(chars: &mut Peekable<Chars>, seq: &str) -> Result<(), ()> {
    for c in seq.chars() {
        match chars.next() {
            Some(d) if !c.eq(&d) => { return Err(()) },
            None => { return Err(()) },
            _ => {},
        }
    }

    Ok(())
}

fn scan_int(chars: &mut Peekable<Chars>) -> Result<i32, ()> {
    match chars.peek() {
        Some(c) if c.eq(&'-') || c.is_ascii_digit() => {},
        _ => { return Err(()); },
    }

    let mut image = String::new();

    loop {
        match chars.next() {
            Some(c) if c.eq(&'-') || c.is_ascii_digit() => {
                image.push(c.clone());
            },
            _ => { break; },
        }
    }

    match image.parse() {
        Ok(i) => Ok(i),
        _ => Err(()),
    }
}

fn parse_problem_def(chars: &mut Peekable<Chars>) -> Option<(usize, usize)> {
    let res = eat(chars, "p cnf ");
    if matches!(res, Err(_)) {
        return None;
    }

    let vars_img = scan_int(chars);
    skip_whitespace(chars);
    let clauses_img = scan_int(chars);

    match (vars_img, clauses_img) {
        (Ok(num_vars), Ok(num_clauses))
            if num_vars > 0 && num_clauses > 0 =>
            Some((
                num_vars as usize,
                num_clauses as usize,
            )),
        _ => None,
    }
}

fn parse_clauses(chars: &mut Peekable<Chars>, num_clauses: usize) -> Option<Vec<Clause>> {
    let mut clauses = Vec::with_capacity(num_clauses);

    skip_irrelevant(chars);
    while !matches!(chars.peek(), None) {
        match parse_clause(chars) {
            Some(clause) => { clauses.push(clause); },
            None => { return None; },
        }
        skip_irrelevant(chars);
    }

    Some(clauses)
}

fn parse_clause(chars: &mut Peekable<Chars>) -> Option<Clause> {
    let mut clause = Vec::new();

    skip_irrelevant(chars);
    loop {
        skip_whitespace(chars);
        let next = scan_int(chars);
        match next {
            Ok(0) => { break; },
            Ok(int) => { clause.push(int); },
            Err(_) => { return None; },
        }
    }

    Some(clause)
}

fn skip_irrelevant(chars: &mut Peekable<Chars>) {
    while see_comment(chars) || see_whitespace(chars) {
        skip_comment(chars);
        skip_whitespace(chars);
    }
}

fn see_comment(chars: &mut Peekable<Chars>) -> bool {
    matches!(chars.peek(), Some('c'))
}

fn skip_comment(chars: &mut Peekable<Chars>) {
    if see_comment(chars) {
        skip_until(chars, |c| matches!(c, Some('\n')));
    }
}

fn see_whitespace(chars: &mut Peekable<Chars>) -> bool {
    matches!(chars.peek(), Some(c) if c.is_ascii_whitespace())
}

fn skip_whitespace(chars: &mut Peekable<Chars>) {
    while see_whitespace(chars) {
        chars.next();
    }
}

fn skip_until(chars: &mut Peekable<Chars>,
              predicate: impl Fn(Option<&char>) -> bool) {
    while !predicate(chars.peek()) {
        chars.next();
    }
}
