#![allow(non_camel_case_types)]

extern crate libc;
use libc::{uint32_t, c_uchar, size_t, c_void, c_uint};
use std::mem::{zeroed};
use utils::{to_void_raw_ctx, to_void_raw_data, to_void_raw_dest};

#[repr(C)]
#[allow(non_snake_case)]
pub struct PanamaContext {
    pub data: [c_uchar; 32usize],
    pub data_ptr: c_uint,
    pub buffer: [[uint32_t; 8usize]; 32usize],
    pub buffer_ptr: c_uint,
    pub state: [uint32_t; 17usize],
}

impl ::std::default::Default for PanamaContext {
    fn default() -> Self { unsafe { zeroed() } }
}

pub type panama = [u8;32];

extern {
    pub fn sph_panama_init(cc: *mut c_void) -> ();
    pub fn sph_panama(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_panama_close(cc: *mut c_void, dst: *mut c_void) -> ();
}


pub fn panama_init(cc: &mut PanamaContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_panama_init(void_raw_cc) };
}

pub fn panama_load(cc: &mut PanamaContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_panama(void_raw_cc, void_raw_data, len) };
}

pub fn panama_close(cc: &mut PanamaContext, dest: &mut panama) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_panama_close(void_raw_cc, void_raw_dest); };
}

pub fn panama_init_load_close(data: &str) -> panama {
    let mut dest: panama = [0;32];
    let mut cc             = PanamaContext::default();

    panama_init(&mut cc);
    panama_load(&mut cc, data);
    panama_close(&mut cc, &mut dest);

    return dest;
}
