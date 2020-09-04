extern crate num_complex;

use num_complex::Complex32;

use crate::operator::Operator;

pub struct Circ{
  nrQub:i32;
  nrBit:i32;
  operator:Operator; 
} 

impl Circ 
{
    pub fn new (nrQub:i32,nrBit:i32)->Self{
        Self{
            nrQub:nrQub,
            nrBit:nrBit,
            operator:
        }
    }
}