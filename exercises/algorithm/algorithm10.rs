/*
    graph
    This problem requires you to implement a basic graph functio
*/

use std::collections::{HashMap, HashSet};
use std::fmt;
#[derive(Debug, Clone)]
pub struct NodeNotInGraph;
impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}
pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}
impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
    // fn add_edge(&mut self, edge: (&str, &str, i32)) {
    //     //TODO
    // }
}
pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;
    fn add_node(&mut self, node: &str) -> bool {
        if self.contains(node) {
            return false;
        }
        self.adjacency_table_mutable()
            .insert(node.to_string(), Vec::new());
        return true;
    }

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (from, to, weight) = edge;

        // 确保起始节点存在
        if !self.contains(from) {
            self.add_node(from);
        }
        // 确保目标节点存在
        if !self.contains(to) {
            self.add_node(to);
        }

        // 为起始节点添加边
        let from_adj_list = self.adjacency_table_mutable().get_mut(from).unwrap();
        if !from_adj_list.iter().any(|(n, w)| *n == to && *w == weight) {
            from_adj_list.push((to.to_string(), weight));
        }

        // 为目标节点添加反向边，因为是无向图
        let to_adj_list = self.adjacency_table_mutable().get_mut(to).unwrap();
        if !to_adj_list.iter().any(|(n, w)| *n == from && *w == weight) {
            to_adj_list.push((from.to_string(), weight));
        }
    }
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}
#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;
    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));
        let expected_edges = [
            ("a", "b", 5),
            ("b", "a", 5),
            ("c", "a", 7),
            ("a", "c", 7),
            ("b", "c", 10),
            ("c", "b", 10),
        ];
        for (expected_from, expected_to, expected_weight) in expected_edges.iter() {
            let found = graph.edges().iter().any(|(from, to, weight)| {
                from.as_str() == *expected_from
                    && to.as_str() == *expected_to
                    && *weight == *expected_weight
            });
            println!(
                "Expected edge: ({}, {}, {}), Found: {}",
                expected_from, expected_to, expected_weight, found
            );
            assert_eq!(found, true);
        }
    }
}
