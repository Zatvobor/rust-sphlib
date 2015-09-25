#![allow(non_camel_case_types)]

extern crate libc;
use libc::{uint32_t, uint64_t, c_uchar, size_t, c_void};
use std::mem::{zeroed};
use utils::{to_void_raw_ctx, to_void_raw_data, to_void_raw_dest};

#[repr(C)]
#[allow(non_snake_case)]
pub struct Md4Context {
    pub buf: [c_uchar; 64usize],
    pub val: [uint32_t; 4usize],
    pub count: uint64_t,
}

impl ::std::default::Default for Md4Context {
    fn default() -> Self { unsafe { zeroed() } }
}

pub type md4 = [u8;16];

extern {
    pub fn sph_md4_init(cc: *mut c_void) -> ();
    pub fn sph_md4(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_md4_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_md4_comp(msg: *mut uint32_t, val: *mut uint32_t) -> ();
}


pub fn md4_init(cc: &mut Md4Context) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_md4_init(void_raw_cc) };
}

pub fn md4_load(cc: &mut Md4Context, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_md4(void_raw_cc, void_raw_data, len) };
}

pub fn md4_close(cc: &mut Md4Context, dest: &mut md4) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_md4_close(void_raw_cc, void_raw_dest); };
}

pub fn md4_init_load_close(data: &str) -> md4 {
    let mut dest: md4 = [0;16];
    let mut cc        = Md4Context::default();

    md4_init(&mut cc);
    md4_load(&mut cc, data);
    md4_close(&mut cc, &mut dest);

    return dest;
}
