mod dijkstra;
mod longest_consecutive_sequence;
mod tire;

use crate::contracts::Algorithm;

static TIRE_ALGORITHM: tire::TireAlgorithm = tire::TireAlgorithm;
static DIJKSTRA_ALGORITHM: dijkstra::DijkstraAlgorithm = dijkstra::DijkstraAlgorithm;
static LONGEST_CONSECUTIVE_SEQUENCE: longest_consecutive_sequence::LongestConsecutiveSequence =
    longest_consecutive_sequence::LongestConsecutiveSequence;

/// Returns all algorithms registered in the CLI.
///
/// The returned list is used for help output and registry-based lookups.
pub(crate) fn available_algorithms() -> [&'static dyn Algorithm; 3] {
    [
        &TIRE_ALGORITHM,
        &DIJKSTRA_ALGORITHM,
        &LONGEST_CONSECUTIVE_SEQUENCE,
    ]
}

/// Creates a registered algorithm instance by its canonical name.
pub(crate) fn create_algorithm(name: &str) -> Option<&'static dyn Algorithm> {
    available_algorithms()
        .into_iter()
        .find(|algorithm| algorithm.name() == name)
}
