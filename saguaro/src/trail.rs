use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use crate::assignments::Assignments;
use crate::cnf::{Clause, Lit};

/// Kappa denotes the conflict node.
pub const KAPPA: Lit = 0;

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

pub struct TrailNode {
    pub lit: Lit,
    pub decorator: TrailNodeDecorator,
}

pub struct Trail {
    list: Vec<TrailNode>,
    assign: Vec<bool>,
    dec_level: usize,
}

impl Trail {
    pub fn new(num_vars: usize) -> Self {
        Trail {
            list: Vec::with_capacity(num_vars + 1), // + 1 for conflict
            assign: vec!(false; num_vars * 2),
            dec_level: 0,
        }
    }

    pub fn push(&mut self, lit: Lit, decorator: TrailNodeDecorator) {
        if matches!(decorator, TrailNodeDecorator::Decision) {
            self.dec_level += 1;
        }

        self.list.push(TrailNode { lit, decorator });
        self.put_assign(&lit, true);
    }

    pub fn pop(&mut self) -> Option<TrailNode> {
        let top = self.list.pop();

        match &top {
            Some(TrailNode {
                lit,
                decorator,
            }) => {
                if matches!(decorator, TrailNodeDecorator::Decision) {
                    self.dec_level -= 1;
                }
                self.put_assign(lit, false);
            },
            _ => {},
        }

        top
    }

    pub fn remove(&mut self, lit: &Lit) {
        let ind = self.list.iter()
            .position(|n| lit.eq(&n.lit))
            .unwrap();
        self.list.remove(ind);
        self.put_assign(lit, false);
    }

    fn put_assign(&mut self, lit: &Lit, assigned: bool) {
        if lit == &KAPPA {
            return;
        }

        self.assign[Self::get_assign_ind(lit)] = assigned;
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

    fn get_assign_ind(lit: &Lit) -> usize {
        assert_ne!(lit, &0);
        (if lit < &0 {
            ((-lit) - 1) * 2
        } else {
            (lit - 1) * 2 + 1
        }) as usize
    }
}

impl Assignments for Trail {
    fn is_sat(&self, lit: &Lit) -> bool {
        self.assign[Self::get_assign_ind(lit)]
    }

    fn get_assignments(&self) -> HashSet<Lit> {
        self.list.iter()
            .map(|n| n.lit)
            .collect()
    }
}

impl Display for Trail {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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

        f.write_fmt(format_args!("digraph G {{ {dec_nodes} {edges} }}"))
    }
}