use num_complex::Complex32;
use std::io::{BufWriter, Write};
use std::fs::File;
use std::process;
use std::fs;
use std::error::Error;
use std::path::PathBuf;
use std::result::Result;
use structopt::StructOpt;

use moara::circuit::Circuit;
use moara::circuit::CircuitState;

use lazy_static::lazy_static;
use regex::Regex;
use moara;

fn main() {
    let config = Config::from_args();

    match config.command {
        Command::Sample { circuit_filenames, current_circuit_id, qubit_count, shots, endianess, output } => {
            
            let serialized_circuit_states = get_serialized_circuit_states(circuit_filenames);
            let results = moara::simulate(serde_json::to_string(&serialized_circuit_states).unwrap(), current_circuit_id, shots, endianess, qubit_count);
            output_u32(results, output);
        },
        Command::Probabilities { circuit_filenames, current_circuit_id, qubit_count, endianess, output } => {

            let serialized_circuit_states = get_serialized_circuit_states(circuit_filenames);   
            let results = moara::get_probabilities(serde_json::to_string(&serialized_circuit_states).unwrap(), current_circuit_id, endianess, qubit_count);
            output_f32(results, output);
        },
        Command::Statevector { circuit_filenames, current_circuit_id, qubit_count, endianess, output } => {
            
            let serialized_circuit_states = get_serialized_circuit_states(circuit_filenames);  
            let results = moara::get_statevector(serde_json::to_string(&serialized_circuit_states).unwrap(), current_circuit_id, endianess, qubit_count);
            output_complex32(results, output);
        },
    }

    
}

fn get_serialized_circuit_states(circuit_filenames: std::vec::Vec<std::path::PathBuf>) -> std::vec::Vec<moara::circuit::CircuitState> {

    lazy_static! {
      static ref CIRCUIT_ID_REGEX : Regex = Regex::new(
        r#""circuit_id":\s([\-]?[0-9]+),"#
      ).unwrap();
    }

    let mut seralized_circuit_states = Vec::new();

    for circuit_filename in circuit_filenames {
      let serialized_circuit = read_file(circuit_filename).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
      });
      
      let caps = CIRCUIT_ID_REGEX.captures(&serialized_circuit).unwrap();
      let circuit_id = caps.get(1).map_or("", |m| m.as_str());
      let circuit_id = circuit_id.parse::<i32>().unwrap();              
      let circuit: Circuit = serde_json::from_str(&serialized_circuit).unwrap();
      let circuit_state = CircuitState {circuit_id: circuit_id, circuit: circuit};
      seralized_circuit_states.push(circuit_state);
    }

    seralized_circuit_states
}

fn read_file(circuit_filename:PathBuf) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(circuit_filename).unwrap();
    
    Ok(contents)
}


fn output_u32(results:Vec<u32>, output:Option<PathBuf>) {
    match output {
        Some(filename) => {
            let f = File::create(filename).unwrap_or_else(|err| {
                println!("{}", err);
                process::exit(1);
            });
            let mut writer = BufWriter::new(f);
            write_u32(results, &mut writer);
        },
        None => {
            let mut writer = std::io::stdout();
            write_u32(results, &mut writer);
        }
    }
}

fn add_extension(path: &mut std::path::PathBuf, extension: impl AsRef<std::path::Path>) {
  match path.extension() {
      Some(ext) => {
          let mut ext = ext.to_os_string();
          ext.push(".");
          ext.push(extension.as_ref());
          path.set_extension(ext)
      }
      None => path.set_extension(extension.as_ref()),
  };
}

fn output_f32(results:Vec<f32>, output:Option<PathBuf>) {
    match output {
        Some(filename) => {
            let mut control_file = filename.clone();
            // writing simulation results
            let f = File::create(filename).unwrap_or_else(|err| {
                println!("{}", err);
                process::exit(1);
            });
            let mut writer = BufWriter::new(f);
            write_f32(results, &mut writer);
            // writing control file          
            add_extension(&mut control_file, "ctrl");
            let mut file = File::create(control_file).unwrap_or_else(|err| {
              println!("{}", err);
              process::exit(1);
            }); 
            if let Err(e) = file.write_all(b"done") {
              println!("Writing error: {}", e.to_string());
              process::exit(1);
            }
        },
        None => {
            let mut writer = std::io::stdout();
            write_f32(results, &mut writer);
        }
    }
}

fn output_complex32(results:Vec<Complex32>, output:Option<PathBuf>) {
    match output {
        Some(filename) => {
            let f = File::create(filename).unwrap_or_else(|err| {
                println!("{}", err);
                process::exit(1);
            });
            let mut writer = BufWriter::new(f);
            write_complex32(results, &mut writer);
        },
        None => {
            let mut writer = std::io::stdout();
            write_complex32(results, &mut writer);
        }
    }
}

fn write_u32(results:Vec<u32>, writer:&mut dyn Write) {
    writer.write("[".as_bytes()).unwrap_or_else(|err| { println!("{}", err); process::exit(1); });
    for r in &results[0..results.len() - 1] {
        if *r == 0 || *r == 1 {
            writer.write(&r.to_string().as_bytes()).unwrap_or_else(|err| { println!("{}", err); process::exit(1); });
        } else {
            writer.write(&format!("{:e}", r).as_bytes()).unwrap_or_else(|err| { println!("{}", err); process::exit(1); });
        }
        writer.write(", ".as_bytes()).unwrap_or_else(|err| { println!("{}", err); process::exit(1); });
    }
    let r = &results[results.len()-1];
    if *r == 0 || *r == 1 {
        writer.write(&r.to_string().as_bytes()).unwrap_or_else(|err| { println!("{}", err); process::exit(1); });
    } else {
        writer.write(&format!("{:e}", r).as_bytes()).unwrap_or_else(|err| { println!("{}", err); process::exit(1); });
    }
    writer.write("]".as_bytes()).unwrap_or_else(|err| { println!("{}", err); process::exit(1); });
    writer.flush().unwrap_or_else(|err| { println!("{}", err); process::exit(1); });
}

fn write_f32(results:Vec<f32>, writer:&mut dyn Write) {
    writer.write("[".as_bytes()).unwrap_or_else(|err| { println!("{}", err); process::exit(1); });
    for r in &results[0..results.len() - 1] {
        if *r == 0.0 || *r == 1.0 {
            writer.write(&r.to_string().as_bytes()).unwrap_or_else(|err| { println!("{}", err); process::exit(1); });
        } else {
            writer.write(&format!("{:e}", r).as_bytes()).unwrap_or_else(|err| { println!("{}", err); process::exit(1); });
        }
        writer.write(", ".as_bytes()).unwrap_or_else(|err| { println!("{}", err); process::exit(1); });
    }
    let r = &results[results.len()-1];
    if *r == 0.0 || *r == 1.0 {
        writer.write(&r.to_string().as_bytes()).unwrap_or_else(|err| { println!("{}", err); process::exit(1); });
    } else {
        writer.write(&format!("{:e}", r).as_bytes()).unwrap_or_else(|err| { println!("{}", err); process::exit(1); });
    }
    writer.write("]".as_bytes()).unwrap_or_else(|err| { println!("{}", err); process::exit(1); });
    writer.flush().unwrap_or_else(|err| { println!("{}", err); process::exit(1); });
}

fn write_complex32(results:Vec<Complex32>, writer:&mut dyn Write) {
    let zero = Complex32::new(0.0,0.0);
    let one = Complex32::new(1.0,0.0);

    writer.write("[".as_bytes()).unwrap_or_else(|err| { println!("{}", err); process::exit(1); });
    for r in &results[0..results.len() - 1] {
        if *r == zero || *r == one {
            writer.write(&r.to_string().as_bytes()).unwrap_or_else(|err| { println!("{}", err); process::exit(1); });
        } else {
            writer.write(&format!("{:e}", r).as_bytes()).unwrap_or_else(|err| { println!("{}", err); process::exit(1); });
        }
        writer.write(", ".as_bytes()).unwrap_or_else(|err| { println!("{}", err); process::exit(1); });
    }
    let r = &results[results.len()-1];
    if *r == zero || *r == one {
        writer.write(&r.to_string().as_bytes()).unwrap_or_else(|err| { println!("{}", err); process::exit(1); });
    } else {
        writer.write(&format!("{:e}", r).as_bytes()).unwrap_or_else(|err| { println!("{}", err); process::exit(1); });
    }
    writer.write("]".as_bytes()).unwrap_or_else(|err| { println!("{}", err); process::exit(1); });
    writer.flush().unwrap_or_else(|err| { println!("{}", err); process::exit(1); });
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
        circuit_filenames:Vec<PathBuf>,

        #[structopt(short = "c", long = "circuitid", help = "The id of the circuit to simulate.")]
        current_circuit_id:i32,
    
        #[structopt(short = "q", long = "qubits", help = "The number of qubits. Must be at least the width of the circuit.")]
        qubit_count:Option<u8>,

        #[structopt(short = "s", long = "shots", default_value = "1024", help = "The number of shots")]
        shots:u32,

        #[structopt(short = "e", long = "endianess", help = "Ordering for state vectors in returned array with results: 'bigendian' or 'littleendian'.")]
        endianess:Option<String>,

        #[structopt(short = "o", long = "output", help = "Output filename")]
        output:Option<PathBuf>,
    },
    
    #[structopt(about = "Get the final real probabilities")]
    Probabilities{
        #[structopt(parse(from_os_str))]
        circuit_filenames:Vec<PathBuf>,

        #[structopt(short = "c", long = "circuitid", help = "The id of the circuit to simulate.")]
        current_circuit_id:i32,
    
        #[structopt(short = "q", long = "qubits", help = "The number of qubits. Must be at least the width of the circuit.")]
        qubit_count:Option<u8>,

        #[structopt(short = "e", long = "endianess", help = "Ordering for state vectors in returned array with results: 'bigendian' or 'littleendian'.")]
        endianess:Option<String>,

        #[structopt(short = "o", long = "output", help = "Output filename")]
        output:Option<PathBuf>,
    },
    
    #[structopt(about = "Get the final statevector")]
    Statevector {
        #[structopt(parse(from_os_str))]
        circuit_filenames:Vec<PathBuf>,

        #[structopt(short = "c", long = "circuitid", help = "The id of the circuit to simulate.")]
        current_circuit_id:i32,
    
        #[structopt(short = "q", long = "qubits", help = "The number of qubits. Must be at least the width of the circuit.")]
        qubit_count:Option<u8>,

        #[structopt(short = "e", long = "endianess", help = "Ordering for state vectors in returned array with results: 'bigendian' or 'littleendian'.")]
        endianess:Option<String>,

        #[structopt(short = "o", long = "output", help = "Output filename")]
        output:Option<PathBuf>,
    }
}