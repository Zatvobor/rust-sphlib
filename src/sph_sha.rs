#![allow(non_camel_case_types)]

extern crate libc;
use libc::{uint32_t, uint64_t, c_uchar, size_t, c_void, c_uint};
use std::mem::{zeroed};
use utils::{to_void_raw_ctx, to_void_raw_data, to_void_raw_dest};

#[repr(C)]
#[allow(non_snake_case)]
pub struct ShaSmallContext {
    pub buf: [c_uchar; 64usize],
    pub val: [uint32_t; 5usize],
    pub count: uint64_t,
}

impl ::std::default::Default for ShaSmallContext {
    fn default() -> Self { unsafe { zeroed() } }
}

pub type sha0 = [u8;20];
pub type sha1 = [u8;20];

extern {
    pub fn sph_sha0_init(cc: *mut c_void) -> ();
    pub fn sph_sha0(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_sha0_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_sha0_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();
    pub fn sph_sha0_comp(msg: *mut uint32_t, val: *mut uint32_t) -> ();

    pub fn sph_sha1_init(cc: *mut c_void) -> ();
    pub fn sph_sha1(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_sha1_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_sha1_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();
    pub fn sph_sha1_comp(msg: *mut uint32_t, val: *mut uint32_t) -> ();
}


pub fn sha0_init(cc: &mut ShaSmallContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_sha0_init(void_raw_cc) };
}

pub fn sha0_load(cc: &mut ShaSmallContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_sha0(void_raw_cc, void_raw_data, len) };
}

pub fn sha0_close(cc: &mut ShaSmallContext, dest: &mut sha0) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_sha0_close(void_raw_cc, void_raw_dest); };
}

pub fn sha0_init_load_close(data: &str) -> sha0 {
    let mut dest: sha0 = [0;20];
    let mut cc        = ShaSmallContext::default();

    sha0_init(&mut cc);
    sha0_load(&mut cc, data);
    sha0_close(&mut cc, &mut dest);

    return dest;
}

pub fn sha1_init(cc: &mut ShaSmallContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_sha1_init(void_raw_cc) };
}

pub fn sha1_load(cc: &mut ShaSmallContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_sha1(void_raw_cc, void_raw_data, len) };
}

pub fn sha1_close(cc: &mut ShaSmallContext, dest: &mut sha1) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_sha1_close(void_raw_cc, void_raw_dest); };
}

pub fn sha1_init_load_close(data: &str) -> sha1 {
    let mut dest: sha1 = [0;20];
    let mut cc        = ShaSmallContext::default();

    sha1_init(&mut cc);
    sha1_load(&mut cc, data);
    sha1_close(&mut cc, &mut dest);

    return dest;
}
