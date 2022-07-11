#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::str::FromStr;
use not_br_lib::not_br;
use not_br_lib::not_br::OutputType;


#[napi]
#[allow(dead_code)]
fn process_text(input: String, frequency: u32, bold_percentage: f64, output_type: String) -> Option<String> {
    let output_type = match OutputType::from_str(output_type.as_str()) {
        Ok(output_type) => { output_type }
        Err(_) => { return None; }
    };
    let bold_percentage =
        if bold_percentage < 0. {
            return None;
        } else if bold_percentage > 1. {
            bold_percentage / 100.
        } else { bold_percentage };

    not_br::process(input.as_str(), frequency as u64, bold_percentage, output_type).map_or(None, |s| Some(s))
}
