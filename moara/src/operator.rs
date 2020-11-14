extern crate num_complex;

use num_complex::Complex32;
use crate::statevector::Statevector;

pub struct ElementAtPosition {
    pub position:usize,
    pub element:Complex32
}

pub trait Operator {

    fn size(&self) -> usize;

    fn get(&self, i:usize, j:usize) -> Complex32;

    fn non_zero_elements_for_row(&self, row:usize) -> Vec<ElementAtPosition>;

    fn apply(&self, statevector:Statevector) -> Statevector {
        let len = self.size();
    
        if len != statevector.data().len()
        {
            panic!("Cannot apply operator of size {} to statevector of size {}", len, statevector.data().len());
        }
    
        let mut result = vec![Complex32::new(0.0, 0.0); len];
    
        for i in 0..len
        {
            let mut elem = C!(0);
            for eap in self.non_zero_elements_for_row(i) {
                elem += eap.element*statevector.data()[eap.position];
            }
            result[i] = elem;            
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

    fn non_zero_elements_for_row(&self, row:usize) -> Vec<ElementAtPosition> {
        self.data[row].iter()
                      .enumerate()
                      .map(|enumeration| ElementAtPosition { position:enumeration.0, element:*enumeration.1 })
                      .collect()
    }
}

pub struct IdentityTensorOperator
{
    size:usize,
    inner_operator:Box<dyn Operator>
}

impl IdentityTensorOperator {
    pub fn new(size: usize, inner_operator: Box<dyn Operator>) -> Self {
        Self {
            size: size,
            inner_operator: inner_operator
        }
    }
}

impl Operator for IdentityTensorOperator {
    
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

    fn non_zero_elements_for_row(&self, row:usize) -> Vec<ElementAtPosition> {
        let m = row % self.inner_operator.size();
        self.inner_operator
            .non_zero_elements_for_row(m)
            .iter()
            .map(|eap| ElementAtPosition{position:eap.position + row - m, element:eap.element})
            .collect()
    }
}

pub struct TensorIdentityOperator
{
    size:usize,
    inner_operator:Box<dyn Operator>
}

impl TensorIdentityOperator {
    pub fn new(size: usize, inner_operator: Box<dyn Operator>) -> Self {
        Self {
            size: size,
            inner_operator: inner_operator
        }
    }
}

impl Operator for TensorIdentityOperator {
    
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

    fn non_zero_elements_for_row(&self, row:usize) -> Vec<ElementAtPosition> {
        let n = self.size / self.inner_operator.size();
        self.inner_operator
            .non_zero_elements_for_row(row / n)
            .iter()
            .map(|eap| ElementAtPosition{position:eap.position*n + (row % n), element:eap.element})
            .collect()
    }
}

pub struct CxOperator {
    size:usize,
    reversed:bool
}

impl CxOperator {
    pub fn new(qubit_span: u8, reversed:bool) -> Self {
        Self {
            size: 1 << qubit_span,
            reversed: reversed
        }
    }
}

impl Operator for CxOperator {
    fn size(&self) -> usize {
        self.size
    }

    fn get(&self, i:usize, j:usize) -> Complex32 {
        if i == j {
            return C!(1);
        } else {
            return C!(0)
        }
    }

    fn non_zero_elements_for_row(&self, row:usize) -> Vec<ElementAtPosition> {
        if self.reversed {
            if row % 2 == 0 {
                return vec![ElementAtPosition { position: row, element: C!(1) }]
            } else {
                return vec![ElementAtPosition { position: (self.size/2 + row)%self.size, element: C!(1) }]
            }
        } else {
            if row < self.size/2 {
                return vec![ElementAtPosition { position: row, element: C!(1) }]
            } else {
                if row % 2 == 0 {
                    return vec![ElementAtPosition { position: row+1, element: C!(1) }]
                } else {
                    return vec![ElementAtPosition { position: row-1, element: C!(1) }]
                }
            }
        }
    }
}