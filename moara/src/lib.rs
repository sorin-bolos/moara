#[macro_use]
pub mod complex;
pub mod gates;
pub mod gate_mapper;
pub mod circuit;
pub mod simulator;
pub mod engine;
pub mod measurement;


use num_complex::Complex32;

pub fn simulate(serialized_circuit:String, shots:u32, endianess:Option<String>, qubit_count:Option<u8>) -> Vec<u32>
{
    simulator::simulate(serialized_circuit, shots, endianess, qubit_count)
}

pub fn get_statevector(serialized_circuit:String, endianess:Option<String>, qubit_count:Option<u8>) -> Vec<Complex32>
{
    simulator::get_statevector(serialized_circuit, endianess, qubit_count)
}

pub fn get_probabilities(serialized_circuit:String, endianess:Option<String>, qubit_count:Option<u8>) -> Vec<f32>
{   
    simulator::get_probabilities(serialized_circuit, endianess, qubit_count)
}