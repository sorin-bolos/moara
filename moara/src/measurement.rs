use std::collections::HashMap;
use num_complex::Complex32;
use rand::Rng;
use rand::prelude::ThreadRng;

pub fn measure(statevector:Vec<Complex32>, shots:u32, measurements:HashMap<u8,u8>, qubit_count:u8) -> Vec<u32> {
    let mut rng = rand::thread_rng();

    let len = statevector.len();
    let bit_count = get_bit_count_from_measurements(&measurements);
    let results_len = if measurements.is_empty() { len } else { 1 << bit_count };
    let mut measurement_results = vec![0u32; results_len];
    
    let i8_count = qubit_count as i8;
    let i8_bit_count = bit_count as i8;
    let i8_measurements = convert_measurements_to_i8(measurements);
    let mut i = 0;
    while i < shots {
        let sample = sample(&statevector, &mut rng, len);
        let bit_position = get_bit_position_from_measurements(&i8_measurements, i8_count, i8_bit_count, sample);
        measurement_results[bit_position] += 1;
        i += 1;
    }

    measurement_results
}

pub fn get_probabilities(statevector:Vec<Complex32>, measurements:HashMap<u8,u8>, qubit_count:u8) -> Vec<f32> {
    let len = statevector.len();
    let bit_count = get_bit_count_from_measurements(&measurements);
    let probabilities_len = if measurements.is_empty() { len } else { 1 << bit_count };
    let mut probabilities = vec![0f32; probabilities_len];

    let i8_count = qubit_count as i8;
    let i8_bit_count = bit_count as i8;
    let i8_measurements = convert_measurements_to_i8(measurements);
    let mut i = 0;
    while i < len {
        let bit_position = get_bit_position_from_measurements(&i8_measurements, i8_count, i8_bit_count, i);
        probabilities[bit_position] += statevector[i].norm_sqr();
        i += 1;
    }

    probabilities
}

fn sample(statevector:&Vec<Complex32>, rng: &mut ThreadRng, len:usize) -> usize {
    let sample:f64 = rng.gen();
    let mut running_sum:f64 = 0.0;
    let mut i = 0;
    while i < len {
        let re = statevector[i].re as f64;
        let im = statevector[i].im as f64;
        running_sum += re*re + im*im;
        if sample < running_sum
        {
            return i;
        }
        i += 1;
    }

    panic!("Sample was not in the expected interval");
}

fn get_bit_count_from_measurements(measurements:&HashMap<u8,u8>) -> u8 {
    let mut max = 0;
    for (_, bit) in measurements {
        if bit > &max {
            max = *bit;
        }
    }
    max+1
}

fn get_bit_position_from_measurements(measurements:&HashMap<i8,i8>, qubit_count:i8, bit_count:i8, index:usize) -> usize {
    if measurements.is_empty() {
        return index;
    }

    let bit_diff = qubit_count-bit_count;
    let mut bit_position = 0;
    for (qubit, bit) in measurements {
        let mut masked = index & (1 << (qubit_count-qubit-1));
        let diff = bit-qubit+bit_diff;
        if diff > 0 {
            masked = masked >> diff;
        }
        if diff < 0 {
            masked = masked << -diff;
        }
        bit_position = bit_position ^ masked;
    }
    bit_position
}

fn convert_measurements_to_i8(measurements:HashMap<u8,u8>) -> HashMap<i8,i8> {
    let mut i8_measurements = HashMap::new();
    for (qubit, bit) in measurements {
        i8_measurements.insert(qubit as i8, bit as i8);
    }
    i8_measurements
}