extern crate libc;

use libc::{c_uchar, size_t, c_char, c_void};

use std::str;
use std::ffi::CStr;


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

pub fn to_void_raw_ctx<T>(cc: &mut T) -> *mut c_void {
    let raw_cc      = cc as *mut T;
    let void_raw_cc = raw_cc as *mut c_void;

    return void_raw_cc;
}

pub fn to_void_raw_data(data: &str) -> (*const c_void, size_t) {
    let void_raw_data = data.as_ptr() as *const c_void;
    let len = data.len() as size_t;

    return (void_raw_data, len);
}

pub fn to_void_raw_dest(dest: &mut [u8]) -> *mut c_void {
    let raw_dest = dest.as_mut() as *mut [u8];
    let void_raw_dest = raw_dest as *mut c_void;

    return void_raw_dest;
}
