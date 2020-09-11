use super::circuit::Circuit;
use super::circuit::Step;
use super::circuit::Gate;
use crate::operator::Operator;
use crate::measurement::measure;
use crate::statevector::Statevector;
use crate::vectors;
use crate::gates;

const MEASUREMENT: &str = "measure-z";

pub fn run(qubit_count:u8, circuit:Circuit, shots:u32) -> Vec<u32>
{
    let (final_statevector, _measurements) = get_final_statevector(qubit_count, circuit);

    (0..shots).map(|_| measure(&final_statevector))
              .fold(vec![0u32;1<<qubit_count],
                    |mut intermediate_results,measurement_result| {intermediate_results[measurement_result] += 1; intermediate_results})
}

fn get_final_statevector(qubit_count:u8, circuit:Circuit) -> (Statevector, Vec<bool>)
{
    let mut measurements = vec![false; qubit_count as usize];

    let mut ordered_steps = circuit.steps;
    ordered_steps.sort_by(|a, b| a.index.cmp(&b.index));

    let mut statevector:Option<Statevector> = None;

    for step in ordered_steps
    {
        match get_step_operator(qubit_count, &mut measurements, step)
        {
            Some(operator) => statevector = match statevector {
                                                None => Some(Statevector::new(operator.data()[..][0].to_vec())),
                                                Some(previous_statevector) => Some(operator.apply(previous_statevector))
                                            },
            None => {}
        }
    }

    match statevector {
        Some(final_statevector) => (final_statevector, measurements),
        None => (vectors::zero(qubit_count), measurements)
    }
}

fn get_step_operator(qubit_count:u8, measurements:&mut Vec<bool>, step:Step) -> Option<Operator>
{
    let mut ordered_gates = step.gates;
    ordered_gates.sort_by(|a, b| a.target.cmp(&b.target));

    let mut qubit_index = 0;
    let mut step_operator:Option<Operator> = None;
    
    for gate in ordered_gates
    {
        match get_intermediate_step_operator(measurements, gate, &mut qubit_index)
        {
            Some(next_operator) => step_operator = match step_operator {
                None => Some(next_operator),
                Some(previous_operator) => Some(previous_operator.tensor(&next_operator))
            },
            None => {}
        }
    }

    if qubit_count > qubit_index
    {
        step_operator = match step_operator {
            None => None,
            Some(previous_operator) => Some(previous_operator.tensor(&gates::identity(&qubit_count-qubit_index)))
        }
    }

    step_operator
}

fn get_intermediate_step_operator(measurements:&mut Vec<bool>, gate:Gate, qubit_index:&mut u8) -> Option<Operator>
{
    if measurements[gate.target as usize] 
    {
        panic!("Cannot add operator after measurement")
    }

    if gate.name == MEASUREMENT
    {
        measurements[gate.target as usize] = true;
        return None;
    }

    let mut next_operator = get_operator(&gate);
    if gate.target > *qubit_index
    {
        next_operator = gates::identity(gate.target-*qubit_index).tensor(&next_operator);
    }
    *qubit_index = gate.target+1;
    
    Some(next_operator)
}

fn get_operator(gate:&Gate) -> Operator
{
    match gate.name.as_ref() {
        "measure-z" => gates::identity(1),
        "pauli-x" => gates::pauli_x(),
        "pauli-y" => gates::pauli_y(),
        "pauli-z" => gates::pauli_z(),
        "hadamard" => gates::hadamard(),
        "ctrl-pauli-x" => {
            let control = match gate.control{
                Some(control_value) => control_value,
                None => panic!("ctrl-pauli-x for qubit {} has no value for control", gate.target)
            };
            gates::cx(((gate.target as i8-control as i8).abs()+1) as u8, control>gate.target)
        },
        "swap" => {
            let target2 = match gate.target2{
                Some(target2_value) => target2_value,
                None => panic!("swap for qubit {} has no value for target2", gate.target)
            };
            gates::swap(((gate.target as i8-target2 as i8).abs()+1) as u8)
        },
        nunknown_gate => panic!("Unknown operator {}", nunknown_gate)
    }
}