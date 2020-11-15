extern crate num_complex;

use num_complex::Complex32;

pub struct Statevector 
{
    data:Vec<Complex32>
}

impl Statevector
{
    pub fn new(data:Vec<Complex32>) -> Self {
        Self {
            data:data
        }
    }

    pub fn data(&self) -> &Vec<Complex32>
    {
        &self.data
    }

    pub fn set_val(&mut self, i:usize, val:Complex32) {
        self.data[i] = val;
    }
}