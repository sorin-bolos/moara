use std::env;
use std::fs;
use std::process;
use std::error::Error;

use moara::uranium;

const INVALID_ARGUMENT_COUNT: &str = "Invalid number of arguments. Need to suply only one argument. Usage: 'moara.exe circuit.filename.json'";

fn main() {
    let args: Vec<String> = env::args().collect();
    let circuit_file = parse_arguments(args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });
    
    let serialized_circuit = read_file(circuit_file).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    let circuit: uranium::circuit::Circuit = serde_json::from_str(&serialized_circuit).unwrap();
    let results = uranium::simulator::run(3, circuit, 1024);
    print!("{:?}", results);
}

fn parse_arguments(input:Vec<String>) -> Result<String, &'static str>
{
    if input.len() != 2
    {
        return Err(INVALID_ARGUMENT_COUNT);
    }

    Ok(input[1].clone())
}

fn read_file(circuit_file:String) -> Result<String, Box<dyn Error>>
{
    let contents = fs::read_to_string(circuit_file)?;
    
    Ok(contents)
}