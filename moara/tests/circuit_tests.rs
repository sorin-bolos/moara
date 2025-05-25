#[cfg(test)]
extern crate moara;
extern crate serde_json;

use moara::circuit::Circuit;

#[test]
fn deserialize_circuit_works() {
    let serialized = "{
        \"steps\": [
          {
            \"index\": 1,
            \"gates\": [
              {
                \"name\": \"pauli-y\",
                \"targets\": [ 0 ]
              }
            ]
          },
          {
            \"index\": 0,
            \"gates\": [
              {
                \"name\": \"pauli-x\",
                \"targets\": [ 0 ]
              },
              {
                \"name\": \"pauli-z\",
                \"targets\": [ 1 ]
              },
              {
                \"name\": \"t\",
                \"targets\": [ 3 ]
              }
            ]
          }
        ]
      }";

      let deserialized: Circuit = serde_json::from_str(&serialized).unwrap();

      assert_eq!(2, deserialized.steps.len());

      assert_eq!(1, deserialized.steps[0].index);
      assert_eq!(1, deserialized.steps[0].gates.len());
      assert_eq!(0, deserialized.steps[0].gates[0].targets[0]);
      assert_eq!("pauli-y", deserialized.steps[0].gates[0].name);

      assert_eq!(0, deserialized.steps[1].index);
      assert_eq!(3, deserialized.steps[1].gates.len());
      assert_eq!(0, deserialized.steps[1].gates[0].targets[0]);
      assert_eq!("pauli-x", deserialized.steps[1].gates[0].name);
      assert_eq!(1, deserialized.steps[1].gates[1].targets[0]);
      assert_eq!("pauli-z", deserialized.steps[1].gates[1].name);
      assert_eq!(3, deserialized.steps[1].gates[2].targets[0]);
      assert_eq!("t", deserialized.steps[1].gates[2].name);
}