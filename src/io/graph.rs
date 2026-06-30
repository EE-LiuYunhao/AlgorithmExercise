use crate::{contracts::{ParseError, Parser}, data_structure::{graph, DataStructure}};

pub struct GraphParser;

enum ScannerState {
    NodeIds, // the first [ ... ], before the ]
    NodeIdsDone, // done with the first [ ... ], after the ] but before the next [
    EdgeAwaitingParenthesis, // The second [ shows up, so awaiting the ( for the new edge. 
    EdgeAwaitingComma, // processing the edge::from, awaiting the `,` to mark the edge::to
    EdgeAwaitingSecondComma(String), // edge::from's process is done, comma is met, string -> the edge::from, processing the edge_from
    EdgeAwaitingCloseParenthesis(String, String), 
    EdgeAwaitingCommaAfterParenthesis, // completed (FROM, TO) edge tuple, so waiting for the next comma, to start processing the next edge::to
}

impl Parser for GraphParser {
    fn name(&self) -> &'static str {
        "graph"
    }

    fn description(&self) -> &'static str {
        concat!(
            "Parse the graph described by adjecent list from the input string.\n",
            "The input are expected to be of format: \n\t`[Node1, Node2, Node3, ...] ",
            " [(Node1, Node2), (Node1, Node3), ...]`\nconsisting of two components:\n",
            "\tthe first should be the list of node names, wrapped by `[` and `]`, with ",
            "each node name separated by `,` and an optional space.\n",
            "\tthe second should be an array of edges, wrapped also by `[` and `]`, with ",
            "each edge represented by a tuple of two node names enclosed by `(` and `)`, and ",
            "separated by `,` and an optional space.\n",
            "Optionally, each edge can be associated with an integer cost, using the syntax:\n\t",
            "[(Node1, Node2, Cost1), (Node2, Node3, Cost2), ...]\nas the edge list.\n\n",
            "The graph is expected to be directional. Hence, if a undirectional graph is ",
            "expected, both directions should be included in the edge list."
        )
    }

    fn parse(&self, input: &str) -> Result<DataStructure, ParseError> {
        let mut scanner = input.trim().chars();
        let mut scanner_state = ScannerState::NodeIds;
        match scanner.next() {
            Some('[') => {}
            _ => {
                return Err(ParseError::new(
                    "graph input must start with `[`",
                ))
            }
        }
        let mut parse_buffer = Vec::<char>::new();
        let mut nodes = Vec::<graph::NodeRef>::new();
        let add_node = |nodes: &mut Vec<graph::NodeRef>, buffer: &mut Vec::<char>| {
            let node_id = buffer.iter().collect();
            let found = nodes.iter().find(|node| node.borrow().id == node_id);
            if found.is_some() {
                return Err(ParseError::new(format!("Duplicate node name {}", node_id)));
            }
            nodes.push(graph::create_node(node_id));
            buffer.clear();
            Ok(())
        };

        let add_edge = |nodes: &mut Vec<graph::NodeRef>, from: String, to: String, cost: i32| {
            if from == to && cost == 0 {
                return Ok(()); // self-no-cost loop is okay, but omitted
            }
            
            let from_node = nodes.iter().find(|node| node.borrow().id == from);
            let to_node = nodes.iter().find(|node| node.borrow().id == to);

            if from_node.is_none() {
                return Err(ParseError::new(format!("Edge ({0}, {1}) is invalid because {0} is unknown", from, to)));
            }
            if to_node.is_none() {
                return Err(ParseError::new(format!("Edge ({0}, {1}) is invalid because {1} is unknown", from, to)));
            }

            from_node.unwrap().borrow_mut().children.push((to_node.unwrap().clone(), cost));
            Ok(())
        };

        loop {
            let char = scanner.next();
            match char {
                None => break,
                Some(c) => match c {
                    ',' => match scanner_state {
                            ScannerState::NodeIds => add_node(&mut nodes, &mut parse_buffer)?,
                            ScannerState::NodeIdsDone => return Err(ParseError::new("Unexpected `,` between the node list and the edge list. Only whitespaces are allowed here. ")),
                            ScannerState::EdgeAwaitingParenthesis => return Err(ParseError::new("Redundant ',' between the edge declaration. Edges should be in format [(..., ...), (..., ...), ...]")),
                            ScannerState::EdgeAwaitingComma => {
                                scanner_state = ScannerState::EdgeAwaitingSecondComma(parse_buffer.iter().collect());
                                parse_buffer.clear();
                            },
                            ScannerState::EdgeAwaitingSecondComma(from) => {
                                scanner_state = ScannerState::EdgeAwaitingCloseParenthesis(from, parse_buffer.iter().collect::<String>());
                                parse_buffer.clear();
                            },
                            ScannerState::EdgeAwaitingCloseParenthesis(_, _) => return Err(ParseError::new("An unexpected third ',' within one edge declaration. Each edge in format (FROM, TO, COST)")),
                            ScannerState::EdgeAwaitingCommaAfterParenthesis => scanner_state = ScannerState::EdgeAwaitingParenthesis,
                    },
                    ']' => match scanner_state {
                        ScannerState::NodeIds => {
                            scanner_state = ScannerState::NodeIdsDone;
                            add_node(&mut nodes, &mut parse_buffer)?;
                        },
                        ScannerState::EdgeAwaitingParenthesis | // (edge_last, edge_last), ] is valid
                        ScannerState::EdgeAwaitingCommaAfterParenthesis => return Ok(DataStructure::Graph(nodes)), // (edge_last, edge_last)] is valie
                        _ => return Err(ParseError::new("unexpected `]`")),
                    },
                    '[' => match scanner_state {
                        ScannerState::NodeIdsDone => scanner_state = ScannerState::EdgeAwaitingParenthesis,
                        _ => return Err(ParseError::new("Unexpected `]`")),
                    },
                    '(' => match scanner_state {
                        ScannerState::EdgeAwaitingParenthesis => scanner_state = ScannerState::EdgeAwaitingComma,
                        _ => return Err(ParseError::new("Unexpected `(`")),
                    },
                    ')' =>match scanner_state {
                        ScannerState::EdgeAwaitingSecondComma(from) => {
                            scanner_state = ScannerState::EdgeAwaitingCommaAfterParenthesis;
                            add_edge(&mut nodes, from,  parse_buffer.iter().collect::<String>(), 0)?;
                            parse_buffer.clear();
                        },
                        ScannerState::EdgeAwaitingCloseParenthesis(from, to) => {
                            scanner_state = ScannerState::EdgeAwaitingCommaAfterParenthesis;
                            add_edge(&mut nodes, from, to, parse_buffer.iter().collect::<String>().parse::<i32>()?)?;
                            parse_buffer.clear();
                        },
                        _ => return Err(ParseError::new("Unexpected `)`")),
                    },
                    _ => if c.is_whitespace() {
                        continue;
                    } else {
                        match scanner_state {
                            ScannerState::NodeIds |
                            ScannerState::EdgeAwaitingComma |
                            ScannerState::EdgeAwaitingSecondComma(_) |
                            ScannerState::EdgeAwaitingCloseParenthesis(_, _) => parse_buffer.push(c),
                            _ => return Err(ParseError::new(format!("unexpected character: {}, only white space should be here", c))),
                        };
                    },
                }
            }
        }
        Err(ParseError::new("unexpected EOF"))
    }
}

#[cfg(test)]
mod tests {
    use super::GraphParser;
    use crate::contracts::Parser;

    fn parse_to_string(input: &str) -> String {
        let parser = GraphParser;
        parser
            .parse(input)
            .expect("graph input should parse")
            .to_string()
    }

    #[test]
    fn parses_node_names_with_spaces_and_matches_edges_without_spaces_bits_ut() {
        let parsed = parse_to_string(
            "[Node A, NodeB, Node C] [(NodeA, NodeB), (Node B, Node C)]",
        );

        assert_eq!(
            parsed,
            "[\n\tNodeA -> [NodeB(0)],\n\tNodeB -> [NodeC(0)],\n\tNodeC -> []]"
        );
    }

    #[test]
    fn ignores_whitespace_anywhere_without_changing_result_bits_ut() {
        let compact = parse_to_string("[NodeA,NodeB,NodeC][(NodeA,NodeB),(NodeB,NodeC)]");
        let spaced = parse_to_string(
            " [ Node A ,  Node B,\n\tNode C ] \n [ ( Node A , Node B ) ,\t( NodeB , Node C ) ] ",
        );

        assert_eq!(compact, spaced);
    }

    #[test]
    fn ignores_self_loop_edges_bits_ut() {
        let parsed = parse_to_string("[NodeA, NodeB] [(NodeA, NodeA), (NodeA, NodeB)]");

        assert_eq!(parsed, "[\n\tNodeA -> [NodeB(0)],\n\tNodeB -> []]");
    }

    #[test]
    fn rejects_duplicate_nodes_after_whitespace_normalization_bits_ut() {
        let parser = GraphParser;
        let error = parser
            .parse("[NodeA, Node A, NodeB] []")
            .expect_err("duplicate nodes should be rejected");

        assert_eq!(
            error.to_string(),
            "input parse error: Duplicate node name NodeA"
        );
    }

    #[test]
    fn keeps_duplicate_edges_without_panicking_bits_ut() {
        let parsed = parse_to_string("[NodeA, NodeB] [(NodeA, NodeB), (Node A, Node B)]");

        assert_eq!(parsed, "[\n\tNodeA -> [NodeB(0), NodeB(0)],\n\tNodeB -> []]");
    }

    #[test]
    fn parses_weighted_edges_and_displays_costs_bits_ut() {
        let parsed =
            parse_to_string("[NodeA, NodeB, NodeC] [(NodeA, NodeB, 7), (NodeB, NodeC, -3)]");

        assert_eq!(
            parsed,
            "[\n\tNodeA -> [NodeB(7)],\n\tNodeB -> [NodeC(-3)],\n\tNodeC -> []]"
        );
    }

    #[test]
    fn supports_mixed_weighted_and_unweighted_edges_bits_ut() {
        let parsed = parse_to_string(
            "[NodeA, NodeB, NodeC] [(NodeA, NodeB), (NodeA, NodeC, 5)]",
        );

        assert_eq!(
            parsed,
            "[\n\tNodeA -> [NodeB(0), NodeC(5)],\n\tNodeB -> [],\n\tNodeC -> []]"
        );
    }

    #[test]
    fn rejects_malformed_graph_inputs_bits_ut() {
        let parser = GraphParser;
        let invalid_inputs = [
            "NodeA, NodeB] [(NodeA, NodeB)]",
            "[NodeA, NodeB [(NodeA, NodeB)]",
            "[NodeA, NodeB] [(NodeA, NodeB) (NodeC, NodeD)]",
            "[NodeA, NodeB] [(NodeA NodeB)]",
            "[NodeA, NodeB] [(NodeA, NodeB]",
            "[NodeA, NodeB] (NodeA, NodeB)]",
            "[NodeA, NodeB] [(NodeA, NodeB, )]",
            "[NodeA, NodeB] [(NodeA, NodeB, cost)]",
            "[NodeA, NodeB] [(NodeA, NodeB, 1, 2)]",
        ];

        for input in invalid_inputs {
            assert!(
                parser.parse(input).is_err(),
                "expected malformed input to fail: {input}"
            );
        }
    }
}
