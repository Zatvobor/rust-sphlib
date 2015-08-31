extern crate libc;

use libc::{c_uchar, size_t, c_char};

use std::str;
use std::ffi::CStr;


#[link(name="utils", kind="static")]
extern {
    pub fn bin2hex(p: *const c_uchar, len: size_t) -> *mut c_char;
}


pub fn to_hex_hash(dst: &[u8]) -> &str {
    let hash;
    unsafe {
        let p           = dst.as_ptr() as *const c_uchar;
        let len         = dst.len() as size_t;
        let buffer_ptr  = bin2hex(p, len);
        hash            = str::from_utf8(CStr::from_ptr(buffer_ptr).to_bytes()).unwrap();
    }
    return hash;
}
