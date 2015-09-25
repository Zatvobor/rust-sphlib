#![allow(non_camel_case_types)]

extern crate libc;
use libc::{uint32_t, uint64_t, c_uchar, size_t, c_void, c_uint};
use std::mem::{zeroed};
use utils::{to_void_raw_ctx, to_void_raw_data, to_void_raw_dest};

#[repr(C)]
#[allow(non_snake_case)]
pub struct Md5Context {
    pub buf: [c_uchar; 64usize],
    pub val: [uint32_t; 4usize],
    pub count: uint64_t,
}

impl ::std::default::Default for Md5Context {
    fn default() -> Self { unsafe { zeroed() } }
}

pub type md5 = [u8;16];

extern {
    pub fn sph_md5_init(cc: *mut c_void) -> ();
    pub fn sph_md5(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_md5_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_md5_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();
    pub fn sph_md5_comp(msg: *mut uint32_t, val: *mut uint32_t) -> ();
}


pub fn md5_init(cc: &mut Md5Context) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_md5_init(void_raw_cc) };
}

pub fn md5_load(cc: &mut Md5Context, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_md5(void_raw_cc, void_raw_data, len) };
}

pub fn md5_close(cc: &mut Md5Context, dest: &mut md5) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_md5_close(void_raw_cc, void_raw_dest); };
}

pub fn md5_init_load_close(data: &str) -> md5 {
    let mut dest: md5 = [0;16];
    let mut cc        = Md5Context::default();

    md5_init(&mut cc);
    md5_load(&mut cc, data);
    md5_close(&mut cc, &mut dest);

    return dest;
}
