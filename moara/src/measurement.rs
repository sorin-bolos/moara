extern crate num_complex;
extern crate rand;

use rand::Rng;
use crate::statevector::Statevector;

pub fn measure(statevector:&Statevector) -> usize
{
    let mut rng = rand::thread_rng();
    let sample:f32 = rng.gen();

    let probabilities = statevector.data.iter()
        .map(|amplitude| amplitude.norm())
        .map(|norm| norm*norm);

    let mut running_sum = 0.0;
    for (index, probability) in probabilities.enumerate()
    {
        running_sum += probability;
        if sample < running_sum
        {
            return index
        }
    }

    panic!("Sample was not in the expected interval");
}