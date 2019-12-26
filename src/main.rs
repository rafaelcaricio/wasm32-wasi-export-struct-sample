#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::CStr;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    unsafe {
        let rt = JS_NewRuntime();
        let ctx = JS_NewContext(rt);
        let text_str = "1+1\0";
        let text = CStr::from_bytes_with_nul(text_str.as_bytes()).unwrap();
        let sec = CStr::from_bytes_with_nul("sec\0".as_bytes()).unwrap();

        let val: JSValue = JS_Eval(
            ctx,
            text.as_ptr(),
            text_str.len() - 1,
            sec.as_ptr(),
            JS_EVAL_TYPE_GLOBAL as i32
        );
        println!("val.tag = {}", val.tag);
    }
}
