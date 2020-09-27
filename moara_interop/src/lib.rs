use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::collections::HashMap;

use moara;

#[pyfunction]
#[text_signature = "(serialized_circuit, shots, qubit_count)"]
pub fn simulate(serialized_circuit:String, shots:u32, qubit_count:Option<u8>) -> PyResult<HashMap<String, u32>> {
    
    let result_vector = moara::simulate(serialized_circuit, shots, qubit_count);

    let results = convert_to_hashmap(result_vector);

    Ok(results)
}

#[pymodule]
fn moara_interop(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(simulate, m)?)?;

    Ok(())
}

fn convert_to_hashmap(result_vector:Vec<u32>) -> HashMap<String, u32> {
    let mut results = HashMap::new();
    for (i, count) in result_vector.iter().enumerate() {
        if *count > 0u32 {
            results.insert(format!("{:b}", i), *count);
        }
    }
    results
}