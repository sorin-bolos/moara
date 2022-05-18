#[macro_use]
pub mod complex;
pub mod gates;
pub mod gate_mapper;
pub mod circuit;
pub mod simulator;
pub mod engine;
pub mod measurement;


use num_complex::Complex32;

pub fn simulate(seralized_circuit_states:String, current_circuit_id:i32, shots:u32, endianess:Option<String>, qubit_count:Option<u8>) -> Vec<u32>
{
    simulator::simulate(seralized_circuit_states, current_circuit_id, shots, endianess, qubit_count)
}

pub fn get_statevector(seralized_circuit_states:String, current_circuit_id:i32, endianess:Option<String>, qubit_count:Option<u8>) -> Vec<Complex32>
{
    simulator::get_statevector(seralized_circuit_states, current_circuit_id, endianess, qubit_count)
}

pub fn get_probabilities(seralized_circuit_states:String, current_circuit_id:i32, endianess:Option<String>, qubit_count:Option<u8>) -> Vec<f32>
{   
    simulator::get_probabilities(seralized_circuit_states, current_circuit_id, endianess, qubit_count)
}