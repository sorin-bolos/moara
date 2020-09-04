extern crate num_complex;

use num_complex::Complex32;
use crate::operator::Operator;

pub struct Gate{
    target:Vec<usize>,
    matrix:Operator,
}

impl Gate{
    pub fn new(target:Vec<usize>,matrix:Operator)->Self{
        
        Self{
            target:target,
            matrix:matrix,
        }
    }
    pub fn matrix(&self)->&Operator{
         &self.matrix
    }
    pub fn target(&self)->&Vec<usize>{
        &self.target
    }
}

// 1 Qbit gates 
impl Gate{
    pub fn i(i:Vec<usize>)->Gate{
        return Gate::new(i,Operator::new(vec![vec![Complex32::new(1.0,0.0),Complex32::new(0.0,0.0)],vec![Complex32::new(0.0,0.0),Complex32::new(1.0,0.0)]]));
    }
    pub fn x(i:Vec<usize>)->Gate{
        return Gate::new(i,Operator::new(vec![vec![Complex32::new(0.0,0.0),Complex32::new(1.0,0.0)],vec![Complex32::new(1.0,0.0),Complex32::new(0.0,0.0)]]));
    }
    pub fn y(i:Vec<usize>)->Gate{
        return Gate::new(i,Operator::new(vec![vec![Complex32::new(0.0,0.0),Complex32::new(0.0,-1.0)],vec![Complex32::new(0.0,1.0),Complex32::new(0.0,0.0)]]));
    }
    pub fn z(i:Vec<usize>)->Gate{
        return Gate::new(i,Operator::new(vec![vec![Complex32::new(1.0,0.0),Complex32::new(0.0,0.0)],vec![Complex32::new(0.0,0.0),Complex32::new(-1.0,0.0)]]));
    }
    pub fn h(i:Vec<usize>)->Gate{
        return Gate::new(i,Operator::new(vec![vec![Complex32::new(1.0/2.0_f32.sqrt(),0.0),Complex32::new(1.0/2.0_f32.sqrt(),0.0)],vec![Complex32::new(1.0/2.0_f32.sqrt(),0.0),Complex32::new(-1.0/2.0_f32.sqrt(),0.0)]]));
    }
    pub fn u3(i:Vec<usize>,theta:f32,phi:f32,lambda:f32)->Gate{
        //theta=theta/2.0;
        return Gate::new(i,Operator::new(vec![vec![Complex32::new((theta).cos(),0.0),Complex32::new(-1.0*(lambda.cos())*(theta.sin()),0.0)+Complex32::new(0.0,-1.0*(lambda.sin())*(theta.sin()))],
        vec![Complex32::new(phi.cos()*(theta.sin()),0.0)+Complex32::new(0.0,phi.sin()*(theta.sin())),Complex32::new((phi+lambda).cos()*(theta.cos()),0.0)+Complex32::new(0.0,(phi+lambda).cos()*(theta.sin()))]]));
    }
}

// 2 Qbit gates
impl Gate{
    pub fn swap(j:Vec<usize>)->Gate{

        let nrq=&j[1]-&j[0]+1;//number  of qbits covered by swap
        //println!("nrq={}",nrq);

        //we need u32 to  create Identity oprator without using tensor  product between 2x2 I;
        let p= ((&j[1]-&j[0])+1) as u32;  
        let mut operator= Operator::unit(2usize.pow(p));
        //Operator for 1 qbit I
        let i=Operator::new(vec![vec![Complex32::new(1.0,0.0),Complex32::new(0.0,0.0)],vec![Complex32::new(0.0,0.0),Complex32::new(1.0,0.0)]]);
       
        //Swap operator for two close qbiti
        let sw=Operator::new(vec![
            vec![Complex32::new(1.0,0.0),Complex32::new(0.0,0.0),Complex32::new(0.0,0.0),Complex32::new(0.0,0.0)],
            vec![Complex32::new(0.0,0.0),Complex32::new(0.0,0.0),Complex32::new(1.0,0.0),Complex32::new(0.0,0.0)],
            vec![Complex32::new(0.0,0.0),Complex32::new(1.0,0.0),Complex32::new(0.0,0.0),Complex32::new(0.0,0.0)],
            vec![Complex32::new(0.0,0.0),Complex32::new(0.0,0.0),Complex32::new(0.0,0.0),Complex32::new(1.0,0.0)]]);
        //println!("sw=");sw.print();

        //The first part of swaps to bring qbitii closer.

        let mut w=Operator::new(vec![vec![Complex32::new(1.0,0.0)]]);
        for l in 0..(1+nrq-2)/2{
            w=Operator::new(vec![vec![Complex32::new(1.0,0.0)]]);
            for k in 0..(nrq-1){
                if k ==l{
                    w=w.tensor(&sw);
                    w.print();
                }   
                else {
                    w=w.tensor(&i);
                }
            }
            //println!("w{}=",l);w.print();
            operator=operator.dot(&w);
            //println!("It looks ok so far");
        }
        //println!("O[0]=");operator.print();//Operator for rhe first part.

        //Now we  apply the gate sw in these case. It would also work for cx.
        for k in 0..(nrq-2){
            w=Operator::new(vec![vec![Complex32::new(1.0,0.0)]]);
            w=w.tensor(&i);          
        }
        w=w.tensor(&sw);
        //println!("w[mij]=");w.print();
        operator=operator.dot(&w);
        //println!("O[1]=");operator.print();


        //W are mouving Qbits back.
        for l in 0..(1+nrq-2)/2{
            w=Operator::new(vec![vec![Complex32::new(1.0,0.0)]]);
            for k in 0..(nrq-1){
                if k ==(nrq-2-l){
                    w=w.tensor(&sw);
                }   
                else {
                    w=w.tensor(&i);
                }
            }
            //println!("w[{}]=",l);w.print();//Looks fine 

            operator=operator.dot(&w);

            //println!("O[2]");operator.print();//Asta nu e bun :(())
        }
        let result= Gate::new(j,operator);
        return result;
    }
    /*
    pub fn cx(j:Vec<usize>)->Gate{
        let cx=Operator::new(vec![
            vec![Complex32::new(1.0,0.0),Complex32::new(0.0,0.0),Complex32::new(0.0,0.0),Complex32::new(0.0,0.0)],
            vec![Complex32::new(0.0,0.0),Complex32::new(1.0,0.0),Complex32::new(0.0,0.0),Complex32::new(0.0,0.0)],
            vec![Complex32::new(0.0,0.0),Complex32::new(0.0,0.0),Complex32::new(0.0,0.0),Complex32::new(1.0,0.0)],
            vec![Complex32::new(0.0,0.0),Complex32::new(0.0,0.0),Complex32::new(1.0,0.0),Complex32::new(0.0,0.0)]]);
        
            let nrq=&j[1]-&j[0]+1;
            let p= ((&j[1]-&j[0])+1) as u32;
            let mut operator= Operator::unit(2usize.pow(p));
            let i=Operator::new(vec![vec![Complex32::new(1.0,0.0),Complex32::new(0.0,0.0)],vec![Complex32::new(0.0,0.0),Complex32::new(1.0,0.0)]]);
            let sw=Operator::new(vec![
                vec![Complex32::new(1.0,0.0),Complex32::new(0.0,0.0),Complex32::new(0.0,0.0),Complex32::new(0.0,0.0)],
                vec![Complex32::new(0.0,0.0),Complex32::new(0.0,0.0),Complex32::new(1.0,0.0),Complex32::new(0.0,0.0)],
                vec![Complex32::new(0.0,0.0),Complex32::new(1.0,0.0),Complex32::new(0.0,0.0),Complex32::new(0.0,0.0)],
                vec![Complex32::new(0.0,0.0),Complex32::new(0.0,0.0),Complex32::new(0.0,0.0),Complex32::new(1.0,0.0)]]);
            
            //swaps 
            let mut w=Operator::new(vec![vec![Complex32::new(1.0,0.0)]]);
            for l in 0..(1+nrq-2)/2{
                w=Operator::new(vec![vec![Complex32::new(1.0,0.0)]]);
                for k in 0..(nrq-1){
                    if k ==l{
                        w=w.tensor(&sw);
                    }   
                    else {
                        w=w.tensor(&i);
                    }
                }
                operator=operator.dot(&w);
                
            }
    
            //final 
        
            for k in 0..(nrq-2){
                w=Operator::new(vec![vec![Complex32::new(1.0,0.0)]]);
                w=w.tensor(&i);          
            }
            w=w.tensor(&cx);
            operator=operator.dot(&w);
            
            //reswaps
            for l in 0..(1+nrq-2)/2{
                w=Operator::new(vec![vec![Complex32::new(1.0,0.0)]]);
               
                for k in 0..(nrq-1){
                   
                    if k ==(nrq-2-l){
                        w=w.tensor(&sw);
                    }   
                    else {
                        w=w.tensor(&i);
                    }
                }
                operator=operator.dot(&w);
            
            }
            let result=Gate::new(j,operator);
            return result;
    }
    */
}




