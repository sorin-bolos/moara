use std::collections::HashSet;
use num_complex::Complex32;
use super::circuit::Circuit;
use super::circuit::Gate;
use crate::operator::Operator;
use crate::operator::MatrixOperator;
use crate::statevector::Statevector;
use crate::measurement::measure;
use crate::gates;

const MEASUREMENT: &str = "measure-z";

pub fn simulate(serialized_circuit:String, shots:u32, qubit_count:Option<u8>) -> Vec<u32>
{
    let circuit: Circuit = serde_json::from_str(&serialized_circuit).unwrap();

    let count = match qubit_count {
        Some(working_qubit_count) => working_qubit_count,
        None => get_qubit_count_from_circuit(&circuit)
    };

    run(count, circuit, shots)
}

fn run(qubit_count:u8, circuit:Circuit, shots:u32) -> Vec<u32>
{
    if qubit_count == 0 {
        return vec![];
    }

    if shots == 0 {
        return vec![0; 1<<qubit_count];
    }

    let (final_statevector, _measurements) = get_final_statevector(qubit_count, circuit);

    let s_final_statevector = Statevector::new(final_statevector);
    (0..shots).map(|_| measure(&s_final_statevector))
              .fold(vec![0u32;1<<qubit_count],
                    |mut intermediate_results,measurement_result| {intermediate_results[measurement_result] += 1; intermediate_results})
}

fn get_final_statevector(qubit_count:u8, circuit:Circuit) -> (Vec<Complex32>, Vec<bool>)
{
    let mut measurements = vec![false; qubit_count as usize];

    let mut ordered_steps = circuit.steps;
    ordered_steps.sort_by(|a, b| a.index.cmp(&b.index));

    let mut statevector = vec![C!(0); 1<<qubit_count];
    statevector[0] = C!(1);

    for step in ordered_steps {
        
        let mut afected_qubits = HashSet::new();
        
        for gate in step.gates {
            let mut is_controlled = false;
            let mut is_multi_target = false;

            if afected_qubits.contains(&gate.target) {
                panic!("The qubit {} is mentioned twice in step {}", gate.target, step.index);
            }
            afected_qubits.insert(gate.target);
            match gate.target2 {
                Some(qubit_target2) => {
                    if afected_qubits.contains(&qubit_target2) {
                        panic!("The qubit {} is mentioned twice in step {}", qubit_target2, step.index);
                    }
                    afected_qubits.insert(qubit_target2);
                    is_multi_target = true;
                },
                None => {}
            }
            match gate.control {
                Some(qubit_control) => {
                    if afected_qubits.contains(&qubit_control) {
                        panic!("The qubit {} is mentioned twice in step {}", qubit_control, step.index);
                    }
                    afected_qubits.insert(qubit_control);
                    is_controlled = true;
                },
                None => {}
            }
            
            if gate.name == MEASUREMENT
            {
                measurements[gate.target as usize] = true;
                continue;
            }

            if is_controlled || is_multi_target {
                
            } else {
                let singel_qubit_operator = get_singel_qubit_operator(&gate);
                apply_singel_qubit_operator(singel_qubit_operator, &mut statevector, gate.target, qubit_count);
            }
            
        }
    }

    (statevector, measurements)
}

fn apply_singel_qubit_operator(operator:MatrixOperator, statevector: &mut Vec<Complex32>, gate_position: u8, qubit_count:u8) {
    let n = 1 << (qubit_count - 1);
    let m00 = operator.get(0,0);
    let m01 = operator.get(0,1);
    let m10 = operator.get(1,0);
    let m11 = operator.get(1,1);
    for i in 0..n {
        let (index0, index1) = get_indexes(i, gate_position, qubit_count);
        let sv0 = statevector[index0];
        let sv1 = statevector[index1];

        statevector[index0] = m00*sv0 + m01*sv1;
        statevector[index1] = m10*sv0 + m11*sv1;
    }
}

fn get_indexes(i: usize, gate_position: u8, qubit_count:u8) -> (usize, usize){
    let reversed_gate_position = qubit_count - gate_position - 1;
    let left_shift = gate_position+1;
    let modulo = (i << left_shift) >> left_shift;
    let remainder = (i >> reversed_gate_position) << (reversed_gate_position+1);
    let index0 = remainder | modulo;
    let index1 = index0 | (1 << reversed_gate_position);

    (index0, index1)
}

fn get_singel_qubit_operator(gate:&Gate) -> MatrixOperator
{
    let gate_name = gate.name.as_ref();
    match gate_name {
        "pauli-x" => gates::pauli_x(),
        "pauli-y" => gates::pauli_y(),
        "pauli-z" => gates::pauli_z(),
        "hadamard" => gates::hadamard(),
        "t" => gates::t(),
        "t-dagger" => gates::t_dagger(),
        "s" => gates::s(),
        "s-dagger" => gates::s_dagger(),
        "sqrt-not" => gates::sqrt_not(),
        "u" => {
            let phi = match gate.phi{
                Some(phi_value) => phi_value,
                None => panic!("u for qubit {} has no value for phi", gate.target)
            };
            let theta = match gate.theta{
                Some(theta_value) => theta_value,
                None => panic!("u for qubit {} has no value for theta", gate.target)
            };
            let lambda = match gate.lambda{
                Some(lambda_value) => lambda_value,
                None => panic!("u for qubit {} has no value for lambda", gate.target)
            };
            gates::u3_gate(theta, phi, lambda)
        },
        "u-phi-theta" => {
            let phi = match gate.phi{
                Some(phi_value) => phi_value,
                None => panic!("u-phi-theta for qubit {} has no value for phi", gate.target)
            };
            let theta = match gate.theta{
                Some(theta_value) => theta_value,
                None => panic!("u-phi-theta for qubit {} has no value for theta", gate.target)
            };
            gates::u_phi_theta(phi, theta)
        },
        "r-phi" => {
            let phi = match gate.phi{
                Some(phi_value) => phi_value,
                None => panic!("r-phi for qubit {} has no value for phi", gate.target)
            };
            gates::r_phi(phi)
        },
        "rx-phi" => {
            let phi = match gate.phi{
                Some(phi_value) => phi_value,
                None => panic!("rx-phi for qubit {} has no value for phi", gate.target)
            };
            gates::rx_phi(phi)
        },
        "ry-phi" => {
            let phi = match gate.phi{
                Some(phi_value) => phi_value,
                None => panic!("ry-phi for qubit {} has no value for phi", gate.target)
            };
            gates::ry_phi(phi)
        },
        "rz-phi" => {
            let phi = match gate.phi{
                Some(phi_value) => phi_value,
                None => panic!("rz-phi for qubit {} has no value for phi", gate.target)
            };
            gates::rz_phi(phi)
        }
        nunknown_gate => panic!("Unknown operator {}", nunknown_gate)
    }
}

fn get_operator(gate:&Gate) ->  Box<dyn Operator>
{
    let gate_name = gate.name.as_ref();
    if gate_name == "ctrl-pauli-x" {
        let control = match gate.control{
            Some(control_value) => control_value,
            None => panic!("ctrl-pauli-x for qubit {} has no value for control", gate.target)
        };
        let gate = gates::cx(((gate.target as i8-control as i8).abs()+1) as u8, control>gate.target);
        return Box::new(gate);
    }

    let gate = match gate_name {
        "measure-z" => gates::identity(1),
        "pauli-x" => gates::pauli_x(),
        "pauli-y" => gates::pauli_y(),
        "pauli-z" => gates::pauli_z(),
        "hadamard" => gates::hadamard(),
        "t" => gates::t(),
        "t-dagger" => gates::t_dagger(),
        "s" => gates::s(),
        "s-dagger" => gates::s_dagger(),
        "sqrt-not" => gates::sqrt_not(),
        "u" => {
            let phi = match gate.phi{
                Some(phi_value) => phi_value,
                None => panic!("u for qubit {} has no value for phi", gate.target)
            };
            let theta = match gate.theta{
                Some(theta_value) => theta_value,
                None => panic!("u for qubit {} has no value for theta", gate.target)
            };
            let lambda = match gate.lambda{
                Some(lambda_value) => lambda_value,
                None => panic!("u for qubit {} has no value for lambda", gate.target)
            };
            gates::u3_gate(theta, phi, lambda)
        },
        "u-phi-theta" => {
            let phi = match gate.phi{
                Some(phi_value) => phi_value,
                None => panic!("u-phi-theta for qubit {} has no value for phi", gate.target)
            };
            let theta = match gate.theta{
                Some(theta_value) => theta_value,
                None => panic!("u-phi-theta for qubit {} has no value for theta", gate.target)
            };
            gates::u_phi_theta(phi, theta)
        },
        "r-phi" => {
            let phi = match gate.phi{
                Some(phi_value) => phi_value,
                None => panic!("r-phi for qubit {} has no value for phi", gate.target)
            };
            gates::r_phi(phi)
        },
        "rx-phi" => {
            let phi = match gate.phi{
                Some(phi_value) => phi_value,
                None => panic!("rx-phi for qubit {} has no value for phi", gate.target)
            };
            gates::rx_phi(phi)
        },
        "ry-phi" => {
            let phi = match gate.phi{
                Some(phi_value) => phi_value,
                None => panic!("ry-phi for qubit {} has no value for phi", gate.target)
            };
            gates::ry_phi(phi)
        },
        "rz-phi" => {
            let phi = match gate.phi{
                Some(phi_value) => phi_value,
                None => panic!("rz-phi for qubit {} has no value for phi", gate.target)
            };
            gates::rz_phi(phi)
        },
        "swap" => {
            let target2 = match gate.target2{
                Some(target2_value) => target2_value,
                None => panic!("swap for qubit {} has no value for target2", gate.target)
            };
            gates::swap(((gate.target as i8-target2 as i8).abs()+1) as u8)
        },
        nunknown_gate => panic!("Unknown operator {}", nunknown_gate)
    };
    Box::new(gate)
}

fn get_qubit_count_from_circuit(circuit:&Circuit) -> u8 {
    let mut qubit_count = 0;

    for step in &circuit.steps {
        for gate in &step.gates {
            if gate.target+1 > qubit_count {
                qubit_count = gate.target+1;
            }
        }
    }

    qubit_count
}