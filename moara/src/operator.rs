extern crate num_complex;

use num_complex::Complex32;
use crate::statevector::Statevector;

pub trait Operator {

    fn size(&self) -> usize;

    fn get(&self, i:usize, j:usize) -> Complex32;

    fn apply(&self, statevector:Statevector) -> Statevector {
        let len = self.size();
    
        if len != statevector.data().len()
        {
            panic!("Cannot apply operator of size {} to statevector of size {}", len, statevector.data().len());
        }
    
        let mut result = vec![Complex32::new(0.0, 0.0); len];
    
        for i in 0..len
        {
            for j in 0..len
            {
                result[i] += self.get(i,j)*statevector.data()[j];
            }
        }
    
        Statevector::new(result)
    }
}

pub struct MatrixOperator 
{
    data:Vec<Vec<Complex32>>
}

impl MatrixOperator {
    pub fn new(data:Vec<Vec<Complex32>>) -> Self {
        Self {
            data:data
        }
    }
}

impl Operator for MatrixOperator {
    fn size(&self) -> usize {
        self.data.len()
    }

    fn get(&self, i:usize, j:usize) -> Complex32 {
        self.data[i][j]
    }
}

pub struct IdentityTensorOperator<T> where T : Operator
{
    size:usize,
    inner_operator:T
}

impl<T> IdentityTensorOperator<T> where T: Operator {
    pub fn new(size: usize, inner_operator: T) -> Self {
        Self {
            size: size,
            inner_operator: inner_operator
        }
    }
}

impl<T> Operator for IdentityTensorOperator<T> where T : Operator {
    
    fn size(&self) -> usize {
        self.size
    }

    fn get(&self, i:usize, j:usize) -> Complex32 {
        let n = self.inner_operator.size();
        if i / n == j / n {
            return self.inner_operator.get(i % n,j % n)
        }
        return C!(0);
    }
}

pub struct TensorIdentityOperator<T> where T : Operator
{
    size:usize,
    inner_operator:T
}

impl<T> TensorIdentityOperator<T> where T: Operator {
    pub fn new(size: usize, inner_operator: T) -> Self {
        Self {
            size: size,
            inner_operator: inner_operator
        }
    }
}

impl<T> Operator for TensorIdentityOperator<T> where T : Operator {
    
    fn size(&self) -> usize {
        self.size
    }

    fn get(&self, i:usize, j:usize) -> Complex32 {
        let n = self.size / self.inner_operator.size();
        if i % n == j % n {
            return self.inner_operator.get(i / n ,j / n)
        }
        return C!(0);
    }
}