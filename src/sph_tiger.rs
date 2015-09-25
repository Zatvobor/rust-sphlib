#![allow(non_camel_case_types)]

extern crate libc;
use libc::{uint64_t, c_uchar, size_t, c_void};
use std::mem::{zeroed};
use utils::{to_void_raw_ctx, to_void_raw_data, to_void_raw_dest};

#[repr(C)]
#[allow(non_snake_case)]
pub struct TigerContext {
    pub buf: [c_uchar; 64usize],
    pub val: [uint64_t; 3usize],
    pub count: uint64_t,
}

impl ::std::default::Default for TigerContext {
    fn default() -> Self { unsafe { zeroed() } }
}

pub type tiger = [u8;24];

extern {
    pub fn sph_tiger_init(cc: *mut c_void) -> ();
    pub fn sph_tiger(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_tiger_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_tiger_comp(msg: *mut uint64_t, val: *mut uint64_t) -> ();

    pub fn sph_tiger2_init(cc: *mut c_void) -> ();
    pub fn sph_tiger2(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_tiger2_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_tiger2_comp(msg: *mut uint64_t, val: *mut uint64_t) -> ();
}


pub fn tiger_init(cc: &mut TigerContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_tiger_init(void_raw_cc) };
}

pub fn tiger_load(cc: &mut TigerContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_tiger(void_raw_cc, void_raw_data, len) };
}

pub fn tiger_close(cc: &mut TigerContext, dest: &mut tiger) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_tiger_close(void_raw_cc, void_raw_dest); };
}

pub fn tiger_init_load_close(data: &str) -> tiger {
    let mut dest: tiger = [0;24];
    let mut cc          = TigerContext::default();

    tiger_init(&mut cc);
    tiger_load(&mut cc, data);
    tiger_close(&mut cc, &mut dest);

    return dest;
}

pub fn tiger2_init(cc: &mut TigerContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_tiger2_init(void_raw_cc) };
}

pub fn tiger2_load(cc: &mut TigerContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_tiger2(void_raw_cc, void_raw_data, len) };
}

pub fn tiger2_close(cc: &mut TigerContext, dest: &mut tiger) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_tiger2_close(void_raw_cc, void_raw_dest); };
}

pub fn tiger2_init_load_close(data: &str) -> tiger {
    let mut dest: tiger = [0;24];
    let mut cc          = TigerContext::default();

    tiger2_init(&mut cc);
    tiger2_load(&mut cc, data);
    tiger2_close(&mut cc, &mut dest);

    return dest;
}
