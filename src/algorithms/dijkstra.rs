use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use crate::{
    contracts::{Algorithm, AlgorithmError},
    data_structure::{graph::NodeRef, DataStructure},
};

pub struct DijkstraAlgorithm;

impl Algorithm for DijkstraAlgorithm {
    fn name(&self) -> &'static str {
        "dijkstra"
    }

    fn description(&self) -> &'static str {
        concat!(
            "Dijkstra algorithm to search a graph from the first-declared node to ",
            "all other nodes in the graph, for the min-distances. The returned value ",
            "will be a list of path in string like [\"ab\", \"ac\", \"abd\", \"abdfe\"",
            "...] where each string is the path of minimal total cost from the first ",
            "node `a` to each node `b`, `c`, `d`, `e`, etc.."
        )
    }

    fn run(
        &self,
        input: &DataStructure,
        debug: &crate::debug::DebugPrinter,
    ) -> Result<DataStructure, AlgorithmError> {
        let DataStructure::Graph(graph) = input else {
            return Err(AlgorithmError::invalid_input("graph", input));
        };
        if graph.len() <= 1 {
            return Ok(DataStructure::StringArray(vec![]));
        }

        let source_id = graph
            .first()
            .expect("graph length should be checked above")
            .borrow()
            .id
            .clone();
        let mut distances = HashMap::<String, i32>::with_capacity(graph.len());
        let mut previous = HashMap::<String, String>::with_capacity(graph.len().saturating_sub(1));
        // Note: Dijkstra algorithm MUST use a previous array, instead of a SPT tree structure
        //       directly. For example, a minimal path from Node A to Node C is Node A -> B -> C.
        //       Then in the previous node array, previous[B] is A, previous[C] is B.
        //       In the SPT tree, A's next is {B, }, and B's next is {C, }
        // The reason we must only use a previous in the iteration, and construct a SPT
        //       tree only after the Dijkstra is completed, is this scenario:
        //       At first, minimal path from A to C is Node A -> B -> C.
        //       Then, we found the path A -> D -> C is shorter.
        //       If using previous array, simply previous[C] = D should work. But if
        //       using SPT structure, simply D.next.append(C) is not sufficiently,
        //       because B.next also contains C, so we should iterate over the entire tree
        //       to find the current tree node pointing to C and remove it.
        let mut node_lookup = HashMap::<String, NodeRef>::with_capacity(graph.len());

        for node in graph.iter() {
            let node_id = node.borrow().id.clone();
            node_lookup.insert(node_id.clone(), node.clone());
            distances.insert(node_id, i32::MAX);
        }
        distances.insert(source_id.clone(), 0); // for the case if there a backloop from
                                                // nodes to the source.

        let mut frontier = BinaryHeap::<(Reverse<i32>, String)>::new();
        frontier.push((Reverse(0), source_id.clone()));

        while let Some((Reverse(dist), node)) = frontier.pop() {
            let current_min_distance = distances.get(&node).copied().unwrap_or(i32::MAX);
            if dist > current_min_distance {
                debug.print(format!(
                    "skip node {0} with distance {1} because the current distance {2}",
                    node, dist, current_min_distance
                ));
                continue;
            }

            let current_node = node_lookup
                .get(&node)
                .expect("all graph nodes should exist in node_lookup")
                .borrow();
            for (neighbor_ref, cost) in current_node.children.iter() {
                let neighbor_id = neighbor_ref.borrow().id.clone();
                if *cost < 0 {
                    return Err(AlgorithmError::new(format!(
                        "dijkstra does not support negative edge cost: {0} -> {1} = {2}",
                        node, neighbor_id, cost
                    )));
                }

                let new_distance = dist.saturating_add(*cost);
                debug.print(format!(
                    "relax edge: ({0} -> {1}) with cost {2} => new distance is {3}",
                    node, neighbor_id, cost, new_distance
                ));
                if new_distance < distances.get(&neighbor_id).copied().unwrap_or(i32::MAX) {
                    distances.insert(neighbor_id.clone(), new_distance);
                    previous.insert(neighbor_id.clone(), node.clone());
                    frontier.push((Reverse(new_distance), neighbor_id.clone()));
                    debug.print(format!(
                        "\tupdated path with new distance: now {0} -> {1}",
                        node, neighbor_id
                    ));
                } else {
                    debug.print(format!(
                        "\tskipped edge because the current path to {0} is smaller",
                        neighbor_id
                    ));
                }
            }
        }

        let mut result = Vec::<String>::with_capacity(graph.len() - 1);
        for node_ref in graph.iter().skip(1) {
            let node_id = node_ref.borrow().id.clone();
            let path = reconstruct_path(&previous, &source_id, &node_id).unwrap_or_default();
            result.push(path.join(" -> "));
        }
        Ok(DataStructure::StringArray(result))
    }
}

fn reconstruct_path(
    previous: &HashMap<String, String>,
    source_id: &str,
    target_id: &str,
) -> Option<Vec<String>> {
    let mut path = vec![target_id.to_string()];
    let mut current = target_id;

    while current != source_id {
        let prev = previous.get(current)?;
        path.push(prev.clone());
        current = prev;
    }

    path.reverse();
    Some(path)
}

#[cfg(test)]
mod tests {
    use super::DijkstraAlgorithm;
    use crate::{
        contracts::Algorithm,
        data_structure::{graph, DataStructure},
        debug::DebugPrinter,
    };

    fn run_dijkstra(nodes: Vec<graph::NodeRef>) -> DataStructure {
        let algorithm = DijkstraAlgorithm;
        algorithm
            .run(&DataStructure::Graph(nodes), &DebugPrinter::new(false))
            .expect("dijkstra should run successfully")
    }

    #[test]
    fn returns_path_for_two_node_graph() {
        let node_a = graph::create_node("NodeA".to_string());
        let node_b = graph::create_node("NodeB".to_string());
        node_a.borrow_mut().children.push((node_b.clone(), 3));

        let result = run_dijkstra(vec![node_a, node_b]);

        assert_eq!(
            result,
            DataStructure::StringArray(vec!["NodeA -> NodeB".to_string()])
        );
    }

    #[test]
    fn updates_predecessor_when_shorter_path_is_found() {
        let node_a = graph::create_node("NodeA".to_string());
        let node_b = graph::create_node("NodeB".to_string());
        let node_c = graph::create_node("NodeC".to_string());

        node_a.borrow_mut().children.push((node_b.clone(), 10));
        node_a.borrow_mut().children.push((node_c.clone(), 1));
        node_c.borrow_mut().children.push((node_b.clone(), 1));

        let result = run_dijkstra(vec![node_a, node_b, node_c]);

        assert_eq!(
            result,
            DataStructure::StringArray(vec![
                "NodeA -> NodeC -> NodeB".to_string(),
                "NodeA -> NodeC".to_string(),
            ])
        );
    }

    #[test]
    fn rejects_negative_edge_weights() {
        let node_a = graph::create_node("NodeA".to_string());
        let node_b = graph::create_node("NodeB".to_string());
        node_a.borrow_mut().children.push((node_b.clone(), -1));

        let algorithm = DijkstraAlgorithm;
        let error = algorithm
            .run(
                &DataStructure::Graph(vec![node_a, node_b]),
                &DebugPrinter::new(false),
            )
            .expect_err("negative edge weights should be rejected");

        assert_eq!(
            error.to_string(),
            "algorithm error: dijkstra does not support negative edge cost: NodeA -> NodeB = -1"
        );
    }
}
