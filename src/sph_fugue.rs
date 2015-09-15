#![allow(non_camel_case_types)]

extern crate libc;
use libc::{uint32_t, uint64_t, size_t, c_void, c_uint};
use std::mem::{zeroed};
use utils::{to_void_raw_ctx, to_void_raw_data, to_void_raw_dest};

#[repr(C)]
#[allow(non_snake_case)]
pub struct FugueContext {
    pub partial: uint32_t,
    pub partial_len: c_uint,
    pub round_shift: c_uint,
    pub S: [uint32_t; 36usize],
    pub bit_count: uint64_t,
}

impl ::std::default::Default for FugueContext {
    fn default() -> Self { unsafe { zeroed() } }
}

pub type fugue224 = [u8;28];
pub type fugue256 = [u8;32];
pub type fugue384 = [u8;48];
pub type fugue512 = [u8;64];

extern {
    pub fn sph_fugue224_init(cc: *mut c_void) -> ();
    pub fn sph_fugue224(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_fugue224_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_fugue224_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_fugue256_init(cc: *mut c_void) -> ();
    pub fn sph_fugue256(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_fugue256_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_fugue256_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_fugue384_init(cc: *mut c_void) -> ();
    pub fn sph_fugue384(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_fugue384_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_fugue384_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_fugue512_init(cc: *mut c_void) -> ();
    pub fn sph_fugue512(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_fugue512_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_fugue512_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();
}


pub fn fugue224_init(cc: &mut FugueContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_fugue224_init(void_raw_cc) };
}

pub fn fugue224_load(cc: &mut FugueContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_fugue224(void_raw_cc, void_raw_data, len) };
}

pub fn fugue224_close(cc: &mut FugueContext, dest: &mut fugue224) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_fugue224_close(void_raw_cc, void_raw_dest); };
}

pub fn fugue224_init_load_close(data: &str) -> fugue224 {
    let mut dest: fugue224 = [0;28];
    let mut cc                = FugueContext::default();

    fugue224_init(&mut cc);
    fugue224_load(&mut cc, data);
    fugue224_close(&mut cc, &mut dest);

    return dest;
}

pub fn fugue256_init(cc: &mut FugueContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_fugue256_init(void_raw_cc) };
}

pub fn fugue256_load(cc: &mut FugueContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_fugue256(void_raw_cc, void_raw_data, len) };
}

pub fn fugue256_close(cc: &mut FugueContext, dest: &mut fugue256) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_fugue256_close(void_raw_cc, void_raw_dest); };
}

pub fn fugue256_init_load_close(data: &str) -> fugue256 {
    let mut dest: fugue256 = [0;32];
    let mut cc                = FugueContext::default();

    fugue256_init(&mut cc);
    fugue256_load(&mut cc, data);
    fugue256_close(&mut cc, &mut dest);

    return dest;
}

pub fn fugue384_init(cc: &mut FugueContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_fugue384_init(void_raw_cc) };
}

pub fn fugue384_load(cc: &mut FugueContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_fugue384(void_raw_cc, void_raw_data, len) };
}

pub fn fugue384_close(cc: &mut FugueContext, dest: &mut fugue384) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_fugue384_close(void_raw_cc, void_raw_dest); };
}

pub fn fugue384_init_load_close(data: &str) -> fugue384 {
    let mut dest: fugue384 = [0;48];
    let mut cc                = FugueContext::default();

    fugue384_init(&mut cc);
    fugue384_load(&mut cc, data);
    fugue384_close(&mut cc, &mut dest);

    return dest;
}

pub fn fugue512_init(cc: &mut FugueContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_fugue512_init(void_raw_cc) };
}

pub fn fugue512_load(cc: &mut FugueContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_fugue512(void_raw_cc, void_raw_data, len) };
}

pub fn fugue512_close(cc: &mut FugueContext, dest: &mut fugue512) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_fugue512_close(void_raw_cc, void_raw_dest); };
}

pub fn fugue512_init_load_close(data: &str) -> fugue512 {
    let mut dest: fugue512 = [0;64];
    let mut cc                = FugueContext::default();

    fugue512_init(&mut cc);
    fugue512_load(&mut cc, data);
    fugue512_close(&mut cc, &mut dest);

    return dest;
}
