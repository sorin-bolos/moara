use std::cmp::max;
use std::cmp::min;
use std::collections::HashSet;
use std::collections::HashMap;
use num_complex::Complex32;
use super::circuit::Circuit;
use crate::circuit::Gate;
use crate::circuit::Control;
use super::gate_mapper;

const MEASUREMENT_X: &str = "measure-x";
const MEASUREMENT_Y: &str = "measure-y";
const MEASUREMENT_Z: &str = "measure-z";

const KNOWN_CONTROL_STATES: [&str; 6] = ["0", "1", "+", "-", "+i", "-i"];

pub fn get_final_statevector(circuits: HashMap<i32, Circuit>, current_circuit_id:i32, qubit_count:u8) -> (Vec<Complex32>, HashMap<u8,u8>) {
    
    let circuit = circuits[&current_circuit_id].clone();
    let mut ordered_steps = circuit.steps;
    ordered_steps.sort_by(|a, b| a.index.cmp(&b.index));

    let mut statevector = vec![C!(0); 1<<qubit_count];
    statevector[0] = C!(1);

    let qubit_start = 0;

    let mut measurements = HashMap::new();

    for step in ordered_steps {      
      let mut afected_qubits = HashSet::new(); 
      for gate in step.gates {
        let mut full_controls = Vec::new();
        apply_generic_gate(
          gate,
          false,
          step.index,
          qubit_start,
          qubit_count,
          &circuits,
          current_circuit_id,
          &mut full_controls,
          &mut statevector, 
          &mut measurements, 
          &mut afected_qubits); 
      }
    }

    (statevector, measurements)
}

fn apply_generic_gate(
  gate:Gate,
  take_hermitian_conjugate:bool,
  step:u32,
  qubit_start:u8,
  qubit_count:u8,
  circuits: &HashMap<i32, Circuit>,
  circuit_id:i32,
  full_controls:&mut Vec<Control>,
  statevector: &mut Vec<Complex32>,
  measurements: &mut HashMap<u8, u8>,
  afected_qubits: &mut HashSet<u8>) {

  // barrier gate is useful only when circuit is converted to Qiskit/QASM format
  // otherwise only provides visual separation betweeen different portions of a circuit 
  if gate.name == "barrier" {
    return;
  }

  let mut actual_targets = Vec::new();
  for target in &gate.targets {
    let actual_target = target + qubit_start;

    if measurements.contains_key(&actual_target) {
        panic!("The qubit {} has been measured. Cannot add gates at step {} after measurement.", actual_target, step);
    }

    if afected_qubits.contains(&actual_target) {
        panic!("The qubit {} is mentioned twice in step {} in circuit with id {}.", actual_target, step, circuit_id);
    }
    afected_qubits.insert(actual_target);
    actual_targets.push(actual_target);
  }
  
  for control in &gate.controls {
    let mut actual_control = control.clone();
    actual_control.target += qubit_start;

    if afected_qubits.contains(&actual_control.target) {
        panic!("The qubit {} is mentioned twice in step {} in circuit with id {}.", actual_control.target, step, circuit_id);
    }

    afected_qubits.insert(actual_control.target);

    if !KNOWN_CONTROL_STATES.contains(&&*actual_control.state) {
        panic!("Unknown state {} step {}", actual_control.state, step);
    }
    full_controls.push(actual_control);
  }

  full_controls.sort_by(|a, b| a.target.cmp(&b.target));

  if actual_targets.len() == 0 && gate.gates.len() == 0 {
    panic!("No targets and no aggregated gates provided for gate {} at step {} in circuit with id {}.", gate.name, step, circuit_id);
  }

  if gate.name == "aggregate" && actual_targets.len() > 1 {
    panic!("Too many targets for gate {} at step {} in circuit with id {}.", gate.name, step, circuit_id);
  } else if gate.name != "qft" && gate.name != "qft-dagger" && gate.name != "circuit" && actual_targets.len() > 2 {
    panic!("Too many targets for gate {} at step {} in circuit with id {}.", gate.name, step, circuit_id);
  }

  let current_full_controls = full_controls.to_vec();
  rotate_single_qubit_states_to_match_control_states(statevector, current_full_controls.to_vec(), qubit_count);

  if gate.name == "circuit" {
    let circuit_id = gate.circuit_id.unwrap();
    let qubit_start = actual_targets[0];
    let mut circuit_power = gate.circuit_power.unwrap();
    if take_hermitian_conjugate {
      circuit_power = - circuit_power;
    }
    if circuit_power > 0 {
      for _ in 0..circuit_power {
        let mut current_full_controls = full_controls.to_vec();
        apply_circuit_gate(qubit_start, qubit_count, circuits, circuit_id, &mut current_full_controls, statevector, measurements);
      }
    } else {
      circuit_power = - circuit_power;
      for _ in 0..circuit_power {
        let mut current_full_controls = full_controls.to_vec();
        apply_reverse_circuit_gate(qubit_start, qubit_count, circuits, circuit_id, &mut current_full_controls, statevector, measurements);
      }
    }
  } else if gate.name == "aggregate" {
    for aggregated_gate in &gate.gates {
      let aggregated_gate_targets = aggregated_gate.targets.to_vec(); 
      let single_target_gate = Gate {
        name: aggregated_gate.name.clone(),
        circuit_id: None,
        circuit_abbreviation: None,
        circuit_power: None,
        targets_expression: None,
        targets: aggregated_gate_targets.into_iter().map(|x| x + qubit_start).rev().collect(),
        controls: current_full_controls.to_vec(),
        phi: aggregated_gate.phi,
        theta: aggregated_gate.theta,
        lambda: aggregated_gate.lambda,
        root: aggregated_gate.root.clone(),
        bit: None,
        gates: Vec::new()
      };
      let single_qubit_operator = gate_mapper::get_single_qubit_operator(&single_target_gate, take_hermitian_conjugate);
      apply_operator(single_qubit_operator, statevector, single_target_gate.targets[0], single_target_gate.controls, qubit_count);
    }
  } else if gate.name == "qft" {
    if take_hermitian_conjugate {
      apply_qft_dagger_gate(statevector, actual_targets.to_vec(), current_full_controls.to_vec(), qubit_count);
    } else {
      apply_qft_gate(statevector,  actual_targets.to_vec(), current_full_controls.to_vec(), qubit_count);
    }
  } else if gate.name == "qft-dagger" {
    if take_hermitian_conjugate {
      apply_qft_gate(statevector,  actual_targets.to_vec(), current_full_controls.to_vec(), qubit_count);
    } else {
      apply_qft_dagger_gate(statevector, actual_targets.to_vec(), current_full_controls.to_vec(), qubit_count);
    }
  } else if actual_targets.len() == 2 {
    let multi_target_operator = gate_mapper::get_double_target_operator(&gate, take_hermitian_conjugate);
    apply_double_target_operator(multi_target_operator, statevector, actual_targets, current_full_controls.to_vec(), qubit_count);
  } else {
    let target = actual_targets[0];
    if gate.name == MEASUREMENT_X || gate.name == MEASUREMENT_Y || gate.name == MEASUREMENT_Z {
        let bit = match gate.bit { Some(bit) => bit, None => target };
        if bit >= qubit_count {
            panic!("Measurement bit cannot be larger than the qubit count - 1 ({}). Received {} for qubit {}.", qubit_count-1, bit, target);
        }
        measurements.insert(target, bit);
        if gate.name == MEASUREMENT_Z {
            return;
        }
    }

    let single_qubit_operator = gate_mapper::get_single_qubit_operator(&gate, take_hermitian_conjugate);
    apply_operator(single_qubit_operator, statevector, target, current_full_controls.to_vec(), qubit_count);
  }

  undo_rotate_single_qubit_states_to_match_control_states(statevector, full_controls.to_vec(), qubit_count);
}

fn apply_circuit_gate(
  qubit_start:u8,
  qubit_count:u8,
  circuits: &HashMap<i32, Circuit>, 
  circuit_id:i32,
  full_controls:&mut Vec<Control>,
  statevector: &mut Vec<Complex32>, 
  measurements: &mut HashMap<u8, u8>
) {

  let circuit = circuits[&circuit_id].clone();
  let mut ordered_steps = circuit.steps;
  ordered_steps.sort_by(|a, b| a.index.cmp(&b.index));

  for step in ordered_steps {
    let mut afected_qubits = HashSet::new();
    for gate in step.gates {
      let mut current_full_controls = full_controls.to_vec();
      apply_generic_gate(
        gate,
        false,
        step.index,
        qubit_start,
        qubit_count,
        &circuits,
        circuit_id,
        &mut current_full_controls,
        statevector, 
        measurements, 
        &mut afected_qubits);
    }
  }
}

fn apply_reverse_circuit_gate(
  qubit_start:u8,
  qubit_count:u8,
  circuits: &HashMap<i32, Circuit>, 
  circuit_id:i32,
  full_controls:&mut Vec<Control>,
  statevector: &mut Vec<Complex32>, 
  measurements: &mut HashMap<u8, u8>
) {

  let circuit = circuits[&circuit_id].clone();
  let mut ordered_steps = circuit.steps;
  ordered_steps.sort_by(|a, b| a.index.cmp(&b.index));

  for step in ordered_steps.iter().rev() {
    let mut afected_qubits = HashSet::new();
    for gate in step.gates.iter().rev() {
      let mut current_full_controls = full_controls.to_vec();
      apply_generic_gate(
        gate.clone(),
        true,
        step.index,
        qubit_start,
        qubit_count,
        &circuits,
        circuit_id,
        &mut current_full_controls,
        statevector, 
        measurements, 
        &mut afected_qubits);
    }
  }
}

fn rotate_single_qubit_states_to_match_control_states(statevector: &mut Vec<Complex32>, controls:Vec<Control>, qubit_count:u8){
  for control in controls {
    let target = control.target;
    let state = control.state;
    if state == "+" || state == "-" {
      let single_qubit_operator = gate_mapper::get_qubit_rotation_operator("X");
      apply_operator(single_qubit_operator, statevector, target, Vec::new(), qubit_count);
    } else if state == "+i" || state == "-i" {
      let single_qubit_operator = gate_mapper::get_qubit_rotation_operator("Y");
      apply_operator(single_qubit_operator, statevector, target, Vec::new(), qubit_count);
    }
  }
}

fn undo_rotate_single_qubit_states_to_match_control_states(statevector: &mut Vec<Complex32>, controls:Vec<Control>, qubit_count:u8){
  for control in controls {
    let target = control.target;
    let state = control.state;
    if state == "+" || state == "-" {
      let single_qubit_operator = gate_mapper::get_qubit_undo_rotation_operator("X");
      apply_operator(single_qubit_operator, statevector, target, Vec::new(), qubit_count);
    } else if state == "+i" || state == "-i" {
      let single_qubit_operator = gate_mapper::get_qubit_undo_rotation_operator("Y");
      apply_operator(single_qubit_operator, statevector, target, Vec::new(), qubit_count);
    }
  }
}

fn apply_operator(operator:[Complex32; 4], statevector: &mut Vec<Complex32>, target: u8, controls:Vec<Control>, qubit_count:u8) {
    let controls_count = controls.len() as u8;
    let n = 1 << (qubit_count - controls_count - 1);

    for i in 0..n {
        let mut n_size = qubit_count - controls_count;

        let mut affected = i;
        for control in &controls {
            let control_positon = if control.target < target { control.target } else { control.target-1 };
            let (affected0, affected1) = get_indexes(affected, control_positon, n_size);
            affected = if control.state == "1"  || control.state == "-" || control.state == "-i" { affected1 } else { affected0 };

            n_size += 1;
        }

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

fn apply_double_target_operator(operator:[Complex32; 16], statevector: &mut Vec<Complex32>, targets:Vec<u8>, controls:Vec<Control>, qubit_count:u8) {
    let controls_count = controls.len() as u8;
    let n = 1 << (qubit_count - controls_count - 2);

    let target1 = targets[0];
    let target2 = targets[1];

    for i in 0..n {
        let mut n_size = qubit_count - controls_count-1;

        let mut affected = i;
        for control in &controls {
            let control_positon = if control.target < target1 && control.target < target2 { control.target } 
                          else { if (control.target < target1 && control.target > target2) || (control.target > target1 && control.target < target2)  { control.target-1 }
                          else { control.target-2 } };

            let (affected0, affected1) = get_indexes(affected, control_positon, n_size);
            affected = if control.state == "1" || control.state == "-" || control.state == "-i" { affected1 } else { affected0 };

            n_size += 1;
        }

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

fn apply_qft_gate(statevector: &mut Vec<Complex32>, targets:Vec<u8>, controls:Vec<Control>, qubit_count:u8) {

    let no_targets = targets.len();

    for i in 0..no_targets {
        let mut pass_targets = Vec::new();
        for j in i..no_targets {
          pass_targets.push(targets[j]);
        }
        apply_qft_pass(statevector, pass_targets, controls.to_vec(), qubit_count);
    }

    swap_qubits(statevector, targets, controls, qubit_count);
}

fn apply_qft_dagger_gate(statevector: &mut Vec<Complex32>, targets:Vec<u8>, controls:Vec<Control>, qubit_count:u8) {

  swap_qubits(statevector, targets.to_vec(), controls.to_vec(), qubit_count);

  let no_targets = targets.len();

  for i in (0..no_targets).rev() {
      let mut pass_targets = Vec::new();
      for j in (i..no_targets).rev() {
        pass_targets.push(targets[j]);
      }
      apply_qft_dagger_pass(statevector, pass_targets, controls.to_vec(), qubit_count);
  }
}

fn apply_qft_pass(statevector: &mut Vec<Complex32>, targets:Vec<u8>, controls:Vec<Control>, qubit_count:u8) {

    let no_targets = targets.len();
    let target = targets[0];

    let hadamard_gate = get_hadamard_gate();
    let hadamard_gate_operator = gate_mapper::get_single_qubit_operator(&hadamard_gate, false);
    apply_operator(hadamard_gate_operator, statevector, target, controls.to_vec(), qubit_count);

    for i in 1..no_targets {
      let mut full_controls =  controls.to_vec();
      full_controls.push(Control { target:targets[i], state:String::from("1") });
      full_controls.sort_by(|a, b| a.target.cmp(&b.target));

      let pauli_z_root_gate = get_pauli_z_root_gate(i);
      let pauli_z_root_gate_operator = gate_mapper::get_single_qubit_operator(&pauli_z_root_gate, false);
      apply_operator(pauli_z_root_gate_operator, statevector, target, full_controls, qubit_count);
    }
}

fn apply_qft_dagger_pass(statevector: &mut Vec<Complex32>, targets:Vec<u8>, controls:Vec<Control>, qubit_count:u8) {

  let no_targets = targets.len();
  let target = targets[no_targets - 1];

  for i in 1..no_targets {
    let mut full_controls = controls.to_vec();
    full_controls.push(Control { target:targets[i - 1], state:String::from("1") });
    full_controls.sort_by(|a, b| a.target.cmp(&b.target));

    let pauli_z_root_dagger_gate = get_pauli_z_root_dagger_gate(no_targets - i);
    let pauli_z_root_dagger_gate_operator = gate_mapper::get_single_qubit_operator(&pauli_z_root_dagger_gate, false);
    apply_operator(pauli_z_root_dagger_gate_operator, statevector, target, full_controls, qubit_count);
  }

  let hadamard_gate = get_hadamard_gate();
  let hadamard_gate_operator = gate_mapper::get_single_qubit_operator(&hadamard_gate, false);
  apply_operator(hadamard_gate_operator, statevector, target, controls.to_vec(), qubit_count);
}

fn swap_qubits(statevector: &mut Vec<Complex32>, targets:Vec<u8>, controls:Vec<Control>, qubit_count:u8) {

  let no_targets = targets.len();
 
  for i in 0..no_targets {
    if i < no_targets - 1 - i {
      let mut swap_targets = Vec::new();
      swap_targets.push(targets[i]);
      swap_targets.push(targets[no_targets - 1 - i]);
      let swap_gate = get_swap_gate();
      let swap_gate_operator = gate_mapper::get_double_target_operator(&swap_gate, false);
      apply_double_target_operator(swap_gate_operator, statevector, swap_targets, controls.to_vec(), qubit_count);
    } else if i == no_targets - 1 - i {
      // do nothing
    } else {
      break;
    }
  }
}

fn get_swap_gate() -> Gate {

  Gate {
    name: String::from("swap"),
    circuit_id: None,
    circuit_abbreviation: None,
    circuit_power: None,
    targets_expression: None,
    targets: Vec::new(),
    controls: Vec::new(),
    phi: None,
    theta: None,
    lambda: None,
    root: None,
    bit: None,
    gates: Vec::new()
  }
}

fn get_hadamard_gate() -> Gate {

  Gate {
    name: String::from("hadamard"),
    circuit_id: None,
    circuit_abbreviation: None,
    circuit_power: None,
    targets_expression: None,
    targets: Vec::new(),
    controls: Vec::new(),
    phi: None,
    theta: None,
    lambda: None,
    root: None,
    bit: None,
    gates: Vec::new()
  }
}

fn get_pauli_z_root_gate(pow:usize) -> Gate {

  Gate {
    name: String::from("pauli-z-root"),
    circuit_id: None,
    circuit_abbreviation: None,
    circuit_power: None,
    targets_expression: None,
    targets: Vec::new(),
    controls: Vec::new(),
    phi: None,
    theta: None,
    lambda: None,
    root: Some(format!("1/2^{}", pow)),
    bit: None,
    gates: Vec::new()
  }
}

fn get_pauli_z_root_dagger_gate(pow:usize) -> Gate {

  Gate {
    name: String::from("pauli-z-root-dagger"),
    circuit_id: None,
    circuit_abbreviation: None,
    circuit_power: None,
    targets_expression: None,
    targets: Vec::new(),
    controls: Vec::new(),
    phi: None,
    theta: None,
    lambda: None,
    root: Some(format!("1/2^{}", pow)),
    bit: None,
    gates: Vec::new()
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