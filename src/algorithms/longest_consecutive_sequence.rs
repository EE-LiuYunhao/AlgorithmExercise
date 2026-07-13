use std::collections::HashMap;

use crate::{
    contracts::{Algorithm, AlgorithmError},
    data_structure::{
        linked_list::{connect, create_node_ref, NodeRef},
        DataStructure,
    },
    debug::DebugPrinter,
};

pub(super) struct LongestConsecutiveSequence;

impl Algorithm for LongestConsecutiveSequence {
    fn name(&self) -> &'static str {
        "longest-consecutive-sequence"
    }

    fn description(&self) -> &'static str {
        concat!(
            "Find the length of the longest consecutive sequence in an int array. ",
            "The algorithm links adjacent values into consecutive chains and returns ",
            "the maximum chain length as a single int."
        )
    }

    fn run(
        &self,
        input: &DataStructure,
        debug: &DebugPrinter,
    ) -> Result<DataStructure, AlgorithmError> {
        let DataStructure::IntArray(i_arr) = input else {
            return Err(AlgorithmError::invalid_input("int-array", input));
        };
        if i_arr.is_empty() {
            return Ok(DataStructure::Int(0));
        }
        if i_arr.len() == 1 {
            return Ok(DataStructure::Int(1));
        }
        let lookup: HashMap<i32, NodeRef<i32>> = i_arr
            .iter()
            .map(|&x| (x, create_node_ref(x))) // 产出 (key, value) 元组
            .collect();

        let mut root = lookup.clone();

        for value in i_arr {
            let Some(value_sui) = lookup.get(value) else {
                return Err(AlgorithmError::new(format!(
                    "Lost the reference to node {value}"
                )));
            };
            if let Some(value_minus_1) = lookup.get(&(value - 1)) {
                connect(value_minus_1, value_sui, debug);
                root.remove(value);
            }
            if let Some(value_plus_1) = lookup.get(&(value + 1)) {
                connect(value_sui, value_plus_1, debug);
                root.remove(&(value + 1));
            }
        }

        Ok(DataStructure::Int(
            root.values()
                .map(|consecutive_sequence_head| {
                    debug.print(format!(
                        "iterate over the consecutive sequence: {}",
                        DataStructure::LinkedListInt(consecutive_sequence_head.clone())
                    ));
                    let mut linked_list_length_count = 0;

                    let mut nullable_node_ref = Some(consecutive_sequence_head.clone());
                    while let Some(node_ref) = nullable_node_ref {
                        linked_list_length_count += 1;
                        nullable_node_ref = node_ref.borrow().next.clone();
                    }
                    linked_list_length_count
                })
                .max()
                .unwrap_or(0),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::LongestConsecutiveSequence;
    use crate::{contracts::Algorithm, data_structure::DataStructure, debug::DebugPrinter};

    fn run_algorithm(
        input: DataStructure,
    ) -> Result<DataStructure, crate::contracts::AlgorithmError> {
        let algorithm = LongestConsecutiveSequence;
        algorithm.run(&input, &DebugPrinter::new(false))
    }

    #[test]
    fn rejects_non_int_array_input() {
        let error = run_algorithm(DataStructure::RawString("hello".to_string()))
            .expect_err("non int-array input should be rejected");

        assert_eq!(
            error.to_string(),
            "algorithm error: algorithm expected int-array, but received raw-string"
        );
    }

    #[test]
    fn returns_zero_for_empty_array() {
        let result =
            run_algorithm(DataStructure::IntArray(vec![])).expect("empty input should succeed");

        assert_eq!(result, DataStructure::Int(0));
    }

    #[test]
    fn returns_one_for_single_value() {
        let result = run_algorithm(DataStructure::IntArray(vec![7]))
            .expect("single-value input should succeed");

        assert_eq!(result, DataStructure::Int(1));
    }

    #[test]
    fn returns_longest_chain_length_for_mixed_values() {
        let result = run_algorithm(DataStructure::IntArray(vec![100, 4, 200, 1, 3, 2]))
            .expect("mixed input should succeed");

        assert_eq!(result, DataStructure::Int(4));
    }

    #[test]
    fn ignores_duplicates_when_counting_consecutive_length() {
        let result = run_algorithm(DataStructure::IntArray(vec![1, 2, 2, 3]))
            .expect("duplicate values should still succeed");

        assert_eq!(result, DataStructure::Int(3));
    }
}
