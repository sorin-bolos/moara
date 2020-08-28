#[cfg(test)]
extern crate moara;
extern crate num_complex;

use num_complex::Complex32;
use moara::statevector::Statevector;
use moara::measurement;

#[test]
fn measure_state0_returns0() {
    let statevector = Statevector
    {
        data:vec![Complex32::new(1.0,0.0), Complex32::new(0.0,0.0)]
    };
    assert_eq!(0, measurement::measure(&statevector));
}

#[test]
fn measure_state1_returns1() {
    let statevector = Statevector
    {
        data:vec![Complex32::new(0.0,0.0), Complex32::new(1.0,0.0)]
    };
    assert_eq!(1, measurement::measure(&statevector));
}

#[test]
fn measure_state_plus_returns_equal_probabilities() {
    let amplitude0 = Complex32::new(1.0/2.0_f32.sqrt(),0.0);
    let amplitude1 = Complex32::new(1.0/2.0_f32.sqrt(),0.0);
    let statevector = Statevector
    {
        data:vec![amplitude0, amplitude1]
    };

    let sums = (0..1000).map(|_| measurement::measure(&statevector))
                        .fold((0,0), |sum,x| if x == 0 {(sum.0+1,sum.1)} else {(sum.0,sum.1+1)});
    assert!(aprox_equals(sums.0, sums.1, 0.1));
}

#[test]
fn measure_state_minus_returns_equal_probabilities() {
    let amplitude0 = Complex32::new(1.0/2.0_f32.sqrt(),0.0);
    let amplitude1 = Complex32::new(-1.0/2.0_f32.sqrt(),0.0);
    let statevector = Statevector
    {
        data:vec![amplitude0, amplitude1]
    };

    let sums = (0..1000).map(|_| measurement::measure(&statevector))
                        .fold((0,0), |sum,x| if x == 0 {(sum.0+1,sum.1)} else {(sum.0,sum.1+1)});
    assert!(aprox_equals(sums.0, sums.1, 0.1));
}

#[test]
fn measure_state_right_circ_returns_equal_probabilities() {
    let amplitude0 = Complex32::new(1.0/2.0_f32.sqrt(),0.0);
    let amplitude1 = Complex32::new(0.0,1.0/2.0_f32.sqrt());
    let statevector = Statevector
    {
        data:vec![amplitude0, amplitude1]
    };

    let sums = (0..1000).map(|_| measurement::measure(&statevector))
                        .fold((0,0), |sum,x| if x == 0 {(sum.0+1,sum.1)} else {(sum.0,sum.1+1)});
    assert!(aprox_equals(sums.0, sums.1, 0.1));
}

#[test]
fn measure_state_left_circ_returns_equal_probabilities() {
    let amplitude0 = Complex32::new(1.0/2.0_f32.sqrt(),0.0);
    let amplitude1 = Complex32::new(0.0,-1.0/2.0_f32.sqrt());
    let statevector = Statevector
    {
        data:vec![amplitude0, amplitude1]
    };

    let sums = (0..1000).map(|_| measurement::measure(&statevector))
                        .fold((0,0), |sum,x| if x == 0 {(sum.0+1,sum.1)} else {(sum.0,sum.1+1)});
    assert!(aprox_equals(sums.0, sums.1, 0.1));
}

#[test]
fn measure_given_state_returns_correct_probabilities() {
    let one_third = 1.0/3.0 as f32;
    let two_thirds = 2.0/3.0 as f32;
    let amplitude0 = Complex32::new(one_third.sqrt(),0.0);
    let amplitude1 = Complex32::new(two_thirds.sqrt(),0.0);
    let statevector = Statevector
    {
        data:vec![amplitude0, amplitude1]
    };

    let sums = (0..1000).map(|_| measurement::measure(&statevector))
                        .fold((0,0), |sum,x| if x == 0 {(sum.0+1,sum.1)} else {(sum.0,sum.1+1)});
    assert!(check_distribution([sums.0, sums.1], [one_third, two_thirds], 0.1));
}

#[test]
fn measure_given_imaginary_state_returns_correct_probabilities() {
    let one_third = 1.0/3.0 as f32;
    let two_thirds = 2.0/3.0 as f32;
    let amplitude0 = Complex32::new(0.0,one_third.sqrt());
    let amplitude1 = Complex32::new(0.0,two_thirds.sqrt());
    let statevector = Statevector
    {
        data:vec![amplitude0, amplitude1]
    };

    let sums = (0..1000).map(|_| measurement::measure(&statevector))
                        .fold((0,0), |sum,x| if x == 0 {(sum.0+1,sum.1)} else {(sum.0,sum.1+1)});
    assert!(check_distribution([sums.0, sums.1], [one_third, two_thirds], 0.1));
}

#[test]
fn measure_given_random_state_returns_correct_probabilities() {
    let one_tenth = 0.1 as f32;
    let two_tenths = 0.2 as f32;
    let three_tenths = 0.3 as f32;
    let four_tenths = 0.4 as f32;
    let amplitude0 = Complex32::new(one_tenth.sqrt(), two_tenths.sqrt());
    let amplitude1 = Complex32::new(three_tenths.sqrt(), four_tenths.sqrt());
    let statevector = Statevector
    {
        data:vec![amplitude0, amplitude1]
    };

    let sums = (0..1000).map(|_| measurement::measure(&statevector))
                        .fold((0,0), |sum,x| if x == 0 {(sum.0+1,sum.1)} else {(sum.0,sum.1+1)});
    assert!(check_distribution([sums.0, sums.1], [0.3_f32, 0.7_f32], 0.05));
}

pub fn aprox_equals(a:i32, b:i32, fraction:f32) -> bool
{
    let diff = (b-a).abs();
    let sum = (b+a).abs();

    (diff as f32) <= (sum as f32) * fraction
}

pub fn check_distribution(counts:[i32;2], distribution:[f32;2], tolerance:f32) -> bool
{
    let sum = (counts[0]+counts[1]).abs();
    let prob0 = counts[0] as f32/sum as f32;
    let prob1 = counts[1] as f32/sum as f32;

    (distribution[0]-prob0).abs() < tolerance 
      && (distribution[1]-prob1).abs() < tolerance
}