use num_complex::Complex32;
use super::circuit::Gate;
use super::gates;

pub fn get_double_target_operator(gate:&Gate) -> [Complex32; 16] {
    let mut gate_name:&str =  gate.name.as_ref();

   
    if  gate_name.len() > 5 && &gate_name[..5] == "ctrl-" {
        gate_name =  &gate_name[5..]
    }

    match gate_name {
        "fredkin" => gates::swap(),
        "swap" => gates::swap(),
        "iswap" => gates::iswap(),
        "sqrt-swap" => gates::sqrt_swap(),
        "swap-theta" => {
            let theta = match gate.theta{
                Some(theta_value) => theta_value,
                None => panic!("swap-theta for qubits {:?} has no value for theta", gate.targets)
            };
            gates::swap_with_add_phase(theta)
        },
        "xx" => {
            let theta = match gate.theta{
                Some(theta_value) => theta_value,
                None => panic!("xx for qubit {:?} has no value for theta", gate.targets)
            };
            gates::xx(theta)
        }
        "yy" => {
            let theta = match gate.theta{
                Some(theta_value) => theta_value,
                None => panic!("yy for qubit {:?} has no value for theta", gate.targets)
            };
            gates::yy(theta)
        }
        "zz" => {
            let theta = match gate.theta{
                Some(theta_value) => theta_value,
                None => panic!("zz for qubit {:?} has no value for theta", gate.targets)
            };
            gates::zz(theta)
        }
        unknown_gate => panic!("Unknown multi-taget operator {}", unknown_gate)
    }
}

pub fn get_single_qubit_operator(gate:&Gate) -> [Complex32; 4] {
    let mut gate_name:&str =  gate.name.as_ref();

    if gate_name.len() > 5 && &gate_name[..5] == "ctrl-" {
        gate_name =  &gate_name[5..]
    }

    match gate_name {
        "identity" => gates::identity(),
        "toffoli" => gates::pauli_x(),
        "pauli-x" => gates::pauli_x(),
        "pauli-y" => gates::pauli_y(),
        "pauli-z" => gates::pauli_z(),
        "hadamard" => gates::hadamard(),
        "t" => gates::t(),
        "t-dagger" => gates::t_dagger(),
        "s" => gates::s(),
        "s-dagger" => gates::s_dagger(),
        "sqrt-not" => gates::sqrt_not(),
        "u3" => {
            let phi = match gate.phi{
                Some(phi_value) => phi_value,
                None => panic!("u3 for qubit {} has no value for phi", gate.targets[0])
            };
            let theta = match gate.theta{
                Some(theta_value) => theta_value,
                None => panic!("u3 for qubit {} has no value for theta", gate.targets[0])
            };
            let lambda = match gate.lambda{
                Some(lambda_value) => lambda_value,
                None => panic!("u3 for qubit {} has no value for lambda", gate.targets[0])
            };
            gates::u3(theta, phi, lambda)
        },
        "u2" => {
            let phi = match gate.phi{
                Some(phi_value) => phi_value,
                None => panic!("u-phi-theta for qubit {} has no value for phi", gate.targets[0])
            };
            let lambda = match gate.lambda{
                Some(lambda_value) => lambda_value,
                None => panic!("u2 for qubit {} has no value for lambda", gate.targets[0])
            };
            gates::u2(phi, lambda)
        },
        "u1" => {
            let lambda = match gate.lambda{
                Some(lambda_value) => lambda_value,
                None => panic!("u1 for qubit {} has no value for lambda", gate.targets[0])
            };
            gates::u1(lambda)
        },
        "rx-theta" => {
            let theta = match gate.theta{
                Some(theta_value) => theta_value,
                None => panic!("rx-theta for qubit {} has no value for theta", gate.targets[0])
            };
            gates::rx_theta(theta)
        },
        "pauli-x-root" => {
            let root = match &gate.root{
                Some(root_value) => get_value_from_root(root_value),
                None => panic!("pauli-x-root for qubit {} has no value for root", gate.targets[0])
            };
            gates::pauli_x_root(root)
        },
        "pauli-x-root-dagger" => {
            let root = match &gate.root{
                Some(root_value) => get_value_from_root(root_value),
                None => panic!("pauli-x-root for qubit {} has no value for root", gate.targets[0])
            };
            gates::pauli_x_root_dagger(root)
        },
        "ry-theta" => {
            let theta = match gate.theta{
                Some(theta_value) => theta_value,
                None => panic!("ry-theta for qubit {} has no value for theta", gate.targets[0])
            };
            gates::ry_theta(theta)
        },
        "pauli-y-root" => {
            let root = match &gate.root{
                Some(root_value) => get_value_from_root(root_value),
                None => panic!("pauli-y-root for qubit {} has no value for root", gate.targets[0])
            };
            gates::pauli_y_root(root)
        },
        "pauli-y-root-dagger" => {
            let root = match &gate.root{
                Some(root_value) => get_value_from_root(root_value),
                None => panic!("pauli-y-root for qubit {} has no value for root", gate.targets[0])
            };
            gates::pauli_y_root_dagger(root)
        },
        "rz-theta" => {
            let theta = match gate.theta{
                Some(theta_value) => theta_value,
                None => panic!("rz-theta for qubit {} has no value for theta", gate.targets[0])
            };
            gates::rz_theta(theta)
        },
        "pauli-z-root" => {
            let root = match &gate.root{
                Some(root_value) => get_value_from_root(root_value),
                None => panic!("pauli-z-root for qubit {} has no value for root", gate.targets[0])
            };
            gates::pauli_z_root(root)
        },
        "pauli-z-root-dagger" => {
            let root = match &gate.root{
                Some(root_value) => get_value_from_root(root_value),
                None => panic!("pauli-z-root for qubit {} has no value for root", gate.targets[0])
            };
            gates::pauli_z_root_dagger(root)
        },
        "measure-x" => {
            gates::hadamard()
        }
        "measure-y" => {
            gates::hadamard_times_s_dagger()
        }
        unknown_gate => panic!("Unknown operator {}", unknown_gate)
    }
}

fn get_value_from_root(root_value: &String) -> f32 {

    let t_str = if root_value.starts_with("1/") { root_value[2..].to_string() } else { root_value[..].to_string() };

    if t_str.starts_with("2^") {
        let t = t_str[2..].parse::<f32>().unwrap();
        2f32.powf(t)
    } else {
        let t = t_str.parse::<f32>().unwrap();
        t
    }
}