#[cfg(test)]
extern crate moara;
extern crate num_complex;

use num_complex::Complex32;
use moara::operator::Operator;
use moara::gate::Gate;


#[test]
fn print(){
  let sw3=Gate::swap(vec![0, 1]);
    println!("sw_3={:?}",&sw3.matrix().data())
}