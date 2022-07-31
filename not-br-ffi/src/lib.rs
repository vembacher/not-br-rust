extern crate core;

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use not_br_lib::not_br;
use not_br_lib::not_br::NotBrProcess;

#[repr(C)]
pub enum OutputType {
    NotBrHtml = 1,
    NotBrMarkdown = 2,
}

impl OutputType {
    fn get_internal_enum(&self) -> not_br::OutputType {
        match self {
            OutputType::NotBrHtml => not_br::OutputType::HTML,
            OutputType::NotBrMarkdown => not_br::OutputType::Markdown,
        }
    }
}


#[no_mangle]
pub extern "C" fn process_text(input: *const c_char, frequency: c_int, bold_percentage: c_int, output_type: OutputType) -> *mut c_char {
    unsafe { CStr::from_ptr(input) }
        .process_text(frequency as u64, (bold_percentage as f64) / 100_f64, output_type.get_internal_enum())
        .map_or(std::ptr::null_mut(), |s| s.into_raw())
}

#[no_mangle]
pub extern "C" fn not_br_free(output: *mut c_char) {
    unsafe { drop(CString::from_raw(output)) };
}

