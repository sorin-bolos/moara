use num_complex::Complex32;
use super::circuit::Gate;
use super::gates;

pub fn get_double_target_operator(gate:&Gate) -> [Complex32; 16] {
    let gate_name:&str =  gate.name.as_ref();

    match gate_name {
        "swap" => gates::swap(),
        "iswap" => gates::iswap(),
        "fswap" => gates::fswap(),
        "sqrt-swap" => gates::sqrt_swap(),
        "sqrt-swap-dagger" => gates::sqrt_swap_dagger(),
        "berkeley" => gates::berkeley(),
        "berkeley-dagger" => gates::berkeley_dagger(),
        "ecp" => gates::ecp(),
        "ecp-dagger" => gates::ecp_dagger(),
        "magic" => gates::magic(),
        "magic-dagger" => gates::magic_dagger(),
        "molmer-sorensen" => gates::molmer_sorensen(),
        "molmer-sorensen-dagger" => gates::molmer_sorensen_dagger(),
        "w" => gates::w(),
        "a" => {
          let theta = match gate.theta{
              Some(theta_value) => theta_value,
              None => panic!("xy for qubit {:?} has no value for theta", gate.targets)
          };
          let phi = match gate.phi{
            Some(phi_value) => phi_value,
            None => panic!("a for qubit {:?} has no value for phi", gate.targets)
          };
          gates::a(theta, phi)
        }
        "cross-resonance" => {
          let theta = match gate.theta{
              Some(theta_value) => theta_value,
              None => panic!("cross-resonance for qubits {:?} has no value for theta", gate.targets)
          };
          gates::cross_resonance(theta)
        },
        "cross-resonance-dagger" => {
          let theta = match gate.theta{
              Some(theta_value) => theta_value,
              None => panic!("cross-resonance-dagger for qubits {:?} has no value for theta", gate.targets)
          };
          gates::cross_resonance_dagger(theta)
        },
        "givens" => {
          let theta = match gate.theta{
              Some(theta_value) => theta_value,
              None => panic!("givens for qubits {:?} has no value for theta", gate.targets)
          };
          gates::givens(theta)
        },
        "swap-theta" => {
            let theta = match gate.theta{
                Some(theta_value) => theta_value,
                None => panic!("swap-theta for qubits {:?} has no value for theta", gate.targets)
            };
            gates::swap_with_add_phase(theta)
        },
        "swap-root" => {
            let root = match &gate.root{
                Some(root_value) => get_value_from_root(root_value),
                None => panic!("swap-root for qubit {:?} has no value for root", gate.targets)
            };
            gates::swap_root(root)
        },
        "swap-root-dagger" => {
          let root = match &gate.root{
              Some(root_value) => get_value_from_root(root_value),
              None => panic!("swap-root-dagger for qubit {:?} has no value for root", gate.targets)
          };
          gates::swap_root_dagger(root)
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
        "xy" => {
          let theta = match gate.theta{
              Some(theta_value) => theta_value,
              None => panic!("xy for qubit {:?} has no value for theta", gate.targets)
          };
          gates::xy(theta)
        }
        unknown_gate => panic!("Unknown multi-taget operator {}", unknown_gate)
    }
}

pub fn get_single_qubit_operator(gate:&Gate) -> [Complex32; 4] {
    let gate_name:&str =  gate.name.as_ref();

    match gate_name {
        "identity" => gates::identity(),
        "pauli-x" => gates::pauli_x(),
        "pauli-y" => gates::pauli_y(),
        "pauli-z" => gates::pauli_z(),
        "c" => gates::c(),
        "c-dagger" => gates::c_dagger(),
        "hadamard" => gates::hadamard(),
        "hadamard-xy" => gates::hadamard_xy(),
        "hadamard-yz" => gates::hadamard_yz(),
        "hadamard-zx" => gates::hadamard_zx(),
        "t" => gates::t(),
        "t-dagger" => gates::t_dagger(),
        "s" => gates::s(),
        "s-dagger" => gates::s_dagger(),
        "v" => gates::v(),
        "v-dagger" => gates::v_dagger(),
        "h" => gates::h(),
        "h-dagger" => gates::h_dagger(),
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