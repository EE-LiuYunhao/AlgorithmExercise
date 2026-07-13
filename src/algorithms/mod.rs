mod dijkstra;
mod longest_consecutive_sequence;
mod tire;

use crate::contracts::Algorithm;

pub fn available_algorithms() -> Vec<Box<dyn Algorithm>> {
    vec![
        Box::new(tire::TireAlgorithm),
        Box::new(dijkstra::DijkstraAlgorithm),
        Box::new(longest_consecutive_sequence::LongestConsecutiveSequence),
    ]
}

pub fn create_algorithm(name: &str) -> Option<Box<dyn Algorithm>> {
    match name {
        "tire" => Some(Box::new(tire::TireAlgorithm)),
        "dijkstra" => Some(Box::new(dijkstra::DijkstraAlgorithm)),
        "longest-consecutive-sequence" | "lcs" | "longest_consecutive_sequence" => Some(Box::new(
            longest_consecutive_sequence::LongestConsecutiveSequence,
        )),
        _ => None,
    }
}
