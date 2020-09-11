use std::env;
use std::process;

use moara;

const USAGE:&str = "Usage: 'moara.exe circuit_filename.json 1024 4'";
const INVALID_ARGUMENT_COUNT:&str = "Invalid number of arguments. Need to suply at least one argument.";
const COULD_NOT_PARSE_SHOTS:&str = "Could not parse argument for 'shots'.";
const COULD_NOT_PARSE_QUBIT_COUNT:&str = "Could not parse argument for 'qubit_count'.";
const DEFAULT_SHOTS:u32 = 1024;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_arguments(args);

    let results = moara::simulate_with_uranium(config.circuit_filename, config.shots, config.qubit_count);
    print!("{:?}", results);
}

fn parse_arguments(input:Vec<String>) -> Config
{
    if input.len() < 2
    {
        println!("{} {}", INVALID_ARGUMENT_COUNT, USAGE);
        process::exit(1);
    }

    let circuit_filename = input[1].clone();
    let shots = match input.get(2) {
        Some(shots_arg) => shots_arg.parse::<u32>().unwrap_or_else(|_| {
            println!("{} {}", COULD_NOT_PARSE_SHOTS, USAGE);
            process::exit(1);
        }),
        None => DEFAULT_SHOTS
    };
    let qubit_count = match input.get(3) {
        Some(qubit_count_arg) => {
            let parsed_qubit_count_arg = qubit_count_arg.parse::<u8>().unwrap_or_else(|_| {
                println!("{} {}", COULD_NOT_PARSE_QUBIT_COUNT, USAGE);
                process::exit(1);
            });
            Some(parsed_qubit_count_arg)
        },
        None => None
    };

    Config{circuit_filename:circuit_filename, shots:shots, qubit_count:qubit_count}
}

struct Config {
    circuit_filename:String,
    shots:u32,
    qubit_count:Option<u8>
}