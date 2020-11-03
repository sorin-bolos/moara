#[cfg(test)]
extern crate moara;
extern crate num_complex;

use num_complex::Complex32;
use moara::operator::Operator;
use moara::operator::MatrixOperator;
use moara::statevector::Statevector;

#[test]
fn create_with_unitary_matrix_works() {
    MatrixOperator::new(vec![
        vec![Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(1.0/2.0_f32.sqrt(),0.0)],
        vec![Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(-1.0/2.0_f32.sqrt(),0.0)]
    ]);
}

#[test]
#[should_panic(expected = "Cannot apply operator of size 4 to statevector of size 2")]
fn apply_to_smaller_length_statevector_panics() {
    let operator = MatrixOperator::new(vec![
        vec![Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,1.0)],
        vec![Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,-1.0), Complex32::new(0.0,0.0)],
        vec![Complex32::new(0.0,0.0), Complex32::new(0.0,1.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0)],
        vec![Complex32::new(0.0,-1.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0)]
    ]);

    let statevector = Statevector::new(vec![Complex32::new(1.0,0.0), Complex32::new(0.0,0.0)]);

    operator.apply(statevector);
}

#[test]
#[should_panic(expected = "Cannot apply operator of size 2 to statevector of size 4")]
fn apply_to_lerger_length_statevector_panics() {
    let operator = MatrixOperator::new(vec![
        vec![Complex32::new(0.0,0.0), Complex32::new(1.0,0.0)],
        vec![Complex32::new(1.0,0.0), Complex32::new(0.0,0.0)]
    ]);

    let statevector = Statevector::new(vec![Complex32::new(1.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0)]);

    operator.apply(statevector);
}

#[test]
fn apply_works() {
    let operator = MatrixOperator::new(vec![
        vec![Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(1.0,0.0)],
        vec![Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(1.0,0.0), Complex32::new(0.0,0.0)],
        vec![Complex32::new(0.0,0.0), Complex32::new(1.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0)],
        vec![Complex32::new(1.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0)]
    ]);

    let statevector = Statevector::new(vec![Complex32::new(1.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0)]);

    let expected = Statevector::new(vec![Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(1.0,0.0)]);

    let result = operator.apply(statevector);

    assert_eq!(expected.data(), result.data())
}