extern crate num_complex;
extern crate rand;
mod tools;

use num_complex::Complex32;
use rand::Rng;

pub fn measure(statevector:[Complex32; 2]) -> u8
{
    let norm_0:f32 = statevector[0].norm();
    let norm_1:f32 = statevector[1].norm();
    let prob_0:f32 = norm_0 * norm_0;
    let prob_1:f32 = norm_1 * norm_1;

    if !tools::equals(prob_0 + prob_1, 1.0)
    {
        panic!("input is not a valid statevector");
    }

    let mut rng = rand::thread_rng();
    let sample:f32 = rng.gen();

    match sample {
        _ if (sample < prob_0) => 0,
        _ => 1
    }
}