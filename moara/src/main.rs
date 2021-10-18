use std::process;
use std::fs;
use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;

use moara;

// const USAGE:&str = "Usage: 'moara.exe circuit_filename.json 1024 4'";
// const INVALID_ARGUMENT_COUNT:&str = "Invalid number of arguments. Need to suply at least one argument.";
// const COULD_NOT_PARSE_QUBIT_COUNT:&str = "Could not parse argument for 'qubit_count'.";

fn main() {
    let config = Config::from_args();

    let serialized_circuit = read_file(config.circuit_filename).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    let results = moara::get_statevector(serialized_circuit, config.qubit_count);
    print!("{:?}", results);
}

fn read_file(circuit_filename:PathBuf) -> Result<String, Box<dyn Error>>
{
    let contents = fs::read_to_string(circuit_filename).unwrap();
    
    Ok(contents)
}

#[derive(StructOpt)]
struct Config {
    #[structopt(parse(from_os_str))]
    circuit_filename:PathBuf,
    //shots:u32,
    qubit_count:Option<u8>
}