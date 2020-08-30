#[cfg(test)]
extern crate moara;
extern crate num_complex;

use num_complex::Complex32;
use moara::operator::Operator;
use moara::statevector::Statevector;
use moara::tools;

#[test]
fn create_with_unitary_matrix_works() {
    Operator::new(vec![
        vec![Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(1.0/2.0_f32.sqrt(),0.0)],
        vec![Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(-1.0/2.0_f32.sqrt(),0.0)]
    ]);
}

#[test]
#[should_panic(expected = "Operator is not unitary")]
fn create_with_non_unitary_matrix_panics() {
    Operator::new(vec![
        vec![Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(1.0/2.0_f32.sqrt(),0.0)],
        vec![Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(1.0/2.0_f32.sqrt(),0.0)]
    ]);
}

#[test]
#[should_panic(expected = "Matrix is not square")]
fn create_with_portrait_matrix_panics() {
    Operator::new(vec![
        vec![Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(1.0/2.0_f32.sqrt(),0.0)],
        vec![Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(1.0/2.0_f32.sqrt(),0.0)],
        vec![Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(1.0/2.0_f32.sqrt(),0.0)]
    ]);
}

#[test]
#[should_panic(expected = "Matrix is not square")]
fn create_with_landscape_matrix_panics() {
    Operator::new(vec![
        vec![Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(1.0/2.0_f32.sqrt(),0.0)],
        vec![Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(1.0/2.0_f32.sqrt(),0.0)]
    ]);
}

#[test]
#[should_panic(expected = "Cannot combine operator of size 2 with operator of size 3")]
fn dot_with_larger_matrix_panics() {
    let a = Operator::new(vec![
        vec![Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(1.0/2.0_f32.sqrt(),0.0)],
        vec![Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(-1.0/2.0_f32.sqrt(),0.0)]
    ]);

    let b = Operator::new(vec![
        vec![Complex32::new(-1.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0)],
        vec![Complex32::new(0.0,0.0), Complex32::new(1.0,0.0), Complex32::new(0.0,0.0)],
        vec![Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(1.0,0.0)]
    ]);

    a.dot(b);
}

#[test]
#[should_panic(expected = "Cannot combine operator of size 3 with operator of size 2")]
fn dot_with_smaller_matrix_panics() {
    let a = Operator::new(vec![
        vec![Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(1.0/2.0_f32.sqrt(),0.0)],
        vec![Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(-1.0/2.0_f32.sqrt(),0.0)]
    ]);

    let b = Operator::new(vec![
        vec![Complex32::new(-1.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0)],
        vec![Complex32::new(0.0,0.0), Complex32::new(1.0,0.0), Complex32::new(0.0,0.0)],
        vec![Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(1.0,0.0)]
    ]);

    b.dot(a);
}

#[test]
fn dot_works() {
    let a = Operator::new(vec![
        vec![Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(1.0/2.0_f32.sqrt(),0.0)],
        vec![Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(-1.0/2.0_f32.sqrt(),0.0)]
    ]);

    let b = Operator::new(vec![
        vec![Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(1.0/2.0_f32.sqrt(),0.0)],
        vec![Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(-1.0/2.0_f32.sqrt(),0.0)]
    ]);

    let i = Operator::new(vec![
        vec![Complex32::new(1.0,0.0), Complex32::new(0.0,0.0)],
        vec![Complex32::new(0.0,0.0), Complex32::new(1.0,0.0)]
    ]);

    let c = a.dot(b);

    assert!(operators_are_equal(i,c));
}

#[test]
fn tensor_same_size_works() {
    let a = Operator::new(vec![
        vec![Complex32::new(0.0,0.0), Complex32::new(1.0,0.0)],
        vec![Complex32::new(1.0,0.0), Complex32::new(0.0,0.0)]
    ]);

    let b = Operator::new(vec![
        vec![Complex32::new(0.0,0.0), Complex32::new(0.0,1.0)],
        vec![Complex32::new(0.0,-1.0), Complex32::new(0.0,0.0)]
    ]);

    let expected = Operator::new(vec![
        vec![Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,1.0)],
        vec![Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,-1.0), Complex32::new(0.0,0.0)],
        vec![Complex32::new(0.0,0.0), Complex32::new(0.0,1.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0)],
        vec![Complex32::new(0.0,-1.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0)]
    ]);

    let c = a.tensor(b);

    assert!(operators_are_equal(expected,c));
}

#[test]
fn tensor_smaller_works() {
    let a = Operator::new(vec![
        vec![Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,1.0)],
        vec![Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,-1.0), Complex32::new(0.0,0.0)],
        vec![Complex32::new(0.0,0.0), Complex32::new(0.0,1.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0)],
        vec![Complex32::new(0.0,-1.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0)]
    ]);

    let b = Operator::new(vec![
        vec![Complex32::new(1.0,0.0), Complex32::new(0.0,0.0)],
        vec![Complex32::new(0.0,0.0), Complex32::new(-1.0,0.0)]
    ]);

    let expected = Operator::new(vec![
        vec![Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,1.0), Complex32::new(0.0,0.0)],
        vec![Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,-1.0)],
        vec![Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,-1.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0)],
        vec![Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,1.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0)],
        vec![Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,1.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0)],
        vec![Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,-1.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0)],
        vec![Complex32::new(0.0,-1.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0)],
        vec![Complex32::new(0.0,0.0), Complex32::new(0.0,1.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0)],
    ]);

    let c = a.tensor(b);

    assert!(operators_are_equal(expected,c));
}

#[test]
fn tensor_greater_works() {
    let a = Operator::new(vec![
        vec![Complex32::new(1.0,0.0), Complex32::new(0.0,0.0)],
        vec![Complex32::new(0.0,0.0), Complex32::new(-1.0,0.0)]
    ]);

    let b = Operator::new(vec![
        vec![Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,1.0)],
        vec![Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,-1.0), Complex32::new(0.0,0.0)],
        vec![Complex32::new(0.0,0.0), Complex32::new(0.0,1.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0)],
        vec![Complex32::new(0.0,-1.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0)]
    ]);

    let expected = Operator::new(vec![
        vec![Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,1.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0)],
        vec![Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,-1.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0)],
        vec![Complex32::new(0.0,0.0), Complex32::new(0.0,1.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0)],
        vec![Complex32::new(0.0,-1.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0)],
        vec![Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,-1.0)],
        vec![Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,1.0), Complex32::new(0.0,0.0)],
        vec![Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,-1.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0)],
        vec![Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,1.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0)],
    ]);

    let c = a.tensor(b);

    assert!(operators_are_equal(expected,c));
}

#[test]
#[should_panic(expected = "Cannot apply operator of size 4 to statevector of size 2")]
fn apply_to_smaller_length_statevector_panics() {
    let operator = Operator::new(vec![
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
    let operator = Operator::new(vec![
        vec![Complex32::new(0.0,0.0), Complex32::new(1.0,0.0)],
        vec![Complex32::new(1.0,0.0), Complex32::new(0.0,0.0)]
    ]);

    let statevector = Statevector::new(vec![Complex32::new(1.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0), Complex32::new(0.0,0.0)]);

    operator.apply(statevector);
}

#[test]
fn apply_works() {
    let operator = Operator::new(vec![
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

fn operators_are_equal(a:Operator, b:Operator) -> bool
{
    if a.data().len() != b.data().len()
    {
        return false;
    }

    for i in 0..a.data().len()
    {
        for j in 0..a.data().len() 
        {
            if !(tools::equals(a.data()[i][j].re, b.data()[i][j].re) && tools::equals(a.data()[i][j].im, b.data()[i][j].im))
            {
                return false;
            }
        }
    }

    return true;
}