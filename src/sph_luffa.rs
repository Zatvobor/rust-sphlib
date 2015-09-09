#![allow(non_camel_case_types)]

extern crate libc;
use libc::{uint32_t, c_uchar, size_t, c_void, c_uint};
use std::mem::{zeroed};
use utils::{to_void_raw_ctx, to_void_raw_data, to_void_raw_dest};

#[repr(C)]
#[allow(non_snake_case)]
pub struct LuffaSmallContext {
    pub buf: [c_uchar; 32usize],
    pub ptr: size_t,
    pub V: [[uint32_t; 8usize]; 3usize],
}

impl ::std::default::Default for LuffaSmallContext {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct LuffaMiddleContext {
    pub buf: [c_uchar; 32usize],
    pub ptr: size_t,
    pub V: [[uint32_t; 8usize]; 4usize],
}

impl ::std::default::Default for LuffaMiddleContext {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct LuffaBigContext {
    pub buf: [c_uchar; 32usize],
    pub ptr: size_t,
    pub V: [[uint32_t; 8usize]; 5usize],
}

impl ::std::default::Default for LuffaBigContext {
    fn default() -> Self { unsafe { zeroed() } }
}

pub type luffa224 = [u8;28];
pub type luffa256 = [u8;32];
pub type luffa384 = [u8;48];
pub type luffa512 = [u8;64];

extern {
    pub fn sph_luffa224_init(cc: *mut c_void) -> ();
    pub fn sph_luffa224(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_luffa224_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_luffa224_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_luffa256_init(cc: *mut c_void) -> ();
    pub fn sph_luffa256(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_luffa256_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_luffa256_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_luffa384_init(cc: *mut c_void) -> ();
    pub fn sph_luffa384(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_luffa384_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_luffa384_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_luffa512_init(cc: *mut c_void) -> ();
    pub fn sph_luffa512(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_luffa512_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_luffa512_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();
}


pub fn luffa224_init(cc: &mut LuffaSmallContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_luffa224_init(void_raw_cc) };
}

pub fn luffa224_load(cc: &mut LuffaSmallContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_luffa224(void_raw_cc, void_raw_data, len) };
}

pub fn luffa224_close(cc: &mut LuffaSmallContext, dest: &mut luffa224) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_luffa224_close(void_raw_cc, void_raw_dest); };
}

pub fn luffa224_init_load_close(data: &str) -> luffa224 {
    let mut dest: luffa224 = [0;28];
    let mut cc            = LuffaSmallContext::default();

    luffa224_init(&mut cc);
    luffa224_load(&mut cc, data);
    luffa224_close(&mut cc, &mut dest);

    return dest;
}

pub fn luffa256_init(cc: &mut LuffaSmallContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_luffa256_init(void_raw_cc) };
}

pub fn luffa256_load(cc: &mut LuffaSmallContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_luffa256(void_raw_cc, void_raw_data, len) };
}

pub fn luffa256_close(cc: &mut LuffaSmallContext, dest: &mut luffa256) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_luffa256_close(void_raw_cc, void_raw_dest); };
}

pub fn luffa256_init_load_close(data: &str) -> luffa256 {
    let mut dest: luffa256 = [0;32];
    let mut cc            = LuffaSmallContext::default();

    luffa256_init(&mut cc);
    luffa256_load(&mut cc, data);
    luffa256_close(&mut cc, &mut dest);

    return dest;
}

pub fn luffa384_init(cc: &mut LuffaMiddleContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_luffa384_init(void_raw_cc) };
}

pub fn luffa384_load(cc: &mut LuffaMiddleContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_luffa384(void_raw_cc, void_raw_data, len) };
}

pub fn luffa384_close(cc: &mut LuffaMiddleContext, dest: &mut luffa384) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_luffa384_close(void_raw_cc, void_raw_dest); };
}

pub fn luffa384_init_load_close(data: &str) -> luffa384 {
    let mut dest: luffa384 = [0;48];
    let mut cc            = LuffaMiddleContext::default();

    luffa384_init(&mut cc);
    luffa384_load(&mut cc, data);
    luffa384_close(&mut cc, &mut dest);

    return dest;
}

pub fn luffa512_init(cc: &mut LuffaBigContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_luffa512_init(void_raw_cc) };
}

pub fn luffa512_load(cc: &mut LuffaBigContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_luffa512(void_raw_cc, void_raw_data, len) };
}

pub fn luffa512_close(cc: &mut LuffaBigContext, dest: &mut luffa512) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_luffa512_close(void_raw_cc, void_raw_dest); };
}

pub fn luffa512_init_load_close(data: &str) -> luffa512 {
    let mut dest: luffa512 = [0;64];
    let mut cc            = LuffaBigContext::default();

    luffa512_init(&mut cc);
    luffa512_load(&mut cc, data);
    luffa512_close(&mut cc, &mut dest);

    return dest;
}
