use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use crate::assignments::Assignments;
use crate::cnf::{Clause, Lit};

#[derive(Debug)]
pub enum TrailNodeDecorator {
    Decision,
    Clause(Clause),
}

impl Display for TrailNodeDecorator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TrailNodeDecorator::Decision => f.write_str("dec")
                .expect("An error occurred while formatting"),
            TrailNodeDecorator::Clause(clause) =>
                f.write_fmt(format_args!("{}", clause.iter()
                    .map(|l| l.to_string())
                    .collect::<Vec<String>>()
                    .join(" v ")))
                    .expect("An error occurred while formatting"),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct TrailNode {
    pub lit: Lit,
    pub decorator: TrailNodeDecorator,
}

#[derive(Debug)]
pub struct Trail {
    list: Vec<TrailNode>,
    dec_level: usize,
}

impl Trail {
    pub fn new() -> Self {
        Trail {
            list: Vec::new(),
            dec_level: 0,
        }
    }

    pub fn push(&mut self, lit: Lit, decorator: TrailNodeDecorator) {
        if matches!(decorator, TrailNodeDecorator::Decision) {
            self.dec_level += 1;
        }

        self.list.push(TrailNode { lit, decorator });
    }

    pub fn pop(&mut self) -> Option<TrailNode> {
        let top = self.list.pop();

        match top {
            Some(TrailNode { decorator: TrailNodeDecorator::Decision, .. }) => {
                self.dec_level -= 1;
            },
            _ => {},
        }

        top
    }

    pub fn dec_level(&self) -> usize {
        self.dec_level
    }

    pub fn get_latest_decision_children(&self) -> HashSet<Lit> {
        let mut children = HashSet::new();

        let mut ind = self.list.len() - 1;
        loop {
            let n = self.list.get(ind).unwrap();
            children.insert(n.lit);

            match n.decorator {
                TrailNodeDecorator::Decision => {
                    break;
                },
                _ => {},
            }
            ind -= 1;
        }

        children
    }

    pub fn get_parents(&self, lit: &Lit) -> HashSet<Lit> {
        let lit_node = self.list.iter()
            .find(|n| lit.eq(&n.lit))
            .unwrap();

        let lit_clause;
        match &lit_node.decorator {
            TrailNodeDecorator::Clause(c) => { lit_clause = c; },
            TrailNodeDecorator::Decision => { return HashSet::new(); },
        }

        self.list.iter()
            .filter(|&n | {
                !lit.eq(&n.lit) && !lit.eq(&-n.lit) &&
                    lit_clause.contains(&-n.lit)
            })
            .map(|n| n.lit)
            .collect()
    }

    pub fn get_latest_in_set(&self, set: &HashSet<Lit>) -> Lit {
        let mut ind = self.list.len() - 1;
        loop {
            let n = self.list.get(ind).unwrap().lit;
            if set.contains(&n) {
                break n
            }
            ind -= 1;
        }
    }

    pub fn remove(&mut self, lit: &Lit) {
        let ind = self.list.iter()
            .position(|n| lit.eq(&n.lit))
            .unwrap();
        self.list.remove(ind);
    }

    // For debugging
    pub fn to_graphviz(&self) -> String {
        let edges = self.list.iter()
            .fold(String::new(), |acc, node| {
                let edges = self.get_parents(&node.lit).iter()
                    .fold(String::new(), |acc, p| {
                        format!("{acc}{p}->{} [label=\"{}\"] ",
                                node.lit, node.decorator)
                    });
                format!("{acc}{edges} ")
            });

        let dec_nodes = self.list.iter()
            .filter(|n|
                matches!(n.decorator, TrailNodeDecorator::Decision))
            .fold(String::new(), |acc, node| {
                format!("{acc}{} [color=\"gold\"]", node.lit)
            });

        format!("digraph G {{ {dec_nodes} {edges} }}")
    }
}

impl Assignments for Trail {
    fn is_sat(&self, lit: &Lit) -> bool {
        self.list.iter()
            .any(|n| lit.eq(&n.lit))
    }

    fn get_assignments(&self) -> HashSet<Lit> {
        self.list.iter()
            .map(|n| n.lit)
            .collect()
    }
}