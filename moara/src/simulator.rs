use  std::cmp::max;
use  std::cmp::min;
use std::collections::HashSet;
use num_complex::Complex32;
use rand::Rng;
use rand::prelude::ThreadRng;
use super::circuit::Circuit;
use super::gate_mapper;

const MEASUREMENT: &str = "measure-z";

pub fn simulate(serialized_circuit:String, shots:u32, qubit_count:Option<u8>) -> Vec<u32> {
    let circuit: Circuit = serde_json::from_str(&serialized_circuit).unwrap();

    let count = match qubit_count {
        Some(working_qubit_count) => working_qubit_count,
        None => get_qubit_count_from_circuit(&circuit)
    };

    run(count, circuit, shots)
}

pub fn get_statevector(serialized_circuit:String, qubit_count:Option<u8>) -> Vec<Complex32> {
    let circuit: Circuit = serde_json::from_str(&serialized_circuit).unwrap();

    let count = match qubit_count {
        Some(working_qubit_count) => working_qubit_count,
        None => get_qubit_count_from_circuit(&circuit)
    };

    if count == 0 {
        return vec![];
    }

    let (final_statevector, _) = get_final_statevector(count, circuit);

    final_statevector
}

pub fn get_probabilities(serialized_circuit:String, qubit_count:Option<u8>) -> Vec<f32> {
    let statevector = get_statevector(serialized_circuit, qubit_count);
    let len = statevector.len();
    let mut probabilities = vec![0f32; len];

    for i in 0..len {
        probabilities[i] = statevector[i].norm_sqr();
    }

    probabilities
}

fn run(qubit_count:u8, circuit:Circuit, shots:u32) -> Vec<u32> {
    if qubit_count == 0 {
        return vec![];
    }

    if shots == 0 {
        return vec![0; 1<<qubit_count];
    }

    let (final_statevector, _measurements) = get_final_statevector(qubit_count, circuit);

    let samples = measure(final_statevector, shots);

    samples
}

fn get_final_statevector(qubit_count:u8, circuit:Circuit) -> (Vec<Complex32>, Vec<bool>) {
    let mut measurements = vec![false; qubit_count as usize];

    let mut ordered_steps = circuit.steps;
    ordered_steps.sort_by(|a, b| a.index.cmp(&b.index));

    let mut statevector = vec![C!(0); 1<<qubit_count];
    statevector[0] = C!(1);

    for step in ordered_steps {
        
        let mut afected_qubits = HashSet::new();
        
        for gate in step.gates {

            if afected_qubits.contains(&gate.target) {
                panic!("The qubit {} is mentioned twice in step {}", gate.target, step.index);
            }
            afected_qubits.insert(gate.target);

            match gate.target2 {
                Some(target2) => {
                    match gate.control {
                        Some(qubit_control) => { 
                            if afected_qubits.contains(&qubit_control) {
                                panic!("The qubit {} is mentioned twice in step {}", qubit_control, step.index);
                            }
                            afected_qubits.insert(qubit_control);
                            
                            let target = gate.target;
                            let multi_target_operator = gate_mapper::get_operator_for_double_target_controlled(gate);
                            apply_controlled_double_target_operator(multi_target_operator, &mut statevector, target, target2, qubit_count, qubit_control);
                         }
                        None => {
                            let target = gate.target;
                            let multi_target_operator = gate_mapper::get_double_target_operator(gate);
                            apply_double_target_operator(multi_target_operator, &mut statevector, target, target2, qubit_count);
                        }
                    }
                },
                None => {
                    match gate.control {
                        Some(qubit_control) => {
                            if afected_qubits.contains(&qubit_control) {
                                panic!("The qubit {} is mentioned twice in step {}", qubit_control, step.index);
                            }
                            afected_qubits.insert(qubit_control);
                            
                            let target = gate.target;
                            let single_qubit_operator = gate_mapper::get_operator_for_controlled(gate);
                            apply_controlled_operator(single_qubit_operator, &mut statevector, target, qubit_count, qubit_control);
                        },
                        None => {
                            if gate.name == MEASUREMENT {
                                measurements[gate.target as usize] = true;
                                continue;
                            }
                            let target = gate.target;
                            let single_qubit_operator = gate_mapper::get_single_qubit_operator(gate);
                            apply_single_qubit_operator(single_qubit_operator, &mut statevector, target, qubit_count);
                        }
                    }
                }
            }
        }
    }

    (statevector, measurements)
}

fn apply_single_qubit_operator(operator:[Complex32; 4], statevector: &mut Vec<Complex32>, gate_position: u8, qubit_count:u8) {
    let n = 1 << (qubit_count - 1);
    for i in 0..n {
        let (index0, index1) = get_indexes(i, gate_position, qubit_count);
        let sv0 = statevector[index0];
        let sv1 = statevector[index1];
        
        let m00 = operator[0];
        let m01 = operator[1];
        let m10 = operator[2];
        let m11 = operator[3];

        statevector[index0] = m00*sv0 + m01*sv1;
        statevector[index1] = m10*sv0 + m11*sv1;
    }
}

fn apply_controlled_operator(operator:[Complex32; 4], statevector: &mut Vec<Complex32>, target: u8, qubit_count:u8, control:u8) {
    let n = 1 << (qubit_count - 2);

    let control_positon = if control < target { control } else { control-1 };
    
    for i in 0..n {
        let (_, affected) = get_indexes(i, control_positon, qubit_count-1);

        let (index0, index1) = get_indexes(affected, target, qubit_count);
        let sv0 = statevector[index0];
        let sv1 = statevector[index1];
        
        let m00 = operator[0];
        let m01 = operator[1];
        let m10 = operator[2];
        let m11 = operator[3];

        statevector[index0] = m00*sv0 + m01*sv1;
        statevector[index1] = m10*sv0 + m11*sv1;
    }
}

fn apply_controlled_double_target_operator(operator:[Complex32; 16], statevector: &mut Vec<Complex32>, target1: u8, target2: u8, qubit_count:u8, control:u8) {
    let n = 1 << (qubit_count - 3);

    let control_positon = if control < target1 && control < target2 { control } 
                          else { if (control < target1 && control > target2) || (control > target1 && control < target2)  { control-1 }
                                 else { control-2 } };
    
    for i in 0..n {
        let (_, affected) = get_indexes(i, control_positon, qubit_count-2);

        let first = max(target1, target2);
        let last = min(target1, target2);

        let (index0, index1) = get_indexes(affected, first, qubit_count);
        let (index00, index01) = get_indexes(index0, last, qubit_count);
        let (index10, index11) = get_indexes(index1, last, qubit_count);

        let sv00 = statevector[index00];
        let sv01 = statevector[index01];
        let sv10 = statevector[index10];
        let sv11 = statevector[index11];
        
        let m0000 = operator[0];
        let m0001 = operator[1];
        let m0010 = operator[2];
        let m0011 = operator[3];
        let m0100 = operator[4];
        let m0101 = operator[5];
        let m0110 = operator[6];
        let m0111 = operator[7];
        let m1000 = operator[8];
        let m1001 = operator[9];
        let m1010 = operator[10];
        let m1011 = operator[11];
        let m1100 = operator[12];
        let m1101 = operator[13];
        let m1110 = operator[14];
        let m1111 = operator[15];

        statevector[index00] = m0000*sv00 + m0001*sv01 + m0010*sv10 + m0011*sv11;
        statevector[index01] = m0100*sv00 + m0101*sv01 + m0110*sv10 + m0111*sv11;
        statevector[index10] = m1000*sv00 + m1001*sv01 + m1010*sv10 + m1011*sv11;
        statevector[index11] = m1100*sv00 + m1101*sv01 + m1110*sv10 + m1111*sv11;
    }
}

fn apply_double_target_operator(operator:[Complex32; 16], statevector: &mut Vec<Complex32>, gate_position1: u8, gate_position2: u8, qubit_count:u8) {
    let n = 1 << (qubit_count - 2);
    for i in 0..n {
        let first = max(gate_position2, gate_position1);
        let last = min(gate_position2, gate_position1);

        let (index0, index1) = get_indexes(i, first, qubit_count);
        let (index00, index01) = get_indexes(index0, last, qubit_count);
        let (index10, index11) = get_indexes(index1, last, qubit_count);

        let sv00 = statevector[index00];
        let sv01 = statevector[index01];
        let sv10 = statevector[index10];
        let sv11 = statevector[index11];
        
        let m0000 = operator[0];
        let m0001 = operator[1];
        let m0010 = operator[2];
        let m0011 = operator[3];
        let m0100 = operator[4];
        let m0101 = operator[5];
        let m0110 = operator[6];
        let m0111 = operator[7];
        let m1000 = operator[8];
        let m1001 = operator[9];
        let m1010 = operator[10];
        let m1011 = operator[11];
        let m1100 = operator[12];
        let m1101 = operator[13];
        let m1110 = operator[14];
        let m1111 = operator[15];

        statevector[index00] = m0000*sv00 + m0001*sv01 + m0010*sv10 + m0011*sv11;
        statevector[index01] = m0100*sv00 + m0101*sv01 + m0110*sv10 + m0111*sv11;
        statevector[index10] = m1000*sv00 + m1001*sv01 + m1010*sv10 + m1011*sv11;
        statevector[index11] = m1100*sv00 + m1101*sv01 + m1110*sv10 + m1111*sv11;
    }
}

fn get_indexes(i: usize, gate_position: u8, qubit_count:u8) -> (usize, usize){
    let reversed_gate_position = qubit_count - gate_position - 1;
    let lowbits = i & MASKS[usize::from(reversed_gate_position)];
    let remainder = (i >> reversed_gate_position) << (reversed_gate_position+1);
    let index0 = remainder | lowbits;
    let index1 = index0 | (1 << reversed_gate_position);

    (index0, index1)
}

fn measure(statevector:Vec<Complex32>, shots:u32) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let len = statevector.len();
    let mut measurement_results = vec![0u32; len];
    
    let mut i = 0;
    while i < shots {
        let sample = sample(&statevector, &mut rng, len);
        measurement_results[sample] += 1;
        i += 1;
    }

    measurement_results
}

fn sample(statevector:&Vec<Complex32>, rng: &mut ThreadRng, len:usize) -> usize {
    let sample:f64 = rng.gen();
    let mut running_sum:f64 = 0.0;
    let mut i = 0;
    while i < len {
        let re = statevector[i].re as f64;
        let im = statevector[i].im as f64;
        running_sum += re*re + im*im;
        if sample < running_sum
        {
            return i;
        }
        i += 1;
    }

    panic!("Sample was not in the expected interval");
}

pub fn get_qubit_count_from_circuit(circuit:&Circuit) -> u8 {
    let mut qubit_count = 0;

    for step in &circuit.steps {
        for gate in &step.gates {

            let mut mx = gate.target;
            match gate.target2 {
                Some(target2) => mx = max(mx, target2),
                None => {}
            };
            match gate.control {
                Some(control) => mx = max(mx, control),
                None => {}
            };
            // match gate.control2 {
            //     Some(control2) => mx = max(mx, control2),
            //     None => {}
            // };

            if mx+1 > qubit_count {
                qubit_count = mx+1;
            }
        }
    }

    qubit_count
}

#[cfg(target_pointer_width = "64")]
const MASKS: [usize; 64] = [
    0, 1, 3, 7, 15, 31, 63, 127, 255, 511, 1023, 2047, 4095, 8191, 16383, 32767, 65535, 131071, 262143,
    524287, 1048575, 2097151, 4194303, 8388607, 16777215, 33554431, 67108863, 134217727, 268435455,
    536870911, 1073741823, 2147483647, 4294967295, 8589934591, 17179869183, 34359738367, 68719476735,
    137438953471, 274877906943, 549755813887, 1099511627775, 2199023255551, 4398046511103, 8796093022207,
    17592186044415, 35184372088831, 70368744177663, 140737488355327, 281474976710655, 562949953421311,
    1125899906842623, 2251799813685247, 4503599627370495, 9007199254740991, 18014398509481983,
    36028797018963967, 72057594037927935, 144115188075855871, 288230376151711743, 576460752303423487,
    1152921504606846975, 2305843009213693951, 4611686018427387903, 9223372036854775807
];

#[cfg(target_pointer_width = "32")]
const MASKS: [usize; 32] = [
    0, 1, 3, 7, 15, 31, 63, 127, 255, 511, 1023, 2047, 4095, 8191, 16383, 32767, 65535, 131071, 262143,
    524287, 1048575, 2097151, 4194303, 8388607, 16777215, 33554431, 67108863, 134217727, 268435455,
    536870911, 1073741823, 2147483647
];