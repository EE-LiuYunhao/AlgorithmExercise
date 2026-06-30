mod tire;
mod dijkstra;

use crate::contracts::Algorithm;

pub fn available_algorithms() -> Vec<Box<dyn Algorithm>> {
    vec![Box::new(tire::TireAlgorithm), Box::new(dijkstra::DijkstraAlgorithm)]
}

pub fn create_algorithm(name: &str) -> Option<Box<dyn Algorithm>> {
    match name {
        "tire" => Some(Box::new(tire::TireAlgorithm)),
        "dijkstra" => Some(Box::new(dijkstra::DijkstraAlgorithm)),
        _ => None,
    }
}
