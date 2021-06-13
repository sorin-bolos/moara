use wasm_bindgen::prelude::*;
use moara;

#[wasm_bindgen]
pub fn simulate(serialized_circuit:String, shots:u32, qubit_count:Option<u8>) -> Vec<u32> {
    moara::simulate(serialized_circuit, shots, qubit_count)
}

#[wasm_bindgen]
pub fn get_probabilities(serialized_circuit:String, qubit_count:Option<u8>) -> Vec<f32> {
    moara::get_probabilities(serialized_circuit, qubit_count)
}
