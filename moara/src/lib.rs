#[macro_use]
pub mod complex;
pub mod tools;
pub mod operator;
pub mod statevector;
pub mod measurement;
pub mod uranium;
pub mod vectors;
pub mod gates;

use std::fs;
use std::process;
use std::error::Error;

pub fn simulate_with_uranium(circuit_filename:String, shots:u32, qubit_count:Option<u8>) -> Vec<u32>
{
    let serialized_circuit = read_file(circuit_filename).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    uranium::simulator::simulate(serialized_circuit, shots, qubit_count)
}

fn read_file(circuit_filename:String) -> Result<String, Box<dyn Error>>
{
    let contents = fs::read_to_string(circuit_filename)?;
    
    Ok(contents)
}