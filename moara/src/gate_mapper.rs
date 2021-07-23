use num_complex::Complex32;
use super::circuit::Gate;
use super::gates;

pub fn get_double_target_operator(gate:Gate) -> [Complex32; 16] {
    let gate_name = gate.name.as_ref();
    match gate_name {
        "swap" => gates::swap(),
        "iswap" => gates::iswap(),
        "sqrt-swap" => gates::sqrt_swap(),
        "swap-phi" => {
            let phi = match gate.phi{
                Some(phi_value) => phi_value,
                None => panic!("swap-phi for qubit {} has no value for phi", gate.target)
            };
            gates::swap_with_add_phase(phi)
        },
        "xx" => {
            let theta = match gate.theta{
                Some(theta_value) => theta_value,
                None => panic!("xx for qubit {} has no value for theta", gate.target)
            };
            gates::xx(theta)
        }
        "yy" => {
            let theta = match gate.theta{
                Some(theta_value) => theta_value,
                None => panic!("yy for qubit {} has no value for theta", gate.target)
            };
            gates::yy(theta)
        }
        "zz" => {
            let theta = match gate.theta{
                Some(theta_value) => theta_value,
                None => panic!("zz for qubit {} has no value for theta", gate.target)
            };
            gates::zz(theta)
        }
        unknown_gate => panic!("Unknown multi-taget operator {}", unknown_gate)
    }
}

pub fn get_single_qubit_operator(gate:Gate) -> [Complex32; 4] {
    let gate_name = gate.name.as_ref();
    match gate_name {
        "identity" => gates::identity(),
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
                None => panic!("u3 for qubit {} has no value for phi", gate.target)
            };
            let theta = match gate.theta{
                Some(theta_value) => theta_value,
                None => panic!("u3 for qubit {} has no value for theta", gate.target)
            };
            let lambda = match gate.lambda{
                Some(lambda_value) => lambda_value,
                None => panic!("u3 for qubit {} has no value for lambda", gate.target)
            };
            gates::u3(theta, phi, lambda)
        },
        "u2" => {
            let phi = match gate.phi{
                Some(phi_value) => phi_value,
                None => panic!("u-phi-theta for qubit {} has no value for phi", gate.target)
            };
            let lambda = match gate.lambda{
                Some(lambda_value) => lambda_value,
                None => panic!("u2 for qubit {} has no value for lambda", gate.target)
            };
            gates::u2(phi, lambda)
        },
        "u1" => {
            let lambda = match gate.lambda{
                Some(lambda_value) => lambda_value,
                None => panic!("u1 for qubit {} has no value for lambda", gate.target)
            };
            gates::u1(lambda)
        },
        "rx-theta" => {
            let theta = match gate.theta{
                Some(theta_value) => theta_value,
                None => panic!("rx-theta for qubit {} has no value for theta", gate.target)
            };
            gates::rx_theta(theta)
        },
        "pauli-x-root" => {
            let root = match &gate.root{
                Some(root_value) => get_value_from_root(root_value),
                None => panic!("pauli-x-root for qubit {} has no value for root", gate.target)
            };
            gates::pauli_x_root(root)
        },
        "pauli-x-root-dagger" => {
            let root = match &gate.root{
                Some(root_value) => get_value_from_root(root_value),
                None => panic!("pauli-x-root for qubit {} has no value for root", gate.target)
            };
            gates::pauli_x_root_dagger(root)
        },
        "ry-theta" => {
            let theta = match gate.theta{
                Some(theta_value) => theta_value,
                None => panic!("ry-theta for qubit {} has no value for theta", gate.target)
            };
            gates::ry_theta(theta)
        },
        "pauli-y-root" => {
            let root = match &gate.root{
                Some(root_value) => get_value_from_root(root_value),
                None => panic!("pauli-y-root for qubit {} has no value for root", gate.target)
            };
            gates::pauli_y_root(root)
        },
        "pauli-y-root-dagger" => {
            let root = match &gate.root{
                Some(root_value) => get_value_from_root(root_value),
                None => panic!("pauli-y-root for qubit {} has no value for root", gate.target)
            };
            gates::pauli_y_root_dagger(root)
        },
        "rz-theta" => {
            let theta = match gate.theta{
                Some(theta_value) => theta_value,
                None => panic!("rz-theta for qubit {} has no value for theta", gate.target)
            };
            gates::rz_theta(theta)
        },
        "pauli-z-root" => {
            let root = match &gate.root{
                Some(root_value) => get_value_from_root(root_value),
                None => panic!("pauli-z-root for qubit {} has no value for root", gate.target)
            };
            gates::pauli_z_root(root)
        },
        "pauli-z-root-dagger" => {
            let root = match &gate.root{
                Some(root_value) => get_value_from_root(root_value),
                None => panic!("pauli-z-root for qubit {} has no value for root", gate.target)
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

pub fn get_operator_for_controlled(gate:Gate) ->  [Complex32; 4] {
    //remove the prefix ex: "ctrl-pauli-x" -> "pauli-x"
    let single_qubit_gate_name = &gate.name[5..];
    let single_qubit_gate = Gate {
        name:single_qubit_gate_name.to_string(),
        target:gate.target,
        target2:gate.target2,
        control:gate.control,
        controlstate:gate.controlstate,
        control2:gate.control2,
        controlstate2:gate.controlstate2,
        bit:gate.bit,
        phi:gate.phi,
        theta:gate.theta,
        lambda:gate.lambda,
        root:gate.root,
    };

    get_single_qubit_operator(single_qubit_gate)
}

pub fn get_operator_for_double_target_controlled(gate:Gate) ->  [Complex32; 16] {
        
    match &gate.name[..] {
        "fredkin" => { return gates::swap() },
        _ => { }
    }

    //remove the prefix ex: "ctrl-pauli-x" -> "pauli-x"
    let single_qubit_gate_name = &gate.name[5..];
    let single_qubit_gate = Gate {
        name:single_qubit_gate_name.to_string(),
        target:gate.target,
        target2:gate.target2,
        control:gate.control,
        controlstate:gate.controlstate,
        control2:gate.control2,
        controlstate2:gate.controlstate2,
        bit:gate.bit,
        phi:gate.phi,
        theta:gate.theta,
        lambda:gate.lambda,
        root:gate.root,
    };

    get_double_target_operator(single_qubit_gate)
}

pub fn get_toffoli_operator(control2:u8, target:u8, control_on_zero2:bool) -> [Complex32; 16] {
    if control_on_zero2 {
        if control2 > target {
            gates::c0x1_0()
        } else { 
            gates::c1x0_0()
        }
    } else {
        if control2 > target {
            gates::c0x1_1()
        } else { 
            gates::c1x0_1()
        }
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