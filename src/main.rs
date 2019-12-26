#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    use std::ffi::CStr;
    let s = CStr::from_bytes_with_nul("script\0".as_bytes()).unwrap();
    let _ = unsafe { X_Run(s.as_ptr(), 0) };
    println!("Hello from Rust code too!");
}
