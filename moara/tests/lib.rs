#[cfg(test)]
mod test_tools;

extern crate moara;
extern crate num_complex;

use num_complex::Complex32;
use moara::statevector::Statevector;

#[test]
fn measure_state0_returns0() {
    let statevector = Statevector
    {
        data:vec![Complex32::new(1.0,0.0), Complex32::new(0.0,0.0)]
    };
    assert_eq!(0, moara::measure(&statevector));
}

#[test]
fn measure_state1_returns1() {
    let statevector = Statevector
    {
        data:vec![Complex32::new(0.0,0.0), Complex32::new(1.0,0.0)]
    };
    assert_eq!(1, moara::measure(&statevector));
}

#[test]
fn measure_state_plus_returns_equal_probabilities() {
    let amplitude0 = Complex32::new(1.0/2.0_f32.sqrt(),0.0);
    let amplitude1 = Complex32::new(1.0/2.0_f32.sqrt(),0.0);
    let statevector = Statevector
    {
        data:vec![amplitude0, amplitude1]
    };

    let sums = (0..1000).map(|_| moara::measure(&statevector))
                        .fold((0,0), |sum,x| if x == 0 {(sum.0+1,sum.1)} else {(sum.0,sum.1+1)});
    assert!(test_tools::aprox_equals(sums.0, sums.1, 0.1));
}

#[test]
fn measure_state_minus_returns_equal_probabilities() {
    let amplitude0 = Complex32::new(1.0/2.0_f32.sqrt(),0.0);
    let amplitude1 = Complex32::new(-1.0/2.0_f32.sqrt(),0.0);
    let statevector = Statevector
    {
        data:vec![amplitude0, amplitude1]
    };

    let sums = (0..1000).map(|_| moara::measure(&statevector))
                        .fold((0,0), |sum,x| if x == 0 {(sum.0+1,sum.1)} else {(sum.0,sum.1+1)});
    assert!(test_tools::aprox_equals(sums.0, sums.1, 0.1));
}

#[test]
fn measure_state_right_circ_returns_equal_probabilities() {
    let amplitude0 = Complex32::new(1.0/2.0_f32.sqrt(),0.0);
    let amplitude1 = Complex32::new(0.0,1.0/2.0_f32.sqrt());
    let statevector = Statevector
    {
        data:vec![amplitude0, amplitude1]
    };

    let sums = (0..1000).map(|_| moara::measure(&statevector))
                        .fold((0,0), |sum,x| if x == 0 {(sum.0+1,sum.1)} else {(sum.0,sum.1+1)});
    assert!(test_tools::aprox_equals(sums.0, sums.1, 0.1));
}

#[test]
fn measure_state_left_circ_returns_equal_probabilities() {
    let amplitude0 = Complex32::new(1.0/2.0_f32.sqrt(),0.0);
    let amplitude1 = Complex32::new(0.0,-1.0/2.0_f32.sqrt());
    let statevector = Statevector
    {
        data:vec![amplitude0, amplitude1]
    };

    let sums = (0..1000).map(|_| moara::measure(&statevector))
                        .fold((0,0), |sum,x| if x == 0 {(sum.0+1,sum.1)} else {(sum.0,sum.1+1)});
    assert!(test_tools::aprox_equals(sums.0, sums.1, 0.1));
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

    let sums = (0..1000).map(|_| moara::measure(&statevector))
                        .fold((0,0), |sum,x| if x == 0 {(sum.0+1,sum.1)} else {(sum.0,sum.1+1)});
    assert!(test_tools::check_distribution([sums.0, sums.1], [one_third, two_thirds], 0.1));
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

    let sums = (0..1000).map(|_| moara::measure(&statevector))
                        .fold((0,0), |sum,x| if x == 0 {(sum.0+1,sum.1)} else {(sum.0,sum.1+1)});
    assert!(test_tools::check_distribution([sums.0, sums.1], [one_third, two_thirds], 0.1));
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

    let sums = (0..1000).map(|_| moara::measure(&statevector))
                        .fold((0,0), |sum,x| if x == 0 {(sum.0+1,sum.1)} else {(sum.0,sum.1+1)});
    assert!(test_tools::check_distribution([sums.0, sums.1], [0.3_f32, 0.7_f32], 0.1));
}