use super::circuit::Circuit;
use super::circuit::Step;
use super::circuit::Gate;
use crate::operator::Operator;
use crate::measurement::measure;
use crate::statevector::Statevector;
use crate::vectors::zero;
use num_complex::Complex32;

const MEASUREMENT: &str = "measure-z";

pub fn run(qubit_count:u8, circuit:Circuit, shots:u32) -> Vec<u32>
{
    let (final_statevector, measurements) = get_final_statevector(qubit_count, circuit);

    (0..shots).map(|_| measure(&final_statevector))
              .fold(vec![0u32;1<<qubit_count],
                    |mut intermediate_results,measurement_result| {intermediate_results[measurement_result] += 1; intermediate_results})
}

fn get_final_statevector(qubit_count:u8, circuit:Circuit) -> (Statevector, Vec<bool>)
{
    let mut measurements = vec![false; qubit_count as usize];

    let mut ordered_steps = circuit.steps;
    ordered_steps.sort_by(|a, b| b.index.cmp(&a.index));

    let mut statevector = match get_first_non_empty_step_operator(qubit_count, &mut measurements, &mut ordered_steps) 
    {
        Some(step_operator) => Statevector::new(step_operator.data()[..][0].to_vec()),
        None => return (zero(qubit_count), measurements)
    };

    for step in ordered_steps
    {
        match get_step_operator(qubit_count, &mut measurements, step)
        {
            Some(operator) => statevector = operator.apply(statevector),
            None => {}
        }
    }

    (statevector, measurements)
}

fn get_first_non_empty_step_operator(qubit_count:u8, measurements:&mut Vec<bool>, ordered_steps:&mut Vec<Step>) -> Option<Operator>
{
    loop
    {
        match ordered_steps.pop()
        {
            Some(step) => match get_step_operator(qubit_count, measurements, step)
            {
                None => {},
                some => return some,
            },
            None => return None
        }
    }
}

fn get_step_operator(qubit_count:u8, measurements:&mut Vec<bool>, step:Step) -> Option<Operator>
{
    let mut ordered_gates = step.gates;
    ordered_gates.sort_by(|a, b| b.target.cmp(&a.target));

    let mut qubit_index = 0;

    let mut step_operator = match get_first_non_measurement_gate_operator(measurements, &mut ordered_gates, &mut qubit_index) 
    {
        Some(first_step_operator) => first_step_operator,
        None => return None
    };
    
    for gate in ordered_gates
    {
        match get_intermediate_step_operator(measurements, gate, &mut qubit_index)
        {
            Some(next_operator) => step_operator = step_operator.tensor(next_operator),
            None => {}
        }
    }

    if qubit_count > qubit_index
    {
        step_operator = step_operator.tensor(get_identity(1 << (qubit_count-qubit_index)));
    }

    Some(step_operator)
}

fn get_first_non_measurement_gate_operator(measurements:&mut Vec<bool>, ordered_gates:&mut Vec<Gate>, qubit_index:&mut u8) -> Option<Operator>
{
    loop
    {
        match ordered_gates.pop()
        {
            Some(step) => match get_intermediate_step_operator(measurements, step, qubit_index)
            {
                None => {},
                some => return some,
            },
            None => return None
        }
    }
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

    let mut next_operator = get_operator(&gate.name);
    if gate.target > *qubit_index
    {
        next_operator = get_identity(1 << (gate.target-*qubit_index)).tensor(next_operator);
    }
    *qubit_index = gate.target+1;
    
    Some(next_operator)
}

fn get_identity(size:u8) -> Operator
{
    let mut matrix = vec![vec![Complex32::new(0.0,0.0);size as usize];size as usize];

    for i in 0..(size as usize)
    {
        matrix[i][i] = Complex32::new(1.0,0.0);
    }

    Operator::new(matrix)
}

fn get_operator(name:&str) -> Operator
{
    match name {
        "measure-z" => get_identity(1),
        "pauli-x" => Operator::new(vec![
                         vec![Complex32::new(0.0,0.0), Complex32::new(1.0,0.0)],
                         vec![Complex32::new(1.0,0.0), Complex32::new(0.0,0.0)]
                     ]),
        "pauli-z" => Operator::new(vec![
                        vec![Complex32::new(1.0,0.0), Complex32::new(0.0,0.0)],
                        vec![Complex32::new(0.0,0.0), Complex32::new(-1.0,0.0)]
                    ]),
        "pauli-y" => Operator::new(vec![
                        vec![Complex32::new(0.0,0.0), Complex32::new(0.0,1.0)],
                        vec![Complex32::new(0.0,-1.0), Complex32::new(0.0,0.0)]
                    ]),
        "hadamard" => Operator::new(vec![
                        vec![Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(1.0/2.0_f32.sqrt(),0.0)],
                        vec![Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(-1.0/2.0_f32.sqrt(),0.0)]
                    ]),
        _ => panic!("Unknown operator {}", name)
    }
}