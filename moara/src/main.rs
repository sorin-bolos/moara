use std::process;
use std::fs;
use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;

use moara;

fn main() {
    let config = Config::from_args();

    match config.command {
        Command::Sample { circuit_filename, qubit_count, shots } => {
            let serialized_circuit = read_file(circuit_filename).unwrap_or_else(|err| {
                println!("{}", err);
                process::exit(1);
            });

            let results = moara::simulate(serialized_circuit, shots, qubit_count);
            print!("{:?}", results);
        },
        Command::Probabilities { circuit_filename, qubit_count } => {
            let serialized_circuit = read_file(circuit_filename).unwrap_or_else(|err| {
                println!("{}", err);
                process::exit(1);
            });

            let results = moara::get_probabilities(serialized_circuit, qubit_count);
            print!("{:?}", results);
        },
        Command::Statevector { circuit_filename, qubit_count } => {
            let serialized_circuit = read_file(circuit_filename).unwrap_or_else(|err| {
                println!("{}", err);
                process::exit(1);
            });

            let results = moara::get_statevector(serialized_circuit, qubit_count);
            print!("{:?}", results);
        },
    }

    
}

fn read_file(circuit_filename:PathBuf) -> Result<String, Box<dyn Error>>
{
    let contents = fs::read_to_string(circuit_filename).unwrap();
    
    Ok(contents)
}

#[derive(StructOpt)]
#[structopt(about = "Moara quatnum simultor")]
struct Config {
    #[structopt(flatten)]
    command:Command,
}

#[derive(StructOpt)]
enum Command {
    
    #[structopt(about = "Sample the circuit over a number of shots")]
    Sample {
        #[structopt(parse(from_os_str))]
        circuit_filename:PathBuf,
    
        #[structopt(short = "q", long = "qubits", help = "The number of qubits. Must be at least the width of the circuit.")]
        qubit_count:Option<u8>,

        #[structopt(short = "s", long = "shots", default_value = "1024", help = "The number of shots")]
        shots:u32,
    },
    
    #[structopt(about = "Get the final real probabilities")]
    Probabilities{
        #[structopt(parse(from_os_str))]
        circuit_filename:PathBuf,
    
        #[structopt(short = "q", long = "qubits", help = "The number of qubits. Must be at least the width of the circuit.")]
        qubit_count:Option<u8>
    },
    
    #[structopt(about = "Get the final statevector")]
    Statevector {
        #[structopt(parse(from_os_str))]
        circuit_filename:PathBuf,
    
        #[structopt(short = "q", long = "qubits", help = "The number of qubits. Must be at least the width of the circuit.")]
        qubit_count:Option<u8>
    }
}