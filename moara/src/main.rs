use num_complex::Complex32;
use std::io::Write;
use std::fs::File;
use std::process;
use std::fs;
use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;

use moara;

fn main() {
    let config = Config::from_args();

    match config.command {
        Command::Sample { circuit_filename, qubit_count, shots, output } => {
            let serialized_circuit = read_file(circuit_filename).unwrap_or_else(|err| {
                println!("{}", err);
                process::exit(1);
            });

            let results = moara::simulate(serialized_circuit, shots, qubit_count);
            
            let results_str = format!("{:?}",results);
            output_results(results_str, output);
        },
        Command::Probabilities { circuit_filename, qubit_count, output } => {
            let serialized_circuit = read_file(circuit_filename).unwrap_or_else(|err| {
                println!("{}", err);
                process::exit(1);
            });

            let results = moara::get_probabilities(serialized_circuit, qubit_count);
            let results_str = format_f32(results);
            output_results(results_str, output);
        },
        Command::Statevector { circuit_filename, qubit_count, output } => {
            let serialized_circuit = read_file(circuit_filename).unwrap_or_else(|err| {
                println!("{}", err);
                process::exit(1);
            });

            let results = moara::get_statevector(serialized_circuit, qubit_count);
            let results_str = format_complex32(results);
            output_results(results_str, output);
        },
    }

    
}

fn read_file(circuit_filename:PathBuf) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(circuit_filename).unwrap();
    
    Ok(contents)
}

fn output_results(results_str:String, output:Option<PathBuf>) {
    match output {
        Some(filename) => {
            let mut f = File::create(filename).unwrap_or_else(|err| {
                println!("{}", err);
                process::exit(1);
            });
            f.write_all(results_str.as_bytes()).unwrap_or_else(|err| {
                println!("{}", err);
                process::exit(1);
            });
        },
        None => {
            print!("{}", results_str);
        }
    }
}

fn format_f32(results:Vec<f32>) -> String {
    let mut result = String::new();
    result.push_str("[");
    for r in &results[0..results.len() - 1] {
        if *r == 0.0 || *r == 1.0 {
            result.push_str(&r.to_string());
        } else {
            result.push_str(&format!("{:e}", r));
        }
        result.push_str(", ");
    }
    let r = &results[results.len()-1];
    if *r == 0.0 || *r == 1.0 {
        result.push_str(&r.to_string());
    } else {
        result.push_str(&format!("{:e}", r));
    }
    result.push_str("]");
    
    return result;
}

fn format_complex32(results:Vec<Complex32>) -> String {
    let mut result = String::new();
    let zero = Complex32::new(0.0,0.0);
    let one = Complex32::new(1.0,0.0);

    result.push_str("[");
    for r in &results[0..results.len() - 1] {
        if *r == zero || *r == one {
            result.push_str(&r.to_string());
        } else {
            result.push_str(&format!("{:e}", r));
        }
        result.push_str(", ");
    }
    let r = &results[results.len()-1];
    if *r == zero || *r == one {
        result.push_str(&r.to_string());
    } else {
        result.push_str(&format!("{:e}", r));
    }
    result.push_str("]");

    return result;
}

#[derive(StructOpt)]
#[structopt(about = "Moara quantum simulator")]
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

        #[structopt(short = "o", long = "output", help = "Output filename")]
        output:Option<PathBuf>
    },
    
    #[structopt(about = "Get the final real probabilities")]
    Probabilities{
        #[structopt(parse(from_os_str))]
        circuit_filename:PathBuf,
    
        #[structopt(short = "q", long = "qubits", help = "The number of qubits. Must be at least the width of the circuit.")]
        qubit_count:Option<u8>,

        #[structopt(short = "o", long = "output", help = "Output filename")]
        output:Option<PathBuf>
    },
    
    #[structopt(about = "Get the final statevector")]
    Statevector {
        #[structopt(parse(from_os_str))]
        circuit_filename:PathBuf,
    
        #[structopt(short = "q", long = "qubits", help = "The number of qubits. Must be at least the width of the circuit.")]
        qubit_count:Option<u8>,

        #[structopt(short = "o", long = "output", help = "Output filename")]
        output:Option<PathBuf>
    }
}