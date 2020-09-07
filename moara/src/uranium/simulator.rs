use super::circuit::Circuit;
use super::circuit::Step;
use super::circuit::Gate;
use crate::operator::Operator;
use crate::measurement::measure;
use crate::statevector::Statevector;
use crate::vectors::zero;
use num_complex::Complex32;

const MEASUREMENT: &str = "measure-z";

// pub fn run(qubit_count:u8, circuit:Circuit) -> usize
// {
//     let (operator_option, measurements) = get_circuit_operator(qubit_count, circuit);
//     match operator_option 
//     {
//         Some(circuit_operator) => return get_measurement_result(circuit_operator, measurements),
//         None => return 0
//     };
// }

// fn get_circuit_operator(qubit_count:u8, circuit:Circuit) -> (Option<Operator>, Vec<bool>)
// {
//     let mut measurements = vec![false; qubit_count as usize];

//     let mut ordered_steps = circuit.steps;
//     ordered_steps.sort_by(|a, b| b.index.cmp(&a.index));

//     let mut circuit_operator = match ordered_steps.pop() 
//     {
//         Some(first_step) => get_step_operator(&mut measurements, first_step),
//         None => return (None, measurements)
//     };

//     for step in ordered_steps
//     {
//         let step_operator = get_step_operator(&mut measurements, step);
//         circuit_operator = circuit_operator.dot(step_operator);
//     }

//     (Some(circuit_operator), measurements)
// }

// fn get_measurement_result(circuit_operator:Operator, measurements:Vec<bool>) -> usize
// {
//     let a = &circuit_operator.data()[..][0]; //TODO: change to reference
//     let final_statevector = Statevector::new(a.to_vec());
//     measure(&final_statevector)
// }

fn get_statevector(qubit_count:u8, circuit:Circuit) -> (Statevector, Vec<bool>)
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

    let mut step_operator = match ordered_gates.pop() 
    {
        Some(first_gate) => get_intermediate_step_operator(measurements, first_gate, &mut qubit_index),
        None => return None
    };
    for gate in ordered_gates
    {
        let next_operator = get_intermediate_step_operator(measurements, gate, &mut qubit_index);
        step_operator = step_operator.tensor(next_operator);
    }

    if qubit_count >= qubit_index
    {
        step_operator = step_operator.tensor(get_identity(1 << (qubit_count-qubit_index+1)));
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