use num_complex::Complex32;
use super::circuit::Circuit;
use super::engine;
use super::measurement;

pub fn simulate(serialized_circuit:String, shots:u32, qubit_count:Option<u8>) -> Vec<u32> {
    let (circuit, count) = deserialize(serialized_circuit, qubit_count);

    if count == 0 {
        return vec![];
    }

    if shots == 0 {
        return vec![0; 1<<count];
    }

    let (final_statevector, measurements) = engine::get_final_statevector(count, circuit);

    let samples = measurement::measure(final_statevector, shots, measurements, count);

    samples
}

pub fn get_statevector(serialized_circuit:String, qubit_count:Option<u8>) -> Vec<Complex32> {
    let (circuit, count) = deserialize(serialized_circuit, qubit_count);

    if count == 0 {
        return vec![];
    }

    let (final_statevector, _) = engine::get_final_statevector(count, circuit);

    final_statevector
}

pub fn get_probabilities(serialized_circuit:String, qubit_count:Option<u8>) -> Vec<f32> {
    let (circuit, count) = deserialize(serialized_circuit, qubit_count);

    if count == 0 {
        return vec![];
    }

    let (statevector, measurements) = engine::get_final_statevector(count, circuit);
    
    measurement::get_probabilities(statevector, measurements, count)
}

fn deserialize(serialized_circuit:String, qubit_count:Option<u8>) -> (Circuit, u8) {
    let circuit: Circuit = serde_json::from_str(&serialized_circuit).unwrap();

    let count = match qubit_count {
        Some(working_qubit_count) => working_qubit_count,
        None => get_qubit_count_from_circuit(&circuit)
    };

    (circuit, count)
}

fn get_qubit_count_from_circuit(circuit:&Circuit) -> u8 {
    let mut qubit_count = 0;

    for step in &circuit.steps {
        for gate in &step.gates {

            let mx = gate.get_max_qubit_index();

            if mx+1 > qubit_count {
                qubit_count = mx+1;
            }
        }
    }

    qubit_count
}