extern crate num_complex;

use num_complex::Complex32;
use crate::tools;
use crate::statevector::Statevector;

pub struct Operator 
{
    data:Vec<Vec<Complex32>>
}

impl Operator
{
    pub fn new(data:Vec<Vec<Complex32>>) -> Self {

        if !is_unitary_operator(&data)
        {
            panic!("Operator is not unitary")
        }

        Self {
            data:data
        }
    }

    pub fn data(&self) -> &Vec<Vec<Complex32>>
    {
        &self.data
    }

    pub fn dot(&self, other:Self) -> Self
    {
        let len = self.data.len();

        if len != other.data.len()
        {
            panic!("Cannot combine operator of size {} with operator of size {}", len, other.data.len());
        }

        let mut result = vec![vec![Complex32::new(0.0, 0.0); len]; len];

        for i in 0..len
        {
            for j in 0..len
            {
                for k in 0..len
                {
                    result[i][j] += self.data[i][k]*other.data[k][j];
                }
            }
        }

        Operator {
            data:result
        }
    }

    pub fn tensor(&self, other:Self) -> Self {
        let self_len = self.data.len();
        let other_len = other.data.len();
        let new_len = self_len*other_len;
        let mut tensored = vec![vec![Complex32::new(0.0, 0.0); new_len]; new_len];

        for i in 0..self_len
        {
            for j in 0..self_len
            {
                for k in 0..other_len
                {
                    for l in 0..other_len
                    {
                        tensored[i*other_len + k][j*other_len + l] = self.data[i][j]*other.data[k][l];
                    }
                }
            }
        }

        Operator {
            data:tensored
        }
    }

    pub fn apply(&self, statevector:Statevector) -> Statevector {
        let len = self.data.len();

        if len != statevector.data().len()
        {
            panic!("Cannot apply operator of size {} to statevector of size {}", len, statevector.data().len());
        }

        let mut result = vec![Complex32::new(0.0, 0.0); len];

        for i in 0..len
        {
            for j in 0..len
            {
                result[i] += self.data[i][j]*statevector.data()[j];
            }
        }

        Statevector::new(result)
    }
}

fn is_unitary_operator(data:&Vec<Vec<Complex32>>) -> bool
{
    for i in 0..data.len()
    {
        for j in 0..data.len()
        {
            if data.len() != data[i].len()
            {
                panic!("Matrix is not square")
            }

            let mut elem = Complex32::new(0.0, 0.0);
            for k in 0..data[i].len()
            {
                elem += data[i][k]*data[k][j];
            }
            if i == j
            {
                if !(tools::equals(1.0, elem.re) && tools::equals(0.0, elem.im))
                {
                    return false;
                }
            }
            else 
            {
                if !(tools::equals(0.0, elem.re) && tools::equals(0.0, elem.im))
                {
                    return false;
                }
            }
        }
    }

    return true;
}