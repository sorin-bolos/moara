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
fn py_moara(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(simulate, m)?)?;

    Ok(())
}

fn convert_to_hashmap(result_vector:Vec<u32>) -> HashMap<String, u32> {
    let qubit_count = (result_vector.len() as f64).log2() as usize;
    let mut results = HashMap::new();
    for (i, count) in result_vector.iter().enumerate() {
        if *count > 0u32 {
            let binary = format!("{:b}", i);
            let padded = pad_to_width(binary, qubit_count);
            results.insert(padded, *count);
        }
    }
    results
}

fn pad_to_width(original:String, width:usize) -> String {
    if width <= original.len() {
        return original;
    }
    
    let pad_count = width - original.len();
    let mut padded = String::with_capacity(width);
    for _ in 0..pad_count {
        padded.push_str("0");
    }
    padded.push_str(&original);
    padded
}