use js_sys::Array;
use wasm_bindgen::prelude::*;
use moara;

#[wasm_bindgen]
pub fn simulate(seralized_circuit_states:String, current_circuit_id:i32, shots:u32, endianess:Option<String>, qubit_count:Option<u8>) -> Vec<u32> {
    moara::simulate(seralized_circuit_states, current_circuit_id, shots, endianess, qubit_count)
}

#[wasm_bindgen]
pub fn get_probabilities(seralized_circuit_states:String, current_circuit_id:i32, endianess:Option<String>, qubit_count:Option<u8>) -> Vec<f32> {
    moara::get_probabilities(seralized_circuit_states, current_circuit_id,  endianess, qubit_count)
}

#[wasm_bindgen]
pub fn get_statevector(seralized_circuit_states:String, current_circuit_id:i32, endianess:Option<String>, qubit_count:Option<u8>) -> Array {
    moara::get_statevector(seralized_circuit_states, current_circuit_id, endianess, qubit_count).into_iter().map(|i| JsValue::from_str(&i.to_string())).collect()
}