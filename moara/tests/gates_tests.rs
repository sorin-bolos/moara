#[cfg(test)]

extern crate moara;
extern crate num_complex;

use core::f32::consts::PI;
use num_complex::Complex32;
use moara::operator::Operator;
use moara::operator::MatrixOperator;
use moara::gates;
use moara::C;
use moara::tools;

#[test]
fn swap_gives_correct_opertator() {
  let swap_4 = gates::swap(4);
  let expected_data = vec![
    vec![ C!(1), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(1), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(1), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(1), C!(0), C!(0), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(0), C!(1), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(1), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(1), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(1), C!(0)],
    vec![ C!(0), C!(1), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(1), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(1), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(1), C!(0), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(0), C!(0), C!(1), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(1), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(1), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(1)]];
    assert_eq!(expected_data, get_data(swap_4));

  let swap_2 = gates::swap(2);
  let expected_data_2 = vec![
    vec![ C!(1), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(1), C!(0)],
    vec![ C!(0), C!(1), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(1)]];
    assert_eq!(expected_data_2, get_data(swap_2));
  
}


#[test]
fn cx_gives_correct_opertator() {
 
  let cx_3 = gates::cx(3,true);
  let expected_data_3 = vec![
    vec![ C!(1), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(0), C!(0), C!(1), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(1), C!(0), C!(0), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(1)],
    vec![ C!(0), C!(0), C!(0), C!(0), C!(1), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(1), C!(0), C!(0), C!(0), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(0), C!(0), C!(0), C!(1), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(1), C!(0), C!(0), C!(0), C!(0)]];
  assert_eq!(expected_data_3, get_data(cx_3));

  let cx_2 = gates::cx(2,true);
  let expected_data_2 = vec![
    vec![ C!(1), C!(0), C!(0), C!(0)],
    vec![ C!(0), C!(0), C!(0), C!(1)],
    vec![ C!(0), C!(0), C!(1), C!(0)],
    vec![ C!(0), C!(1), C!(0), C!(0)]];
    assert_eq!(expected_data_2, get_data(cx_2));
}

#[test]
fn u3_gives_correct_opertator() {
    let u3_1= gates::u3_gate(0.0,0.0,0.0);
    let expected_data_1= vec![
      vec![ C!(1), C!(0)],
      vec![ C!(0), C!(1)]];
    assert_eq!(expected_data_1, get_data(u3_1));

    let u3_2= gates::u3_gate(PI/2.0,-PI/2.0,PI/2.0);
    let expected = MatrixOperator::new(vec![
      vec![ C!((1.0/2_f32.sqrt())), C!((-1.0/2_f32.sqrt())*i)],
      vec![ C!((-1.0/2_f32.sqrt())*i), C!((1.0/2_f32.sqrt()))]]);

    assert!(operators_are_equal(expected, u3_2));
}

fn operators_are_equal(a:impl Operator, b:impl Operator) -> bool
{
    if a.size() != b.size()
    {
        return false;
    }

    for i in 0..a.size()
    {
        for j in 0..a.size() 
        {
            if !(tools::equals(a.get(i,j).re, b.get(i,j).re) && tools::equals(a.get(i,j).im, b.get(i,j).im))
            {
                return false;
            }
        }
    }

    return true;
}

fn get_data(operator:impl Operator) -> Vec<Vec<Complex32>> {
    (0..operator.size()).map(|i| (0..operator.size()).map(|j| operator.get(i,j)).collect()).collect()
}