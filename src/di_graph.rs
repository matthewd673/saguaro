#[cfg(test)]
mod tests;

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

struct Edges<N> {
    incoming: HashSet<N>,
    outgoing: HashSet<N>,
}

impl<N> Edges<N> {
    fn new() -> Self {
        Edges {
            incoming: HashSet::new(),
            outgoing: HashSet::new(),
        }
    }
}

pub struct DiGraph<N> {
    nodes: HashMap<N, Edges<N>>,
}

impl<N : Eq + Hash + Clone> DiGraph<N> {
    /**
     * Create a new directed graph.
     */
    pub fn new() -> Self {
        DiGraph {
            nodes: HashMap::new(),
        }
    }

    /**
     * Add a given node to the graph.
     */
    pub fn add_node(&mut self, node: N) {
        self.nodes.insert(node, Edges::new());
    }

    /**
     * Add an edge between the given nodes to the graph. Both nodes must already
     * exist in the graph.
     */
    pub fn add_edge(&mut self, src: &N, dst: &N) {
        self.nodes.get_mut(&src).unwrap().outgoing.insert(dst.clone());
        self.nodes.get_mut(&dst).unwrap().incoming.insert(src.clone());
    }

    /**
     * Get the set of parent nodes for a given node. That is, the set of all
     * nodes in the graph that have edges which originate from them and
     * terminate at the given node.
     */
    pub fn get_parents(&self, node: &N) -> &HashSet<N> {
        &self.nodes.get(node).unwrap().incoming
    }

    /**
     * Get the set of all child nodes for a given node. That is, the set of all
     * nodes in the graph that have edges which originate from the given node
     * and terminate at them.
     */
    pub fn get_children(&self, node: &N) -> &HashSet<N> {
        &self.nodes.get(node).unwrap().outgoing
    }
}