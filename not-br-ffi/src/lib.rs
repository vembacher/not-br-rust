#![feature(core_ffi_c)]

use core::ffi::{c_int};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char};
use not_br_lib::not_br;
use not_br_lib::not_br::NotBrProcess;

#[repr(C)]
pub enum InputType {
    HTML = 1,
    Markdown = 2,
}

impl InputType {
    fn get_internal_enum(&self) -> not_br::InputType {
        match self {
            InputType::HTML => not_br::InputType::HTML,
            InputType::Markdown => not_br::InputType::Markdown,
        }
    }
    fn from(i: c_int) -> InputType {
        match i {
            1 => InputType::HTML,
            2 => InputType::Markdown,
            _ => panic!("Error.")
        }
    }
}


#[no_mangle]
pub extern "C" fn process_text(input: *const c_char, frequency: c_int, bold_percentage: c_int, input_type: c_int) -> *mut c_char {
    unsafe { CStr::from_ptr(input) }
        .process_text(frequency as u64, (bold_percentage as f64) / 100_f64, InputType::from(input_type).get_internal_enum())
        .map_or(std::ptr::null_mut(), |s| s.into_raw())
}

#[no_mangle]
pub extern "C" fn not_br_free(output: *mut c_char) {
    unsafe { drop(CString::from_raw(output)) };
}

