#![allow(non_camel_case_types)]

extern crate libc;
use libc::{uint32_t, c_uchar, size_t, c_void, c_uint};
use std::mem::{zeroed};
use utils::{to_void_raw_ctx, to_void_raw_data, to_void_raw_dest};

#[repr(C)]
#[allow(non_snake_case)]
pub struct SimdSmallContext {
    pub buf: [c_uchar; 64usize],
    pub ptr: size_t,
    pub state: [uint32_t; 16usize],
    pub count_low: uint32_t,
    pub count_high: uint32_t,
}

impl ::std::default::Default for SimdSmallContext {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct SimdBigContext {
    pub buf: [c_uchar; 128usize],
    pub ptr: size_t,
    pub state: [uint32_t; 32usize],
    pub count_low: uint32_t,
    pub count_high: uint32_t,
}

impl ::std::default::Default for SimdBigContext {
    fn default() -> Self { unsafe { zeroed() } }
}

pub type simd224 = [u8;28];
pub type simd256 = [u8;32];
pub type simd384 = [u8;48];
pub type simd512 = [u8;64];

extern  {
    pub fn sph_simd224_init(cc: *mut c_void) -> ();
    pub fn sph_simd224(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_simd224_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_simd224_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_simd256_init(cc: *mut c_void) -> ();
    pub fn sph_simd256(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_simd256_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_simd256_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_simd384_init(cc: *mut c_void) -> ();
    pub fn sph_simd384(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_simd384_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_simd384_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_simd512_init(cc: *mut c_void) -> ();
    pub fn sph_simd512(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_simd512_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_simd512_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();
}


pub fn simd224_init(cc: &mut SimdSmallContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_simd224_init(void_raw_cc) };
}

pub fn simd224_load(cc: &mut SimdSmallContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_simd224(void_raw_cc, void_raw_data, len) };
}

pub fn simd224_close(cc: &mut SimdSmallContext, dest: &mut simd224) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_simd224_close(void_raw_cc, void_raw_dest); };
}

pub fn simd224_init_load_close(data: &str) -> simd224 {
    let mut dest: simd224 = [0;28];
    let mut cc            = SimdSmallContext::default();

    simd224_init(&mut cc);
    simd224_load(&mut cc, data);
    simd224_close(&mut cc, &mut dest);

    return dest;
}

pub fn simd256_init(cc: &mut SimdSmallContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_simd256_init(void_raw_cc) };
}

pub fn simd256_load(cc: &mut SimdSmallContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_simd256(void_raw_cc, void_raw_data, len) };
}

pub fn simd256_close(cc: &mut SimdSmallContext, dest: &mut simd256) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_simd256_close(void_raw_cc, void_raw_dest); };
}

pub fn simd256_init_load_close(data: &str) -> simd256 {
    let mut dest: simd256 = [0;32];
    let mut cc            = SimdSmallContext::default();

    simd256_init(&mut cc);
    simd256_load(&mut cc, data);
    simd256_close(&mut cc, &mut dest);

    return dest;
}

pub fn simd384_init(cc: &mut SimdBigContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_simd384_init(void_raw_cc) };
}

pub fn simd384_load(cc: &mut SimdBigContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_simd384(void_raw_cc, void_raw_data, len) };
}

pub fn simd384_close(cc: &mut SimdBigContext, dest: &mut simd384) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_simd384_close(void_raw_cc, void_raw_dest); };
}

pub fn simd384_init_load_close(data: &str) -> simd384 {
    let mut dest: simd384 = [0;48];
    let mut cc            = SimdBigContext::default();

    simd384_init(&mut cc);
    simd384_load(&mut cc, data);
    simd384_close(&mut cc, &mut dest);

    return dest;
}

pub fn simd512_init(cc: &mut SimdBigContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_simd512_init(void_raw_cc) };
}

pub fn simd512_load(cc: &mut SimdBigContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_simd512(void_raw_cc, void_raw_data, len) };
}

pub fn simd512_close(cc: &mut SimdBigContext, dest: &mut simd512) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_simd512_close(void_raw_cc, void_raw_dest); };
}

pub fn simd512_init_load_close(data: &str) -> simd512 {
    let mut dest: simd512 = [0;64];
    let mut cc            = SimdBigContext::default();

    simd512_init(&mut cc);
    simd512_load(&mut cc, data);
    simd512_close(&mut cc, &mut dest);

    return dest;
}
