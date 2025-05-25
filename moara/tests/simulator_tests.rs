#[cfg(test)]
extern crate moara;
extern crate serde_json;

use moara::simulator;

#[test]
fn simple_circuit_works() {
    let serialized = "{
        \"steps\": [
            {
            \"index\": 0,
            \"gates\": [
              {
                \"name\": \"hadamard\",
                \"targets\": [ 0 ]
              }
            ]
          }
        ]
      }";

      let results = simulator::simulate(serialized.to_string(), 1000u32, Some("bigendian".to_string()), Some(1u8));
      assert_eq!(2, results.len());
      assert!(aprox_equals(500, results[0], 0.1));
      assert!(aprox_equals(500, results[1], 0.1));
}

pub fn aprox_equals(a:u32, b:u32, fraction:f32) -> bool
{
    let diff = (b as i32 - a as i32).abs();
    let sum = b+a;

    (diff as f32) <= (sum as f32) * fraction
}