#[macro_use]
pub mod complex;
pub mod tools;
pub mod operator;
pub mod statevector;
pub mod measurement;
pub mod uranium;
pub mod vectors;
pub mod gates;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::collections::HashMap;

pub fn simulate_with_uranium(serialized_circuit:String, shots:u32, qubit_count:Option<u8>) -> Vec<u32>
{
    uranium::simulator::simulate(serialized_circuit, shots, qubit_count)
}

#[pyfunction]
#[text_signature = "(serialized_circuit, shots, qubit_count)"]
pub fn simulate_qiskit(serialized_circuit:String, shots:u32, qubit_count:Option<u8>) -> PyResult<HashMap<String, u32>>
{
    let mut results = HashMap::new();
    results.insert("00".to_string(), 1);
    results.insert("01".to_string(), 2);
    results.insert("10".to_string(), 3);
    results.insert("11".to_string(), shots-6);

    Ok(results)
}

#[pymodule]
fn moara(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(simulate_qiskit, m)?)?;

    Ok(())
}