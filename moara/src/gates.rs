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

pub fn c() -> [Complex32; 4] {
  [C!(1-1*i)*0.5, C!(-1-1*i)*0.5, C!(1-1*i)*0.5, C!(1+1*i)*0.5]
}

pub fn c_dagger() -> [Complex32; 4] {
  [C!(1+1*i)*0.5, C!(1+1*i)*0.5, C!(-1+1*i)*0.5, C!(1-1*i)*0.5]
}

pub fn hadamard() -> [Complex32; 4] {
    [Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(-1.0/2.0_f32.sqrt(),0.0)]
}

pub fn hadamard_xy() -> [Complex32; 4] {
  [ C!(0), C!(1+1*i)/2.0_f32.sqrt(), C!(1-1*i)/2.0_f32.sqrt(), C!(0)]
}


pub fn hadamard_yz() -> [Complex32; 4] {
  [Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(0.0,-1.0/2.0_f32.sqrt()), Complex32::new(0.0,1.0/2.0_f32.sqrt()), Complex32::new(-1.0/2.0_f32.sqrt(),0.0)]
}


pub fn hadamard_zx() -> [Complex32; 4] {
  [Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(-1.0/2.0_f32.sqrt(),0.0)]
}


pub fn v() -> [Complex32; 4] {
    [C!(1+1*i)*0.5, C!(1-1*i)*0.5, C!(1-1*i)*0.5, C!(1+1*i)*0.5]
}

pub fn v_dagger() -> [Complex32; 4] {
  [C!(1-1*i)*0.5, C!(1+1*i)*0.5, C!(1+1*i)*0.5, C!(1-1*i)*0.5]
}

pub fn h() -> [Complex32; 4] {
  [Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(-1.0/2.0_f32.sqrt(),0.0), Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(1.0/2.0_f32.sqrt(),0.0)]
}

pub fn h_dagger() -> [Complex32; 4] {
  [Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(-1.0/2.0_f32.sqrt(),0.0), Complex32::new(1.0/2.0_f32.sqrt(),0.0)]
}

pub fn y_basis_to_standard_basis_rotation() -> [Complex32; 4] {
  [Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(0.0,-1.0/2.0_f32.sqrt()), Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(0.0, 1.0/2.0_f32.sqrt())]
}

pub fn standard_basis_to_y_basis_rotation() -> [Complex32; 4] {
  [Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(0.0, 1.0/2.0_f32.sqrt()), Complex32::new(0.0, -1.0/2.0_f32.sqrt())]
}

pub fn standard_basis_to_x_basis_rotation() -> [Complex32; 4] {
  [Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(-1.0/2.0_f32.sqrt(),0.0)]
}

pub fn x_basis_to_standard_basis_rotation() -> [Complex32; 4] {
  [Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(-1.0/2.0_f32.sqrt(),0.0)]
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

pub fn u3(theta:f32, phi:f32, lambda:f32) -> [Complex32; 4] {
    let half_theta = theta/2.0_f32;
    [C!((half_theta.cos())), -1.0*C!(lambda*i).exp()*half_theta.sin(), C!(phi*i).exp()*half_theta.sin(), C!((phi+lambda)*i).exp()*half_theta.cos()]
}

pub fn u2(phi:f32, lambda:f32) -> [Complex32; 4] {
    let one_over_sqrt2 = 1.0/2.0_f32.sqrt();
    [C!(one_over_sqrt2), -1.0*C!(lambda*i).exp()*one_over_sqrt2, C!(phi*i).exp()*one_over_sqrt2, C!((phi+lambda)*i).exp()*one_over_sqrt2]
}

pub fn u1(lambda:f32) -> [Complex32; 4] {
    [C!(1), C!(0), C!(0), C!(lambda*i).exp()]
}

pub fn p(theta:f32) -> [Complex32; 4] {
  [C!(1), C!(0), C!(0), C!(theta*i).exp()]
}


pub fn rx_theta(theta:f32) -> [Complex32; 4] {
    let half_theta = theta/2f32;
    let a = C!((half_theta.cos()));
    let b = C!(-1*i)*half_theta.sin();
    [a, b, b, a]
}

pub fn pauli_x_root(root:f32) -> [Complex32; 4] {
    let theta = PI/root;
    let half_theta = theta/2f32;
    let a = C!((half_theta.cos()))*C!(half_theta*i).exp();
    let b = C!(-1*i)*half_theta.sin()*C!(half_theta*i).exp();
    [a, b, b, a]
}

pub fn pauli_x_root_dagger(root:f32) -> [Complex32; 4] {
    let theta = PI/root;
    let half_theta = theta/2f32;
    let a = C!((half_theta.cos()))*C!(-half_theta*i).exp();
    let b = C!(1*i)*half_theta.sin()*C!(-half_theta*i).exp();
    [a, b, b, a]
}

pub fn ry_theta(theta:f32) -> [Complex32; 4] {
    let half_theta = theta/2f32;
    let a = C!((half_theta.cos()));
    let b = C!((half_theta.sin()));
    [a, -b, b, a]
}

pub fn pauli_y_root(root:f32) -> [Complex32; 4] {
    let theta = PI/root;
    let half_theta = theta/2f32;
    let a = C!((half_theta.cos()))*C!(half_theta*i).exp();
    let b = C!((half_theta.sin()))*C!(half_theta*i).exp();
    [a, -b, b, a]
}

pub fn pauli_y_root_dagger(root:f32) -> [Complex32; 4] {
    let theta = PI/root;
    let half_theta = theta/2f32;
    let a = C!((half_theta.cos()))*C!(-half_theta*i).exp();
    let b = C!((half_theta.sin()))*C!(-half_theta*i).exp();
    [a, b, -b, a]
}

pub fn rz_theta(theta:f32) -> [Complex32; 4] {
    let half_theta = theta/2f32;
    [C!(-half_theta*i).exp(), C!(0), C!(0), C!(half_theta*i).exp()]
}
    
pub fn pauli_z_root(root:f32) -> [Complex32; 4] {
    let theta = PI/root;
    [C!(1), C!(0), C!(0), C!(theta*i).exp()]
}

pub fn pauli_z_root_dagger(root:f32) -> [Complex32; 4] {
    let theta = PI/root;
    [C!(1), C!(0), C!(0), C!(-theta*i).exp()]
}

//for y measurement
pub fn hadamard_times_s_dagger() -> [Complex32; 4] {
    [Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(0.0,-1.0/2.0_f32.sqrt()), Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(0.0,1.0/2.0_f32.sqrt())]
    }

pub fn swap() -> [Complex32; 16] {
    [C!(1), C!(0), C!(0), C!(0),
     C!(0), C!(0), C!(1), C!(0),
     C!(0), C!(1), C!(0), C!(0),
     C!(0), C!(0), C!(0), C!(1)]
}

pub fn swap_with_add_phase(phase:f32) -> [Complex32; 16] {
    [C!(1), C!(0), C!(0), C!(0),
     C!(0), C!(0), C!(phase*i).exp(), C!(0),
     C!(0), C!(phase*i).exp(), C!(0), C!(0),
     C!(0), C!(0), C!(0), C!(1)]
}

pub fn iswap() -> [Complex32; 16] {
    [C!(1), C!(0), C!(0), C!(0),
     C!(0), C!(0), C!(1*i), C!(0),
     C!(0), C!(1*i), C!(0), C!(0),
     C!(0), C!(0), C!(0), C!(1)]
}

pub fn fswap() -> [Complex32; 16] {
  [C!(1), C!(0), C!(0), C!(0),
   C!(0), C!(0), C!(1), C!(0),
   C!(0), C!(1), C!(0), C!(0),
   C!(0), C!(0), C!(0), C!(-1)]
}

pub fn sqrt_swap() -> [Complex32; 16] {
    [C!(1), C!(0), C!(0), C!(0),
     C!(0), C!(1+1*i)*0.5, C!(1-1*i)*0.5, C!(0),
     C!(0), C!(1-1*i)*0.5, C!(1+1*i)*0.5, C!(0),
     C!(0), C!(0), C!(0), C!(1)]
}

pub fn sqrt_swap_dagger() -> [Complex32; 16] {
  [C!(1), C!(0), C!(0), C!(0),
   C!(0), C!(1-1*i)*0.5, C!(1+1*i)*0.5, C!(0),
   C!(0), C!(1+1*i)*0.5, C!(1-1*i)*0.5, C!(0),
   C!(0), C!(0), C!(0), C!(1)]
}

pub fn swap_root(root:f32) -> [Complex32; 16] {
  let theta = PI/root;
  let phase = (C!(-theta*i)/4_f32).exp();
  let half_theta = theta/2f32;

  [phase * (C!(theta*i)/2_f32).exp(), C!(0), C!(0), C!(0),
   C!(0), phase * half_theta.cos(), phase * C!(1*i) * half_theta.sin(), C!(0),
   C!(0), phase * C!(1*i) * half_theta.sin(), phase * half_theta.cos(), C!(0),
   C!(0), C!(0), C!(0), phase * (C!(theta*i)/2_f32).exp()]
}

pub fn swap_root_dagger(root:f32) -> [Complex32; 16] {
  let theta = -PI/root;
  let phase = (C!(-theta*i)/4_f32).exp();
  let half_theta = theta/2f32;

  [phase * (C!(theta*i)/2_f32).exp(), C!(0), C!(0), C!(0),
   C!(0), phase * half_theta.cos(), phase * C!(1*i) * half_theta.sin(), C!(0),
   C!(0), phase * C!(1*i) * half_theta.sin(), phase * half_theta.cos(), C!(0),
   C!(0), C!(0), C!(0), phase * (C!(theta*i)/2_f32).exp()]
}

pub fn xx(theta:f32) -> [Complex32; 16] {
    let half_theta = theta/2f32;
    let a = C!((half_theta.cos()));
    let b = C!(-1*i)*half_theta.sin();

    [a, C!(0), C!(0), b,
     C!(0), a, b, C!(0),
     C!(0), b, a, C!(0),
     b, C!(0), C!(0), a]
}

pub fn yy(theta:f32) -> [Complex32; 16] {
    let half_theta = theta/2f32;
    let a = C!((half_theta.cos()));
    let b = C!(1*i)*half_theta.sin();

    [a, C!(0), C!(0), b,
     C!(0), a, -b, C!(0),
     C!(0), -b, a, C!(0),
     b, C!(0), C!(0), a]
}

pub fn zz(theta:f32) -> [Complex32; 16] {
    let half_theta = theta/2f32;
    let a = C!(-half_theta*i).exp();
    let b = C!(half_theta*i).exp();

    [a, C!(0), C!(0), C!(0),
     C!(0), b, C!(0), C!(0),
     C!(0), C!(0), b, C!(0),
     C!(0), C!(0), C!(0), a]
}

pub fn xy(theta:f32) -> [Complex32; 16] {
  let theta_cos = theta.cos();
  let theta_sin = theta.sin();

  [C!(1), C!(0), C!(0), C!(0),
   C!(0), C!(theta_cos), C!(-theta_sin*i), C!(0),
   C!(0), C!(-theta_sin*i), C!(theta_cos), C!(0),
   C!(0), C!(0), C!(0), C!(1)]
}

pub fn berkeley() -> [Complex32; 16] {

  let theta = PI/8_f32;
  let theta_times3 = 3_f32 * PI/8_f32;

  [C!((theta.cos())), C!(0), C!(0), C!((theta.sin())*i),
   C!(0), C!((theta_times3.cos())), C!((theta_times3.sin())*i), C!(0),
   C!(0), C!((theta.sin())*i), C!((theta_times3.cos())), C!(0),
   C!((theta.sin())*i), C!(0), C!(0), C!((theta.cos()))]
}

pub fn berkeley_dagger() -> [Complex32; 16] {

  let theta = PI/8_f32;
  let theta_times3 = 3_f32 * PI/8_f32;

  [C!((theta.cos())), C!(0), C!(0), C!(-(theta.sin())*i),
   C!(0), C!((theta_times3.cos())),  C!(-(theta_times3.sin())*i), C!(0),
   C!(0),  C!(-(theta.sin())*i), C!((theta_times3.cos())), C!(0),
   C!(-(theta.sin())*i), C!(0), C!(0), C!((theta.cos()))]
}

pub fn ecp() -> [Complex32; 16] {

  let c = (PI/8_f32).cos();
  let s = (PI/8_f32).sin();

  [C!((2_f32*c)), C!(0), C!(0), C!((-2_f32*s)*i),
   C!(0), C!(1+1*i)*(c-s), C!(1-1*i)*(c+s), C!(0),
   C!(0), C!(1-1*i)*(c+s), C!(1+1*i)*(c-s), C!(0),
   C!((-2_f32*s)*i), C!(0), C!(0), C!((2_f32*c))]
}

pub fn ecp_dagger() -> [Complex32; 16] {

  let c = (PI/8_f32).cos();
  let s = (PI/8_f32).sin();

  [C!((2_f32*c)), C!(0), C!(0), C!((2_f32*s)*i),
   C!(0), C!(1-1*i)*(c-s), C!(1+1*i)*(c+s), C!(0),
   C!(0), C!(1+1*i)*(c+s), C!(1-1*i)*(c-s), C!(0),
   C!((2_f32*s)*i), C!(0), C!(0), C!((2_f32*c))]
}

pub fn w() -> [Complex32; 16] {
  [C!(1), C!(0), C!(0), C!(0),
   C!(0), Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(1.0/2.0_f32.sqrt(),0.0), C!(0),
   C!(0), Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(-1.0/2.0_f32.sqrt(),0.0), C!(0),
   C!(0), C!(0), C!(0), C!(1)]
}


pub fn magic() -> [Complex32; 16] {
  [C!(1), C!(1*i), C!(0), C!(0),
   C!(0), C!(0), C!(1*i), C!(1),
   C!(0), C!(0), C!(1*i), C!(-1),
   C!(1), C!(-1*i), C!(0), C!(0)]
}

pub fn magic_dagger() -> [Complex32; 16] {
  [C!(1), C!(0), C!(0), C!(1),
   C!(-1*i), C!(0), C!(0), C!(1*i),
   C!(0), C!(-1*i), C!(-1*i), C!(0),
   C!(0), C!(1), C!(-1), C!(0)]
}


pub fn cross_resonance(theta:f32) -> [Complex32; 16] {
  let half_theta = theta/2f32;

  [C!((half_theta.cos())), C!((-half_theta.sin())*i), C!(0), C!(0),
   C!((-half_theta.sin())*i), C!((half_theta.cos())), C!(0), C!(0),
   C!(0), C!(0), C!((half_theta.cos())), C!((half_theta.sin())*i),
   C!(0), C!(0), C!((half_theta.sin())*i), C!((half_theta.cos()))]
}

pub fn cross_resonance_dagger(theta:f32) -> [Complex32; 16] {
  let half_theta = theta/2f32;

  [C!((half_theta.cos())), C!((half_theta.sin())*i), C!(0), C!(0),
   C!((half_theta.sin())*i), C!((half_theta.cos())), C!(0), C!(0),
   C!(0), C!(0), C!((half_theta.cos())), C!((-half_theta.sin())*i),
   C!(0), C!(0), C!((-half_theta.sin())*i), C!((half_theta.cos()))]
}

pub fn givens(theta:f32) -> [Complex32; 16] {

  let theta_cos = theta.cos();
  let theta_sin = theta.sin();

  [C!(1), C!(0), C!(0), C!(0),
   C!(0), C!(theta_cos), C!(-theta_sin), C!(0),
   C!(0), C!(theta_sin), C!(theta_cos), C!(0),
   C!(0), C!(0), C!(0), C!(1)]

}

pub fn a(theta:f32, phi:f32) -> [Complex32; 16] {

  [C!(1), C!(0), C!(0), C!(0),
   C!(0), C!((theta.cos())), C!((theta.sin())) * C!(phi*i).exp(), C!(0),
   C!(0), C!((theta.sin())) * C!(-phi*i).exp(), C!((-theta.cos())), C!(0),
   C!(0), C!(0), C!(0), C!(1)]
}

pub fn molmer_sorensen() -> [Complex32; 16] {
  let factor = 1.0/2.0_f32.sqrt();

  [C!(factor), C!(0), C!(0), C!(factor*i),
   C!(0), C!(factor), C!(factor*i), C!(0),
   C!(0), C!(factor*i), C!(factor), C!(0),
   C!(factor*i), C!(0), C!(0), C!(factor)]
}


pub fn molmer_sorensen_dagger() -> [Complex32; 16] {
  let factor = 1.0/2.0_f32.sqrt();

  [C!(factor), C!(0), C!(0), C!(-factor*i),
   C!(0), C!(factor), C!(-factor*i), C!(0),
   C!(0), C!(-factor*i), C!(factor), C!(0),
   C!(-factor*i), C!(0), C!(0), C!(factor)]
}
