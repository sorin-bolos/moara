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

pub fn simulate_with_uranium(serialized_circuit:String, shots:u32, qubit_count:Option<u8>) -> Vec<u32>
{
    uranium::simulator::simulate(serialized_circuit, shots, qubit_count)
}

#[pyfunction]
pub fn simulate_qiskit(serialized_circuit:String, shots:u32, qubit_count:Option<u8>) -> PyResult<String>
{
    Ok("Hello from rust".to_string())
}

#[pymodule]
fn moara(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(simulate_qiskit, m)?)?;

    Ok(())
}