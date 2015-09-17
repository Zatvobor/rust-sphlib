#![allow(non_camel_case_types)]

extern crate libc;
use libc::{uint64_t, c_uchar, size_t, c_void};
use std::mem::{zeroed};
use utils::{to_void_raw_ctx, to_void_raw_data, to_void_raw_dest};

#[repr(C)]
#[allow(non_snake_case)]
pub struct WhirlpoolContext {
    pub buf: [c_uchar; 64usize],
    pub state: [uint64_t; 8usize],
    pub count: uint64_t,
}

impl ::std::default::Default for WhirlpoolContext {
    fn default() -> Self { unsafe { zeroed() } }
}

pub type whirlpool = [u8;64];

extern {
    pub fn sph_whirlpool_init(cc: *mut c_void) -> ();
    pub fn sph_whirlpool(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_whirlpool_close(cc: *mut c_void, dst: *mut c_void) -> ();

    pub fn sph_whirlpool0(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_whirlpool0_close(cc: *mut c_void, dst: *mut c_void) -> ();

    pub fn sph_whirlpool1(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_whirlpool1_close(cc: *mut c_void, dst: *mut c_void) -> ();
}


pub fn whirlpool_init(cc: &mut WhirlpoolContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_whirlpool_init(void_raw_cc) };
}

pub fn whirlpool_load(cc: &mut WhirlpoolContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_whirlpool(void_raw_cc, void_raw_data, len) };
}

pub fn whirlpool_close(cc: &mut WhirlpoolContext, dest: &mut whirlpool) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_whirlpool_close(void_raw_cc, void_raw_dest); };
}

pub fn whirlpool_init_load_close(data: &str) -> whirlpool {
    let mut dest: whirlpool = [0;64];
    let mut cc              = WhirlpoolContext::default();

    whirlpool_init(&mut cc);
    whirlpool_load(&mut cc, data);
    whirlpool_close(&mut cc, &mut dest);

    return dest;
}

pub fn whirlpool0_load(cc: &mut WhirlpoolContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_whirlpool0(void_raw_cc, void_raw_data, len) };
}

pub fn whirlpool0_close(cc: &mut WhirlpoolContext, dest: &mut whirlpool) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_whirlpool0_close(void_raw_cc, void_raw_dest); };
}

pub fn whirlpool0_init_load_close(data: &str) -> whirlpool {
    let mut dest: whirlpool = [0;64];
    let mut cc              = WhirlpoolContext::default();

    whirlpool_init(&mut cc);
    whirlpool0_load(&mut cc, data);
    whirlpool0_close(&mut cc, &mut dest);

    return dest;
}

pub fn whirlpool1_load(cc: &mut WhirlpoolContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_whirlpool1(void_raw_cc, void_raw_data, len) };
}

pub fn whirlpool1_close(cc: &mut WhirlpoolContext, dest: &mut whirlpool) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_whirlpool1_close(void_raw_cc, void_raw_dest); };
}

pub fn whirlpool1_init_load_close(data: &str) -> whirlpool {
    let mut dest: whirlpool = [0;64];
    let mut cc              = WhirlpoolContext::default();

    whirlpool_init(&mut cc);
    whirlpool1_load(&mut cc, data);
    whirlpool1_close(&mut cc, &mut dest);

    return dest;
}
