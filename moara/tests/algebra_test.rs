#[cfg(test)]
extern crate moara;
extern crate num_complex;

use num_complex::Complex32;
use moara::algebra::Matrix;

#[test]
fn add_test() {
    let x1= Matrix::new( vec![vec![Complex32::new(0.0,1.0),Complex32::new(1.0,0.0),Complex32::new(7.0,0.0),Complex32::new(1.0,0.0)],vec![Complex32::new(0.0,1.0),Complex32::new(1.0,0.0),Complex32::new(7.0,0.0),Complex32::new(1.0,0.0)],vec![Complex32::new(0.0,1.0),Complex32::new(1.0,0.0),Complex32::new(7.0,0.0),Complex32::new(1.0,0.0)],vec![Complex32::new(0.0,1.0),Complex32::new(1.0,0.0),Complex32::new(7.0,0.0),Complex32::new(1.0,0.0)]] );
    let  x2= Matrix::new( vec![vec![Complex32::new(0.0,1.0),Complex32::new(1.0,0.0),Complex32::new(7.0,0.0),Complex32::new(1.0,0.0)],vec![Complex32::new(0.0,1.0),Complex32::new(1.0,0.0),Complex32::new(7.0,0.0),Complex32::new(1.0,0.0)],vec![Complex32::new(0.0,1.0),Complex32::new(1.0,0.0),Complex32::new(7.0,0.0),Complex32::new(1.0,0.0)],vec![Complex32::new(0.0,1.0),Complex32::new(1.0,0.0),Complex32::new(7.0,0.0),Complex32::new(1.0,0.0)]] );
    let x3= x1.add(&x2);
    //println!("X1 {:?}",&x1.data());
    //println!("X2 {:?}",&x2.data());
    println!("X3 {:?}",&x3.data());
}

#[test]
fn tensor_test() {
    let x1= Matrix::new( vec![vec![Complex32::new(5.0,0.0)],vec![Complex32::new(0.0,1.0)]]);
    let x2= Matrix::new( vec![vec![Complex32::new(0.0,1.0),Complex32::new(0.0,1.0)],vec![Complex32::new(1.0,1.0),Complex32::new(9.0,1.0)]]);
    println!("X1 {:?}",&x1.data().len());
    println!("X2 {:?}",&x2.data());
    let x3= x1.tensor(&x2);
    println!("X3 {:?}",&x3.data());
}

#[test]
fn dot_test() {
    let a = Matrix::new(vec![
        vec![Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(1.0/2.0_f32.sqrt(),0.0)],
        vec![Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(-1.0/2.0_f32.sqrt(),0.0)]
    ]);

    let b = Matrix::new(vec![
        vec![Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(1.0/2.0_f32.sqrt(),0.0)],
        vec![Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(-1.0/2.0_f32.sqrt(),0.0)]
    ]);

    let i = Matrix::new(vec![
        vec![Complex32::new(1.0,0.0), Complex32::new(0.0,0.0)],
        vec![Complex32::new(0.0,0.0), Complex32::new(1.0,0.0)]
    ]);

    let c = a.dot(b);
    println!("C:={:?}",&c.data());
    println!("i:={:?}",&i.data());
}

#[test]
fn transpose_test() {
    let x1= Matrix::new( vec![vec![Complex32::new(5.0,0.0)],vec![Complex32::new(0.0,1.0)]]);
    let x3= x1.transpose();
    println!("X3 {:?}",&x3.data());
}