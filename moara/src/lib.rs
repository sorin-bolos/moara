#[macro_use]
pub mod complex;
pub mod tools;
pub mod operator;
pub mod statevector;
pub mod measurement;
pub mod uranium;
pub mod vectors;
pub mod gates;

pub fn simulate_with_uranium(serialized_circuit:String, shots:u32, qubit_count:Option<u8>) -> Vec<u32>
{
    uranium::simulator::simulate(serialized_circuit, shots, qubit_count)
}