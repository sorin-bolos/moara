use js_sys::Array;
use wasm_bindgen::prelude::*;
use moara;

#[wasm_bindgen]
pub fn simulate(serialized_circuit:String, shots:u32, endianess:Option<String>, qubit_count:Option<u8>) -> Vec<u32> {
    moara::simulate(serialized_circuit, shots, endianess, qubit_count)
}

#[wasm_bindgen]
pub fn get_probabilities(serialized_circuit:String, endianess:Option<String>, qubit_count:Option<u8>) -> Vec<f32> {
    moara::get_probabilities(serialized_circuit, endianess, qubit_count)
}

#[wasm_bindgen]
pub fn get_statevector(serialized_circuit:String, endianess:Option<String>, qubit_count:Option<u8>) -> Array {
    moara::get_statevector(serialized_circuit, endianess, qubit_count).into_iter().map(|i| JsValue::from_str(&i.to_string())).collect()
}