extern crate num_complex;
mod tools;

use num_complex::Complex32;

struct Statevector 
{
    let data:Vec<Complex32>;
}

impl Statevector
{
    fn new(data:Vec<Complex32>) -> Statevector {

        if !is_unitary_vector(data)
        {
            panic!("Vector is not unitary")
        }

        Statevector {
            data = data
        }
    };
}

fn is_unitary_vector(data:Vec<Complex32>) -> bool
{
    let sum = data.map(|amplitude| amplitude.norm())
                  .map(|norm| norm*norm)
                  .fold(0, |sum,squared_amplitudes| sum+squared_amplitudes);

    tools::equals(sum, 1.0)
}