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