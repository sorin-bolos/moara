use std::mem;
use num_complex::Complex32;
use super::circuit::Gate;
use super::gates;

pub fn get_double_target_operator(gate:&Gate, conjugate:bool) -> [Complex32; 16] {
    let gate_name:&str =  gate.name.as_ref();

    match gate_name {
        "swap" => gates::swap(),
        "iswap" => {
          if !conjugate {
            gates::iswap()
          } else {
            gates::iswap_dagger()
          }
        },
        "fswap" => gates::fswap(),
        "sqrt-swap" => {
          if !conjugate {
            gates::sqrt_swap()
          } else {
            gates::sqrt_swap_dagger()
          }
        },
        "sqrt-swap-dagger" => {
          if !conjugate {
            gates::sqrt_swap_dagger()
          } else {
            gates::sqrt_swap()
          }
        },
        "berkeley" => {
          if !conjugate {
            gates::berkeley()
          } else {
            gates::berkeley_dagger()
          }
        },
        "berkeley-dagger" => {
          if !conjugate {
            gates::berkeley_dagger()
          } else {
            gates::berkeley()
          }
        },
        "ecp" => {
          if !conjugate {
            gates::ecp()
          } else {
            gates::ecp_dagger()
          }
        },
        "ecp-dagger" => {
          if !conjugate {
            gates::ecp_dagger()
          } else {
            gates::ecp()
          }
        },
        "magic" => {
          if !conjugate {
            gates::magic()
          } else {
            gates::magic_dagger()
          }
        },
        "magic-dagger" => {
          if !conjugate {
            gates::magic_dagger()
          } else {
            gates::magic()
          }
        },
        "molmer-sorensen" => {
          if !conjugate {
            gates::molmer_sorensen()
          } else {
            gates::molmer_sorensen_dagger()
          }
        },
        "molmer-sorensen-dagger" => {
          if !conjugate {
            gates::molmer_sorensen_dagger()
          } else {
            gates::molmer_sorensen()
          }
        },
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
          if !conjugate {
            gates::cross_resonance(theta)
          } else {
            gates::cross_resonance_dagger(theta)
          }
        },
        "cross-resonance-dagger" => {
          let theta = match gate.theta{
              Some(theta_value) => theta_value,
              None => panic!("cross-resonance-dagger for qubits {:?} has no value for theta", gate.targets)
          };
          if !conjugate {
            gates::cross_resonance_dagger(theta)
          } else {
            gates::cross_resonance(theta)
          }
        },
        "givens" => {
          let theta = match gate.theta{
              Some(theta_value) => theta_value,
              None => panic!("givens for qubits {:?} has no value for theta", gate.targets)
          };
          gates::givens(theta)
        },
        "swap-theta" => {
            let mut theta = match gate.theta{
                Some(theta_value) => theta_value,
                None => panic!("swap-theta for qubits {:?} has no value for theta", gate.targets)
            };
            if conjugate {
              theta = - theta;
            }
            gates::swap_with_add_phase(theta)
        },
        "swap-root" => {
            let root = match &gate.root{
                Some(root_value) => get_value_from_root(root_value),
                None => panic!("swap-root for qubit {:?} has no value for root", gate.targets)
            };
            if conjugate {
              gates::swap_root(root)
            } else {
              gates::swap_root_dagger(root)
            }
        },
        "swap-root-dagger" => {
          let root = match &gate.root{
              Some(root_value) => get_value_from_root(root_value),
              None => panic!("swap-root-dagger for qubit {:?} has no value for root", gate.targets)
          };
          if conjugate {
            gates::swap_root_dagger(root)
          } else {
            gates::swap_root(root)
          }
        },
        "xx" => {
            let mut theta = match gate.theta{
                Some(theta_value) => theta_value,
                None => panic!("xx for qubit {:?} has no value for theta", gate.targets)
            };
            if conjugate {
              theta = - theta;
            }
            gates::xx(theta)
        }
        "yy" => {
            let mut theta = match gate.theta{
                Some(theta_value) => theta_value,
                None => panic!("yy for qubit {:?} has no value for theta", gate.targets)
            };
            if conjugate {
              theta = - theta;
            }
            gates::yy(theta)
        }
        "zz" => {
            let mut theta = match gate.theta{
                Some(theta_value) => theta_value,
                None => panic!("zz for qubit {:?} has no value for theta", gate.targets)
            };
            if conjugate {
              theta = - theta;
            }
            gates::zz(theta)
        }
        "xy" => {
          let mut theta = match gate.theta{
              Some(theta_value) => theta_value,
              None => panic!("xy for qubit {:?} has no value for theta", gate.targets)
          };
          if conjugate {
            theta = - theta;
          }
          gates::xy(theta)
        }
        unknown_gate => panic!("Unknown multi-taget operator {}", unknown_gate)
    }
}

pub fn get_single_qubit_operator(gate:&Gate, conjugate:bool) -> [Complex32; 4] {
    let gate_name:&str =  gate.name.as_ref();

    match gate_name {
        "identity" => gates::identity(),
        "pauli-x" => gates::pauli_x(),
        "pauli-y" => gates::pauli_y(),
        "pauli-z" => gates::pauli_z(),
        "c" => if !conjugate {
            gates::c()
          } else {
            gates::c_dagger()
          },
        "c-dagger" => if !conjugate {
            gates::c_dagger()
          } else {
            gates::c()
          },
        "hadamard" => gates::hadamard(),
        "hadamard-xy" => gates::hadamard_xy(),
        "hadamard-yz" => gates::hadamard_yz(),
        "hadamard-zx" => gates::hadamard_zx(),
        "t" => if !conjugate {
            gates::t()
          } else {
            gates::t_dagger()
          },
        "t-dagger" => if !conjugate {
            gates::t_dagger()
          } else {
            gates::t()
          },
        "s" => if !conjugate {
            gates::s()
          } else {
            gates::s_dagger()
          },
        "s-dagger" => if !conjugate {
            gates::s_dagger()
          } else {
            gates::s()
          },
        "v" => if !conjugate {
            gates::v()
          } else {
            gates::v_dagger()
          },
        "v-dagger" => if !conjugate {
            gates::v_dagger()
          } else {
            gates::v()
          },
        "h" => if !conjugate {
            gates::h()
          } else {
            gates::h_dagger()
          },
        "h-dagger" => if !conjugate {
            gates::h_dagger()
          } else {
            gates::h()
          },
        "u3" => {
            let mut phi = match gate.phi{
                Some(phi_value) => phi_value,
                None => panic!("u3 for qubit {} has no value for phi", gate.targets[0])
            };
            let theta = match gate.theta{
                Some(theta_value) => theta_value,
                None => panic!("u3 for qubit {} has no value for theta", gate.targets[0])
            };
            let mut lambda = match gate.lambda{
                Some(lambda_value) => lambda_value,
                None => panic!("u3 for qubit {} has no value for lambda", gate.targets[0])
            };
            if conjugate {
              mem::swap(&mut phi, &mut lambda);
            }
            gates::u3(theta, phi, lambda)
        },
        "u2" => {
            let mut phi = match gate.phi{
                Some(phi_value) => phi_value,
                None => panic!("u-phi-theta for qubit {} has no value for phi", gate.targets[0])
            };
            let mut lambda = match gate.lambda{
                Some(lambda_value) => lambda_value,
                None => panic!("u2 for qubit {} has no value for lambda", gate.targets[0])
            };
            if conjugate {
              mem::swap(&mut phi, &mut lambda);
            }
            gates::u2(phi, lambda)
        },
        "u1" => {
            let mut lambda = match gate.lambda{
                Some(lambda_value) => lambda_value,
                None => panic!("u1 for qubit {} has no value for lambda", gate.targets[0])
            };
            if conjugate {
              lambda = - lambda;
            }
            gates::u1(lambda)
        },
        "p" => {
          let mut theta = match gate.theta{
              Some(theta_value) => theta_value,
              None => panic!("p for qubit {} has no value for theta", gate.targets[0])
          };
          if conjugate {
            theta = - theta;
          }
          gates::p(theta)
        },
        "rx-theta" => {
            let mut theta = match gate.theta{
                Some(theta_value) => theta_value,
                None => panic!("rx-theta for qubit {} has no value for theta", gate.targets[0])
            };
            if conjugate {
              theta = - theta;
            }
            gates::rx_theta(theta)
        },
        "pauli-x-root" => {
            let root = match &gate.root{
                Some(root_value) => get_value_from_root(root_value),
                None => panic!("pauli-x-root for qubit {} has no value for root", gate.targets[0])
            };
            if !conjugate {
              gates::pauli_x_root(root)
            } else {
              gates::pauli_x_root_dagger(root)
            }
        },
        "pauli-x-root-dagger" => {
            let root = match &gate.root{
                Some(root_value) => get_value_from_root(root_value),
                None => panic!("pauli-x-root for qubit {} has no value for root", gate.targets[0])
            };
            if !conjugate {
              gates::pauli_x_root_dagger(root)
            } else {
              gates::pauli_x_root(root)
            }
        },
        "ry-theta" => {
            let mut theta = match gate.theta{
                Some(theta_value) => theta_value,
                None => panic!("ry-theta for qubit {} has no value for theta", gate.targets[0])
            };
            if conjugate {
              theta = - theta;
            }
            gates::ry_theta(theta)
        },
        "pauli-y-root" => {
            let root = match &gate.root{
                Some(root_value) => get_value_from_root(root_value),
                None => panic!("pauli-y-root for qubit {} has no value for root", gate.targets[0])
            };
            if !conjugate {
              gates::pauli_y_root(root)
            } else {
              gates::pauli_y_root_dagger(root)
            }
        },
        "pauli-y-root-dagger" => {
            let root = match &gate.root{
                Some(root_value) => get_value_from_root(root_value),
                None => panic!("pauli-y-root for qubit {} has no value for root", gate.targets[0])
            };
            if !conjugate {
              gates::pauli_y_root_dagger(root)
            } else {
              gates::pauli_y_root(root)
            }
        },
        "rz-theta" => {
            let mut theta = match gate.theta{
                Some(theta_value) => theta_value,
                None => panic!("rz-theta for qubit {} has no value for theta", gate.targets[0])
            };
            if conjugate {
              theta = - theta;
            }
            gates::rz_theta(theta)
        },
        "pauli-z-root" => {
            let root = match &gate.root{
                Some(root_value) => get_value_from_root(root_value),
                None => panic!("pauli-z-root for qubit {} has no value for root", gate.targets[0])
            };
            if !conjugate {
              gates::pauli_z_root(root)
            } else {
              gates::pauli_z_root_dagger(root)
            }
        },
        "pauli-z-root-dagger" => {
            let root = match &gate.root{
                Some(root_value) => get_value_from_root(root_value),
                None => panic!("pauli-z-root for qubit {} has no value for root", gate.targets[0])
            };
            if !conjugate {
              gates::pauli_z_root_dagger(root)
            } else {
              gates::pauli_z_root(root)
            }
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

pub fn get_qubit_rotation_operator(basis: &str) -> [Complex32; 4] {

  // X-basis: |+⟩ = 1/√2(|0⟩ + |1⟩), |-⟩ = 1/√2(|0⟩ - |1⟩)
  // Y-basis: |+i⟩ = 1/√2(|0⟩ + i|1⟩), |-i⟩ = 1/√2(|0⟩ - i|1⟩)

  match basis {
    "X" => gates::x_basis_to_standard_basis_rotation(),
    "Y" => gates::y_basis_to_standard_basis_rotation(),
    unknown_basis => panic!("Unknown basis {}", unknown_basis)
  }
}

pub fn get_qubit_undo_rotation_operator(basis: &str) -> [Complex32; 4] {

// X-basis: |+⟩ = 1/√2(|0⟩ + |1⟩), |-⟩ = 1/√2(|0⟩ - |1⟩)
// Y-basis: |+i⟩ = 1/√2(|0⟩ + i|1⟩), |-i⟩ = 1/√2(|0⟩ - i|1⟩)

  match basis {
    "X" => gates::standard_basis_to_x_basis_rotation(),
    "Y" => gates::standard_basis_to_y_basis_rotation(),
    unknown_basis => panic!("Unknown basis {}", unknown_basis)
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