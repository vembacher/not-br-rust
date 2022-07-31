use std::str::FromStr;
use wasm_bindgen::prelude::*;
use not_br_lib::not_br;


#[wasm_bindgen]
pub fn process(input: &str, frequency: u32, bold_percentage: f64, output_type: &str) -> Result<String, JsValue> {
    let output_type = match not_br::OutputType::from_str(output_type) {
        Ok(output_type) => { output_type }
        Err(_) => { return Err(JsValue::from_str("Invalid output type.")); }
    };
    not_br::process(input, frequency as u64, bold_percentage, output_type)
        .map_or(Err(JsValue::from_str("Internal notbr errror")), |r| Ok(r))
}
