use num_complex::Complex32;
use super::circuit::Circuit;
use super::engine;
use super::measurement;

pub fn simulate(serialized_circuit:String, shots:u32, endianess:Option<String>, qubit_count:Option<u8>) -> Vec<u32> {
    let (circuit, count) = deserialize(serialized_circuit, qubit_count);

    if count == 0 {
        return vec![];
    }

    if shots == 0 {
        return vec![0; 1<<count];
    }

    let (final_statevector, measurements) = engine::get_final_statevector(count, circuit);

    let endianess:String = match endianess {
      Some(endianess) => endianess,
      None => String::from("bigendian"),
    };

    if endianess == String::from("bigendian") {
      measurement::measure(final_statevector, shots, measurements, count)
    } else if endianess == String::from("littleendian") {
      let reordered_state_vector = reorder_state_vector(final_statevector, count);
      measurement::measure(reordered_state_vector, shots, measurements, count)
    } else {
      panic!("endianess can be either: 'bigendian' or 'littleendian'")
    }
}

pub fn get_statevector(serialized_circuit:String, endianess:Option<String>, qubit_count:Option<u8>) -> Vec<Complex32> {
    let (circuit, count) = deserialize(serialized_circuit, qubit_count);

    if count == 0 {
        return vec![];
    }

    let (final_statevector, _) = engine::get_final_statevector(count, circuit);

    let endianess:String = match endianess {
      Some(endianess) => endianess,
      None => String::from("bigendian"),
    };

    if endianess == String::from("bigendian") {
      final_statevector
    } else if endianess == String::from("littleendian") {
      reorder_state_vector(final_statevector, count)
    } else {
      panic!("endianess can be either: 'bigendian' or 'littleendian'")
    } 
}

pub fn get_probabilities(serialized_circuit:String, endianess:Option<String>, qubit_count:Option<u8>) -> Vec<f32> {
    let (circuit, count) = deserialize(serialized_circuit, qubit_count);
    
    if count == 0 {
        return vec![];
    }

    let (statevector, measurements) = engine::get_final_statevector(count, circuit);

    let endianess:String = match endianess {
      Some(endianess) => endianess,
      None => String::from("bigendian"),
    };

    if endianess == String::from("bigendian") {
      measurement::get_probabilities(statevector, measurements, count)
    } else if endianess == String::from("littleendian") {
      let reordered_state_vector = reorder_state_vector(statevector, count);
      measurement::get_probabilities(reordered_state_vector, measurements, count)
    } else {
      panic!("endianess can be either: 'bigendian' or 'littleendian'")
    }
}

fn reorder_state_vector(mut statevector:Vec<Complex32>, qubit_count:u8) -> Vec<Complex32> {
  let length = statevector.len();
  for i in 0..length {
    let fst = i;
    let snd = get_reversed_qbits_state(qubit_count, i);
    if fst < snd {
      let tmp = statevector[fst];
      statevector[fst] = statevector[snd];
      statevector[snd] = tmp;
    }
  }
  statevector
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