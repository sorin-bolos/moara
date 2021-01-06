#[cfg(test)]
use std::process::Command;

static PATH_TO_EXE: &str = "target/debug/moara.exe";
static FAILURE_MESSAGE: &str = "failed to execute process";

#[test]
fn entanglement_circuit_works() {
    let output = Command::new(PATH_TO_EXE)
                .args(&["tests/entanglement_2.json"])
                .output()
                .expect(FAILURE_MESSAGE);

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    
    assert!(output.status.success());
    let data = get_vector_from_string(stdout);

    assert_eq!(4, data.len());
    assert!(aprox_equals(500, data[0], 0.1));
    assert_eq!(0u32, data[1]);
    assert_eq!(0u32, data[2]);
    assert!(aprox_equals(500, data[3], 0.1));
}

#[test]
fn gate_space_two_qubit_space_gate_works() {
    let output = Command::new(PATH_TO_EXE)
                .args(&["tests/gate_space_two_qubit_space_gate.json"])
                .output()
                .expect(FAILURE_MESSAGE);

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    
    assert!(output.status.success());
    let data = get_vector_from_string(stdout);

    assert_eq!(64, data.len());
    for (i, count) in data.iter().enumerate() {
        if i == 33 || i == 41 {
            assert!(aprox_equals(500, *count, 0.1));
        } else {
            assert_eq!(0u32, *count);
        }
    }
}

#[test]
fn gate_space_two_qubit_works() {
    let output = Command::new(PATH_TO_EXE)
                .args(&["tests/gate_space_two_qubit.json"])
                .output()
                .expect(FAILURE_MESSAGE);

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    
    assert!(output.status.success());
    let data = get_vector_from_string(stdout);

    assert_eq!(16, data.len());
    for (i, count) in data.iter().enumerate() {
        if i == 8 || i == 11 {
            assert!(aprox_equals(500, *count, 0.1));
        } else {
            assert_eq!(0u32, *count);
        }
    }
}

fn get_vector_from_string(vector_as_string:String) -> Vec<u32> {
    let data: Vec<u32> = vector_as_string.trim_start_matches('[')
                                         .trim_end_matches(']')
                                         .split(", ")
                                         .map(|s| s.parse().unwrap())
                                         .collect();
    data
}

fn aprox_equals(a:u32, b:u32, fraction:f32) -> bool
{
    let diff = (b as i32 - a as i32).abs();
    let sum = b+a;

    (diff as f32) <= (sum as f32) * fraction
}

