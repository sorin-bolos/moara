#[cfg(test)]
extern crate moara;
extern crate num_complex;

use num_complex::Complex32;
use moara::statevector::Statevector;

#[test]
fn create_with_unitary_vector_works() {
    Statevector::new(
        vec![Complex32::new(1.0/2.0,0.0),Complex32::new(0.0,1.0/2.0), Complex32::new(1.0/2.0,0.0), Complex32::new(0.0,1.0/2.0)]);
}

#[test]
#[should_panic(expected = "Vector is not unitary")]
fn create_with_non_unitary_vector_panics() {
    Statevector::new(
        vec![Complex32::new(1.0,0.0),Complex32::new(0.0,1.0/2.0)]);
}

#[test]
fn tensor_returns_correct_result() {
    let a = Statevector::new(vec![Complex32::new(1.0/2.0_f32.sqrt(),0.0),Complex32::new(0.0,1.0/2.0_f32.sqrt())]);
    let b = Statevector::new(vec![Complex32::new(1.0/3.0_f32.sqrt(),0.0),Complex32::new(1.0/3.0_f32.sqrt(),0.0),Complex32::new(0.0,1.0/3.0_f32.sqrt())]);

    let c = a.tensor(b);

    assert_eq!(vec![Complex32::new(1.0/6.0_f32.sqrt(), 0.0),
                    Complex32::new(1.0/6.0_f32.sqrt(), 0.0),
                    Complex32::new(0.0, 1.0/6.0_f32.sqrt()),
                    Complex32::new(0.0, 1.0/6.0_f32.sqrt()),
                    Complex32::new(0.0, 1.0/6.0_f32.sqrt()),
                    Complex32::new(-1.0/6.0_f32.sqrt(), 0.0)], *c.data())

}