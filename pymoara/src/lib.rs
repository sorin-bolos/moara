use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use moara as moara_base;

#[pyfunction]
#[text_signature = "(serialized_circuit, shots, qubit_count)"]
pub fn simulate(serialized_circuit:String, shots:u32, qubit_count:Option<u8>) -> PyResult<Vec<u32>> {
    
    let result = moara_base::simulate(serialized_circuit, shots, qubit_count);

    Ok(result)
}

#[pymodule]
fn pymoara(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(simulate, m)?)?;

    Ok(())
}