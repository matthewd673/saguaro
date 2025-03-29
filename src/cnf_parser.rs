use crate::cnf;

pub struct ProblemDef {
    pub num_vars: usize,
    pub num_clauses: usize,
}

pub fn parse(str: String) -> (ProblemDef, cnf::Cnf) {
    let mut lines = str.lines()
        .filter(|l| !l.is_empty() && !l.starts_with('c'));

    let problem_def = parse_prob_def(lines.next().unwrap()).unwrap();
    let cnf = lines
        .map(|l| parse_clause(l).unwrap())
        .collect();

    (problem_def, cnf)
}

fn parse_prob_def(line: &str) -> Option<ProblemDef> {
    let words: Vec<&str> = line.split(' ').collect();
    if words.len() != 4
        || !matches!(words[0], "p")
        || !matches!(words[1], "cnf") {
        return None;
    }

    Some(ProblemDef {
        num_vars: words[2].parse().unwrap(),
        num_clauses: words[3].parse().unwrap(),
    })
}

fn parse_clause(line: &str) -> Option<cnf::Clause> {
    let words: Vec<&str> = line.split(' ').collect();
    if words.last() != Some(&"0") {
        return None;
    }

    words.iter()
        .filter(|w| !w.eq(&&"0"))
        .map(|w| Some(w.parse().unwrap()))
        .collect()
}