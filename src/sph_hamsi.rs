#![allow(non_camel_case_types)]

extern crate libc;
use libc::{uint32_t, uint64_t, c_uchar, size_t, c_void, c_uint};
use std::mem::{zeroed};
use utils::{to_void_raw_ctx, to_void_raw_data, to_void_raw_dest};

#[repr(C)]
#[allow(non_snake_case)]
pub struct HamsiSmallContext {
    pub partial: [c_uchar; 4usize],
    pub partial_len: size_t,
    pub h: [uint32_t; 8usize],
    pub count: uint64_t,
}

impl ::std::default::Default for HamsiSmallContext {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct HamsiBigContext {
    pub partial: [c_uchar; 8usize],
    pub partial_len: size_t,
    pub h: [uint32_t; 16usize],
    pub count: uint64_t,
}
impl ::std::default::Default for HamsiBigContext {
    fn default() -> Self { unsafe { zeroed() } }
}

pub type hamsi224 = [u8;28];
pub type hamsi256 = [u8;32];
pub type hamsi384 = [u8;48];
pub type hamsi512 = [u8;64];

extern  {
    pub fn sph_hamsi224_init(cc: *mut c_void) -> ();
    pub fn sph_hamsi224(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_hamsi224_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_hamsi224_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_hamsi256_init(cc: *mut c_void) -> ();
    pub fn sph_hamsi256(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_hamsi256_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_hamsi256_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_hamsi384_init(cc: *mut c_void) -> ();
    pub fn sph_hamsi384(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_hamsi384_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_hamsi384_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_hamsi512_init(cc: *mut c_void) -> ();
    pub fn sph_hamsi512(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_hamsi512_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_hamsi512_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();
}


pub fn hamsi224_init(cc: &mut HamsiSmallContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_hamsi224_init(void_raw_cc) };
}

pub fn hamsi224_load(cc: &mut HamsiSmallContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_hamsi224(void_raw_cc, void_raw_data, len) };
}

pub fn hamsi224_close(cc: &mut HamsiSmallContext, dest: &mut hamsi224) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_hamsi224_close(void_raw_cc, void_raw_dest); };
}

pub fn hamsi224_init_load_close(data: &str) -> hamsi224 {
    let mut dest: hamsi224 = [0;28];
    let mut cc             = HamsiSmallContext::default();

    hamsi224_init(&mut cc);
    hamsi224_load(&mut cc, data);
    hamsi224_close(&mut cc, &mut dest);

    return dest;
}

pub fn hamsi256_init(cc: &mut HamsiSmallContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_hamsi256_init(void_raw_cc) };
}

pub fn hamsi256_load(cc: &mut HamsiSmallContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_hamsi256(void_raw_cc, void_raw_data, len) };
}

pub fn hamsi256_close(cc: &mut HamsiSmallContext, dest: &mut hamsi256) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_hamsi256_close(void_raw_cc, void_raw_dest); };
}

pub fn hamsi256_init_load_close(data: &str) -> hamsi256 {
    let mut dest: hamsi256 = [0;32];
    let mut cc             = HamsiSmallContext::default();

    hamsi256_init(&mut cc);
    hamsi256_load(&mut cc, data);
    hamsi256_close(&mut cc, &mut dest);

    return dest;
}

pub fn hamsi384_init(cc: &mut HamsiBigContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_hamsi384_init(void_raw_cc) };
}

pub fn hamsi384_load(cc: &mut HamsiBigContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_hamsi384(void_raw_cc, void_raw_data, len) };
}

pub fn hamsi384_close(cc: &mut HamsiBigContext, dest: &mut hamsi384) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_hamsi384_close(void_raw_cc, void_raw_dest); };
}

pub fn hamsi384_init_load_close(data: &str) -> hamsi384 {
    let mut dest: hamsi384 = [0;48];
    let mut cc             = HamsiBigContext::default();

    hamsi384_init(&mut cc);
    hamsi384_load(&mut cc, data);
    hamsi384_close(&mut cc, &mut dest);

    return dest;
}

pub fn hamsi512_init(cc: &mut HamsiBigContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_hamsi512_init(void_raw_cc) };
}

pub fn hamsi512_load(cc: &mut HamsiBigContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_hamsi512(void_raw_cc, void_raw_data, len) };
}

pub fn hamsi512_close(cc: &mut HamsiBigContext, dest: &mut hamsi512) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_hamsi512_close(void_raw_cc, void_raw_dest); };
}

pub fn hamsi512_init_load_close(data: &str) -> hamsi512 {
    let mut dest: hamsi512 = [0;64];
    let mut cc             = HamsiBigContext::default();

    hamsi512_init(&mut cc);
    hamsi512_load(&mut cc, data);
    hamsi512_close(&mut cc, &mut dest);

    return dest;
}
