use super::*;
use rstest::*;

#[rstest]
#[case(vec![])]
#[case(vec![1])]
#[case(vec![0, 2, 4])]
fn add_node_test(#[case] nodes: Vec<u8>) {
    let di_graph = di_graph_of_nodes(&nodes);

    assert_eq!(nodes.len(), di_graph.nodes.len());
    nodes.iter()
        .for_each(|node| assert!(di_graph.nodes.contains_key(node)));
}

#[rstest]
#[case(vec![], vec![])]
#[case(vec![0, 1], vec![(0, 1)])]
#[case(vec![0, 1], vec![(0, 1), (1, 0)])]
#[case(vec![0, 1, 2], vec![(0, 1), (2, 2)])]
#[case(vec![0, 1, 2, 3], vec![(0, 1), (2, 3), (0, 2), (0, 3)])]
fn add_edge_test(#[case] nodes: Vec<u8>,
                 #[case] edges: Vec<(u8, u8)>) {
    let di_graph = di_graph_of_nodes_and_edges(&nodes, &edges);

    let in_out_ct = di_graph.nodes.values()
        .map(|node| node.incoming.len() + node.outgoing.len())
        .reduce(|acc, ct| acc + ct)
        .unwrap_or_default();

    assert_eq!(edges.len() * 2, in_out_ct);

    edges.iter()
        .for_each(|(src, dst)| {
            assert!(di_graph.get_parents(dst).contains(&src));
            assert!(di_graph.get_children(src).contains(&dst));
        });
}

fn di_graph_of_nodes<N : Eq + Hash + Clone>(nodes: &Vec<N>) -> DiGraph<N> {
    let mut di_graph = DiGraph::new();
    nodes.iter().for_each(|node| di_graph.add_node(node.clone()));

    di_graph
}

fn di_graph_of_nodes_and_edges<N : Eq + Hash + Clone>(nodes: &Vec<N>,
                                                      edges: &Vec<(N, N)>)
    -> DiGraph<N> {
    let mut di_graph = di_graph_of_nodes(nodes);
    edges.iter().for_each(|(src, dst)| di_graph.add_edge(src, dst));

    di_graph
}