#[macro_use]
pub mod complex;
pub mod tools;
pub mod gates;
pub mod circuit;
pub mod simulator;

pub fn simulate(serialized_circuit:String, shots:u32, qubit_count:Option<u8>) -> Vec<u32>
{
    simulator::simulate(serialized_circuit, shots, qubit_count)
}