use core::f32::consts::PI;
use num_complex::Complex32;
use crate::operator::MatrixOperator;
use crate::operator::CxOperator;
use crate::C;

pub fn pauli_x() -> MatrixOperator {
    MatrixOperator::new(vec![
        vec![Complex32::new(0.0,0.0),Complex32::new(1.0,0.0)],
        vec![Complex32::new(1.0,0.0),Complex32::new(0.0,0.0)]])
}

pub fn pauli_y() -> MatrixOperator {
    MatrixOperator::new(vec![
        vec![Complex32::new(0.0,0.0),Complex32::new(0.0,-1.0)],
        vec![Complex32::new(0.0,1.0),Complex32::new(0.0,0.0)]])
}

pub fn pauli_z() -> MatrixOperator {
    MatrixOperator::new(vec![
        vec![Complex32::new(1.0,0.0),Complex32::new(0.0,0.0)],
        vec![Complex32::new(0.0,0.0),Complex32::new(-1.0,0.0)]])
}

pub fn hadamard() -> MatrixOperator {
    MatrixOperator::new(vec![
        vec![Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(1.0/2.0_f32.sqrt(),0.0)],
        vec![Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(-1.0/2.0_f32.sqrt(),0.0)]])
}

pub fn sqrt_not() -> MatrixOperator {
    MatrixOperator::new(vec![
        vec![C!(1+1*i)*0.5, C!(1-1*i)*0.5],
        vec![C!(1-1*i)*0.5, C!(1+1*i)*0.5]])
}

pub fn t() -> MatrixOperator {
    let pi_over_4 = PI/4_f32;
    MatrixOperator::new(vec![
        vec![C!(1), C!(0)],
        vec![C!(0), C!(pi_over_4*i).exp()]])
}


pub fn t_dagger() -> MatrixOperator {
    let minus_pi_over_4 = -PI/4_f32;
    MatrixOperator::new(vec![
        vec![C!(1), C!(0)],
        vec![C!(0), C!(minus_pi_over_4*i).exp()]])
}

pub fn s() -> MatrixOperator {
    MatrixOperator::new(vec![
        vec![C!(1), C!(0)],
        vec![C!(0), C!(1*i)]])
}


pub fn s_dagger() -> MatrixOperator {
    MatrixOperator::new(vec![
        vec![C!(1), C!(0)],
        vec![C!(0), C!(-1*i)]])
}

pub fn identity(qubit_count:u8) -> MatrixOperator {
    let size = 1<<qubit_count;
    let mut data = vec![vec![C!(0);size as usize];size as usize];

    for i in 0..(size as usize)
    {
        data[i][i] = C!(1);
    }

    MatrixOperator::new(data)
}

pub fn u3_gate(theta:f32,phi:f32,lambda:f32) -> MatrixOperator {
    let half_theta = theta/2f32;
    MatrixOperator::new(vec![
        vec![C!((half_theta.cos())), -1.0*C!(lambda*i).exp()*half_theta.sin()],
        vec![C!(phi*i).exp()*half_theta.sin(), C!((phi+lambda)*i).exp()*half_theta.cos()]])
}

pub fn u_phi_theta(phi:f32,theta:f32) -> MatrixOperator {
    let one_over_sqrt2 = 1.0/2.0_f32.sqrt();
    MatrixOperator::new(vec![
        vec![C!(one_over_sqrt2), -1.0*C!(theta*i).exp()*one_over_sqrt2],
        vec![C!(phi*i).exp()*one_over_sqrt2, C!((phi+theta)*i).exp()*one_over_sqrt2]])
}

pub fn r_phi(phi:f32) -> MatrixOperator {
    MatrixOperator::new(vec![
        vec![C!(1), C!(0)],
        vec![C!(0), C!(phi*i).exp()]])
}

pub fn rx_phi(phi:f32) -> MatrixOperator {
    let half_phi = phi/2f32;
    let a = C!((half_phi.cos()));
    let b = C!(-1*i)*half_phi.sin();
    MatrixOperator::new(vec![
        vec![a, b],
        vec![b, a]])
}

pub fn ry_phi(phi:f32) -> MatrixOperator {
    let half_phi = phi/2f32;
    let a = C!((half_phi.cos()));
    let b = C!((half_phi.sin()));
    MatrixOperator::new(vec![
        vec![a, -b],
        vec![b, a]])
}

pub fn rz_phi(phi:f32) -> MatrixOperator {
    let half_phi = phi/2f32;
    MatrixOperator::new(vec![
        vec![C!(-half_phi*i).exp(), C!(0)],
        vec![C!(0), C!(half_phi*i).exp()]])
}

pub fn cx(qubit_span:u8, reversed:bool) -> CxOperator {
    CxOperator::new(qubit_span, reversed)
}

pub fn swap(qubit_span:u8) -> MatrixOperator {
    let size = 1<<qubit_span;
    let mut data = vec![vec![Complex32::new(0.0,0.0);size];size];

    for i in 0..size {
        if i < size/2 {
            if i % 2 == 0 {
                data[i][i] = Complex32::new(1.0,0.0);
            } else {
                data[i][size/2+i-1] = Complex32::new(1.0,0.0);
            }
        } else {
            if i % 2 == 0 {
                data[i][i-size/2+1] = Complex32::new(1.0,0.0);
            } else {
                data[i][i] = Complex32::new(1.0,0.0);
            }
        }
    }
    
    MatrixOperator::new(data)
}