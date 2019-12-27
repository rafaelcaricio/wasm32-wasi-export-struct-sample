#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

mod bindings;

use std::ffi::CStr;
use bindings::*;

fn main() {
    unsafe {
        let ctx = X_NewContext();
        let text_str = "Rafael\0";
        let text = CStr::from_bytes_with_nul(text_str.as_bytes()).unwrap();
        let sec = CStr::from_bytes_with_nul("sec\0".as_bytes()).unwrap();


        let val: XValue = X_Run(
            ctx,
            text.as_ptr(),
            text_str.len() - 1,
            sec.as_ptr(),
            0
        );

        let _ = JS_IsNumber(&val as *const XValue);

        println!("val.tag = {}", val.tag);
    }
}
