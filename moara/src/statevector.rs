extern crate num_complex;

use num_complex::Complex32;
use crate::tools;

pub struct Statevector 
{
    data:Vec<Complex32>
}

impl Statevector
{
    pub fn new(data:Vec<Complex32>) -> Self {

        if !is_unitary_vector(&data)
        {
            panic!("Vector is not unitary")
        }

        Self {
            data:data
        }
    }

    pub fn data(&self) -> &Vec<Complex32>
    {
        &self.data
    }

    pub fn tensor(&self, other:Self) -> Self {
        let self_len = self.data.len();
        let other_len = other.data.len();
        let mut tensored = vec![Complex32::new(0.0, 0.0); self_len*other_len];

        for i in 0..self_len
        {
            for j in 0..other_len
            {
                tensored[i*other_len + j] = self.data[i]*other.data[j];
            }
        }

        Statevector {
            data:tensored
        }
    }
}

fn is_unitary_vector(data:&Vec<Complex32>) -> bool
{
    let sum = data.iter()
                  .map(|amplitude| amplitude.norm())
                  .map(|norm| norm*norm)
                  .fold(0.0, |sum,squared_amplitudes| sum+squared_amplitudes);

    tools::equals(sum, 1.0)
}