

extern crate num_complex;

use num_complex::Complex32;
use crate::operator::Operator;

pub fn idenity() -> Operator {
    Operator::new(vec![
        vec![Complex32::new(1.0,0.0),Complex32::new(0.0,0.0)],
        vec![Complex32::new(0.0,0.0),Complex32::new(1.0,0.0)]])
}

pub fn pauli_x() -> Operator {
    Operator::new(vec![
        vec![Complex32::new(0.0,0.0),Complex32::new(1.0,0.0)],
        vec![Complex32::new(1.0,0.0),Complex32::new(0.0,0.0)]])
}

pub fn pauli_y() -> Operator {
    Operator::new(vec![
        vec![Complex32::new(0.0,0.0),Complex32::new(0.0,-1.0)],
        vec![Complex32::new(0.0,1.0),Complex32::new(0.0,0.0)]])
}

pub fn pauli_z() -> Operator {
    Operator::new(vec![
        vec![Complex32::new(1.0,0.0),Complex32::new(0.0,0.0)],
        vec![Complex32::new(0.0,0.0),Complex32::new(-1.0,0.0)]])
}

pub fn hadamard() -> Operator {
    Operator::new(vec![
        vec![Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(1.0/2.0_f32.sqrt(),0.0)],
        vec![Complex32::new(1.0/2.0_f32.sqrt(),0.0), Complex32::new(-1.0/2.0_f32.sqrt(),0.0)]])
}

pub fn u3_gate(theta:f32,phi:f32,lambda:f32) -> Operator {
    println!("phi :{:?}",phi);
    Operator::new(vec![
        vec![Complex32::new((theta/2.0).cos(),0.0),Complex32::new(-1.0*(lambda.cos())*((theta/2.0).sin()),0.0)+Complex32::new(0.0,-1.0*(lambda.sin())*((theta/2.0).sin()))],
        vec![Complex32::new(phi.cos()*((theta/2.0).sin()),0.0)+Complex32::new(0.0,phi.sin()*((theta/2.0).sin())),Complex32::new(((phi+lambda).cos())*((theta/2.0).cos()),0.0)+Complex32::new(0.0,((phi+lambda).sin())*((theta/2.0).cos()))]])
}

pub fn cx(qubit_span:u8, reversed:bool) -> Operator {
    let size = 1<<qubit_span;
    let mut data = vec![vec![Complex32::new(0.0,0.0);size];size];

    if reversed {
        for i in 0..size {
            if i % 2 == 0 {
                data[i][i] = Complex32::new(1.0,0.0);
            } else {
                data[i][(size/2 + i)%size] = Complex32::new(1.0,0.0);
            }
        }
    } else {
        for i in 0..size {
            if i < size/2 {
                data[i][i] = Complex32::new(1.0,0.0);
            } else {
                if i % 2 == 0 {
                    data[i][i+1] = Complex32::new(1.0,0.0);
                } else {
                    data[i][i-1] = Complex32::new(1.0,0.0);
                }
            }
        }
    }
    
    Operator::new(data)
}

pub fn swap(qubit_span:u8) -> Operator {
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
    
    Operator::new(data)
}

// pub fn CU3(theta:f32,phi:f32,lambda:f32)->Operator{
//     return Operator::new(vec![
//         vec![Complex32::new(1.0,0.0),Complex32::new(0.0,0.0),Complex32::new(0.0,0.0),Complex32::new(0.0,0.0)],
//         vec![Complex32::new(0.0,0.0),Complex32::new(1.0,0.0),Complex32::new(0.0,0.0),Complex32::new(0.0,0.0)],
//         vec![Complex32::new(0.0,0.0),Complex32::new(0.0,0.0),Complex32::new((theta).cos(),0.0),Complex32::new(-1.0*(lambda.cos())*(theta.sin()),0.0)+Complex32::new(0.0,-1.0*(lambda.sin())*(theta.sin()))],
//         vec![Complex32::new(0.0,0.0),Complex32::new(0.0,0.0),Complex32::new(phi.cos()*(theta.sin()),0.0)+Complex32::new(0.0,phi.sin()*(theta.sin())),Complex32::new((phi+lambda).cos()*(theta.cos()),0.0)+Complex32::new(0.0,(phi+lambda).cos()*(theta.sin()))]])
// }

// pub fn cu3_gate(q1:usize,q2:usize,theta:f32,phi:f32,lambda:f32)->Operator{
//     let mut nrq=&q2-&q1+1;
//     let mut p= ((&q2-&q1)+1) as u32;

//     let mut operator= Operator::unit(2usize.pow(p));
//     if q2<q1 {
//         nrq=&q1-&q2+1;
//         p= ((&q1-&q2)+1) as u32;
//         operator=operator.dot(&sw_gate(q1,q2));
//     } 
//     let i=Operator::new(vec![vec![Complex32::new(1.0,0.0),Complex32::new(0.0,0.0)],vec![Complex32::new(0.0,0.0),Complex32::new(1.0,0.0)]]);
   
//     let sw=Operator::new(vec![
//         vec![Complex32::new(1.0,0.0),Complex32::new(0.0,0.0),Complex32::new(0.0,0.0),Complex32::new(0.0,0.0)],
//         vec![Complex32::new(0.0,0.0),Complex32::new(0.0,0.0),Complex32::new(1.0,0.0),Complex32::new(0.0,0.0)],
//         vec![Complex32::new(0.0,0.0),Complex32::new(1.0,0.0),Complex32::new(0.0,0.0),Complex32::new(0.0,0.0)],
//         vec![Complex32::new(0.0,0.0),Complex32::new(0.0,0.0),Complex32::new(0.0,0.0),Complex32::new(1.0,0.0)]]);

//     let mut w=Operator::new(vec![vec![Complex32::new(1.0,0.0)]]);
//     for l in 0..(nrq-2){
//         w=Operator::new(vec![vec![Complex32::new(1.0,0.0)]]);
//         for k in 0..(nrq-1){
//             if k ==l{
//                 w=w.tensor(&sw);
//             }   
//             else {
//                 w=w.tensor(&i);
//             }
//         }
//         operator=operator.dot(&w);
//     }
    

//     w=Operator::new(vec![vec![Complex32::new(1.0,0.0)]]);
//     for k in 0..(nrq-2){
//         w=w.tensor(&i);          
//     }
//     w=w.tensor(&CU3(theta,phi,lambda));
//     operator=operator.dot(&w);
    

//     for l in 0..(nrq-2){
//         w=Operator::new(vec![vec![Complex32::new(1.0,0.0)]]);
//         for k in 0..(nrq-1){
//             if k ==(nrq-3-l){
//                 w=w.tensor(&sw);
//             }   
//             else {
//                 w=w.tensor(&i);
//             }
//         }
//         operator=operator.dot(&w);
//     }
//     if q2<q1 {
//         operator=operator.dot(&sw_gate(q1,q2));
//     } 
//     return operator;
// }