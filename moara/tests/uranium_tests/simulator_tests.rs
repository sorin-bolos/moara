#[cfg(test)]
extern crate moara;
extern crate serde_json;

use moara::uranium::circuit::Circuit;
use moara::uranium::simulator::run;

#[test]
fn simple_circuit_works() {
    let serialized = "{
        \"steps\": [
            {
            \"index\": 0,
            \"gates\": [
              {
                \"name\": \"hadamard\",
                \"target\": 0
              }
            ]
          }
        ]
      }";

      let circuit: Circuit = serde_json::from_str(&serialized).unwrap();

      let results = run(1u8, circuit, 1000);
      assert_eq!(2, results.len());
      assert!(aprox_equals(500, results[0], 0.01));
      assert!(aprox_equals(500, results[1], 0.01));
}

pub fn aprox_equals(a:u32, b:u32, fraction:f32) -> bool
{
    let diff = (b as i32 - a as i32).abs();
    let sum = b+a;

    (diff as f32) <= (sum as f32) * fraction
}