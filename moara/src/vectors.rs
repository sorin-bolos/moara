use num_complex::Complex32;
use crate::statevector::Statevector;

pub fn zero(qubit_count:u8) -> Statevector
{
    let mut data = vec![Complex32::new(0.0,0.0); 1<<qubit_count ];
    data[0] = Complex32::new(1.0,0.0);

    Statevector::new(data)
}