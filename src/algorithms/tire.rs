use crate::contracts::{Algorithm, AlgorithmError};
use crate::data_structure::DataStructure;

pub struct TireAlgorithm;

impl Algorithm for TireAlgorithm {
    fn name(&self) -> &'static str {
        "tire"
    }

    fn description(&self) -> &'static str {
        "Placeholder tire algorithm entry point."
    }

    fn run(&self, _input: &DataStructure) -> Result<String, AlgorithmError> {
        Err(AlgorithmError::not_implemented(self.name()))
    }
}
