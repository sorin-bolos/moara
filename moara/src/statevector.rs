extern crate num_complex;

use num_complex::Complex32;
use crate::tools;

pub struct Statevector 
{
    pub data:Vec<Complex32>
}

impl Statevector
{
    pub fn new(data:Vec<Complex32>) -> Statevector {

        if !is_unitary_vector(&data)
        {
            panic!("Vector is not unitary")
        }

        Statevector {
            data:data
        }
    }
}

fn is_unitary_vector(data:&Vec<Complex32>) -> bool
{
    let sum = data.into_iter()
                  .map(|amplitude| amplitude.norm())
                  .map(|norm| norm*norm)
                  .fold(0.0, |sum,squared_amplitudes| sum+squared_amplitudes);

    tools::equals(sum, 1.0)
}