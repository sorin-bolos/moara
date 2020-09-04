#[cfg(test)]
extern crate moara;
extern crate num_complex;

use num_complex::Complex32;
use moara::gate::Gate;
use moara::operator::Operator;

#[test]
fn one_qbit_gate_test() {
    let i=Gate::i(vec![1]);
    let x=Gate::x(vec![1]);
    let y=Gate::y(vec![5]);
    let z=Gate::i(vec![1]);
    let h=Gate::h(vec![9]);
    let ui=Gate::u3(vec![2],0.0,00.0,0.0);
    println!("i {:?}",&i.matrix().data());
    println!("X {:?}",&x.matrix().data());
    println!("y {:?}",&y.matrix().data());
    println!("z {:?}",&z.matrix().data());
    println!("h {:?}",&h.matrix().data());
    println!("u3 {:?}",&ui.matrix().data());
}

#[test]
fn two_qbit_gate(){
    let swap_0_1=Gate::swap(vec![0,1]);
    println!("swap(0,1)={:?}",&swap_0_1.matrix().data());
    let swap_1_2=Gate::swap(vec![1,2]);
    println!("swap(1,2)={:?}",&swap_1_2.matrix().data());
    let swap_0_2=Gate::swap(vec![0,2]);//Nu da bine 
    //swap_0_2.matrix().print();

}