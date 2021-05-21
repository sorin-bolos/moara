use wasm_bindgen::prelude::*;
use moara;

#[wasm_bindgen]
pub fn simulate(serialized_circuit:String, shots:u32, qubit_count:Option<u8>) -> Vec<u32> {
    moara::simulate(serialized_circuit, shots, qubit_count)
}
