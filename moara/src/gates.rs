use core::f32::consts::PI;
use num_complex::Complex32;
use crate::C;

pub fn pauli_x() -> [Complex32; 4] {
    [Complex32::new(0.0,0.0), Complex32::new(1.0,0.0), Complex32::new(1.0,0.0), Complex32::new(0.0,0.0)]
}

pub fn pauli_y() -> [Complex32; 4] {
    [Complex32::new(0.0,0.0),Complex32::new(0.0,-1.0), Complex32::new(0.0,1.0),Complex32::new(0.0,0.0)]
}

pub fn pauli_z() -> [Complex32; 4] {
    [Complex32::new(1.0,0.0),Complex32::new(0.0,0.0), Complex32::new(0.0,0.0),Complex32::new(-1.0,0.0)]
}

pub fn hadamard() -> [Complex32; 4] {
    [Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(-1.0/2.0_f32.sqrt(),0.0)]
}

pub fn sqrt_not() -> [Complex32; 4] {
    [C!(1+1*i)*0.5, C!(1-1*i)*0.5, C!(1-1*i)*0.5, C!(1+1*i)*0.5]
}

pub fn t() -> [Complex32; 4] {
    let pi_over_4 = PI/4_f32;
    [C!(1), C!(0), C!(0), C!(pi_over_4*i).exp()]
}


pub fn t_dagger() -> [Complex32; 4] {
    let minus_pi_over_4 = -PI/4_f32;
    [C!(1), C!(0), C!(0), C!(minus_pi_over_4*i).exp()]
}

pub fn s() -> [Complex32; 4] {
    [C!(1), C!(0), C!(0), C!(1*i)]
}


pub fn s_dagger() -> [Complex32; 4] {
    [C!(1), C!(0), C!(0), C!(-1*i)]
}

pub fn identity() -> [Complex32; 4] {
    [C!(1), C!(0), C!(0), C!(1)]
}

pub fn u3_gate(theta:f32,phi:f32,lambda:f32) -> [Complex32; 4] {
    let half_theta = theta/2f32;
    [C!((half_theta.cos())), -1.0*C!(lambda*i).exp()*half_theta.sin(), C!(phi*i).exp()*half_theta.sin(), C!((phi+lambda)*i).exp()*half_theta.cos()]
}

pub fn u_phi_theta(phi:f32,theta:f32) -> [Complex32; 4] {
    let one_over_sqrt2 = 1.0/2.0_f32.sqrt();
    [C!(one_over_sqrt2), -1.0*C!(theta*i).exp()*one_over_sqrt2, C!(phi*i).exp()*one_over_sqrt2, C!((phi+theta)*i).exp()*one_over_sqrt2]
}

pub fn r_phi(phi:f32) -> [Complex32; 4] {
    [C!(1), C!(0), C!(0), C!(phi*i).exp()]
}

pub fn rx_phi(phi:f32) -> [Complex32; 4] {
    let half_phi = phi/2f32;
    let a = C!((half_phi.cos()));
    let b = C!(-1*i)*half_phi.sin();
    [a, b, b, a]
}

pub fn ry_phi(phi:f32) -> [Complex32; 4] {
    let half_phi = phi/2f32;
    let a = C!((half_phi.cos()));
    let b = C!((half_phi.sin()));
    [a, -b, b, a]
}

pub fn rz_phi(phi:f32) -> [Complex32; 4] {
    let half_phi = phi/2f32;
    [C!(-half_phi*i).exp(), C!(0), C!(0), C!(half_phi*i).exp()]
}

pub fn swap() -> [Complex32; 16] {
    [C!(1), C!(0), C!(0), C!(0),
     C!(0), C!(0), C!(1), C!(0),
     C!(0), C!(1), C!(0), C!(0),
     C!(0), C!(0), C!(0), C!(1)]
}