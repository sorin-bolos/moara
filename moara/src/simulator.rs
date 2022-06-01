use num_complex::Complex32;
use super::circuit::Circuit;
use super::circuit::CircuitState;
use super::engine;
use super::measurement;
use std::collections::HashMap;

pub fn simulate(seralized_circuit_states:String, current_circuit_id:i32, shots:u32, endianess:Option<String>, qubit_count:Option<u8>) -> Vec<u32> {
    let (circuits, count) = deserialize(seralized_circuit_states, current_circuit_id, qubit_count);

    if count == 0 {
        return vec![];
    }

    if shots == 0 {
        return vec![0; 1<<count];
    }

    let (final_statevector, measurements) = engine::get_final_statevector(circuits, current_circuit_id, count);

    let endianess:String = match endianess {
      Some(endianess) => endianess,
      None => String::from("bigendian"),
    };

    if endianess == String::from("bigendian") {
      measurement::measure(final_statevector, shots, measurements, count)
    } else if endianess == String::from("littleendian") {
      let measurements_vector = measurement::measure(final_statevector, shots, measurements, count);
      let qubit_count = (measurements_vector.len() as f64).log2() as u8;
      reorder_vector(measurements_vector, qubit_count)
    } else {
      panic!("endianess can be either: 'bigendian' or 'littleendian'")
    }
}

pub fn get_statevector(seralized_circuit_states:String, current_circuit_id:i32, endianess:Option<String>, qubit_count:Option<u8>) -> Vec<Complex32> {
    let (circuits, count) = deserialize(seralized_circuit_states, current_circuit_id, qubit_count);

    if count == 0 {
        return vec![];
    }

    let (final_statevector, _) = engine::get_final_statevector(circuits, current_circuit_id, count);

    let endianess:String = match endianess {
      Some(endianess) => endianess,
      None => String::from("bigendian"),
    };

    if endianess == String::from("bigendian") {
      final_statevector
    } else if endianess == String::from("littleendian") {
      reorder_vector(final_statevector, count)
    } else {
      panic!("endianess can be either: 'bigendian' or 'littleendian'")
    } 
}

pub fn get_probabilities(seralized_circuit_states:String, current_circuit_id:i32, endianess:Option<String>, qubit_count:Option<u8>) -> Vec<f32> {
    let (circuits, count) = deserialize(seralized_circuit_states, current_circuit_id, qubit_count);
    
    if count == 0 {
        return vec![];
    }

    let (statevector, measurements) = engine::get_final_statevector(circuits, current_circuit_id, count);

    let endianess:String = match endianess {
      Some(endianess) => endianess,
      None => String::from("bigendian"),
    };

    if endianess == String::from("bigendian") {
      measurement::get_probabilities(statevector, measurements, count)
    } else if endianess == String::from("littleendian") {
      let probabilities_vector = measurement::get_probabilities(statevector, measurements, count);
      let qubit_count = (probabilities_vector.len() as f64).log2() as u8;
      reorder_vector(probabilities_vector, qubit_count)
    } else {
      panic!("endianess can be either: 'bigendian' or 'littleendian'")
    }
}

fn reorder_vector<T>(mut vector:Vec<T>, qubit_count:u8) -> Vec<T> {
  let length = vector.len();
  for i in 0..length {
    let fst = i;
    let snd = get_reversed_qbits_state(qubit_count, i);
    if fst < snd {
      vector.swap(fst, snd);
    }
  }
  vector
}

fn get_reversed_qbits_state(mut qbits: u8, mut state: usize) -> usize
{
    let mut rev = 0;
    while qbits > 0 {

      // bitwise left shift 'rev' by 1
      rev <<= 1;
  
      // if current bit is '1'
      if state & 1 == 1 {
        rev ^= 1;
      }

      // bitwise right shift 'state' by 1
      state >>= 1;
      qbits -= 1;
    }
    rev
}

fn deserialize(seralized_circuit_states:String, current_circuit_id:i32, qubit_count:Option<u8>) -> (HashMap<i32, Circuit>, u8) {
    let circuit_states: Vec<CircuitState> = serde_json::from_str(&seralized_circuit_states).unwrap();

    let mut circuits = HashMap::new();

    for circuit_state in &circuit_states {
      let circuit_id = circuit_state.circuit_id;
      let circuit = circuit_state.circuit.clone();
      circuits.insert(circuit_id, circuit);
    }

    let count = match qubit_count {
        Some(working_qubit_count) => working_qubit_count,
        None => get_qubit_count_from_circuit(&circuits[&current_circuit_id])
    };

    (circuits, count)
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