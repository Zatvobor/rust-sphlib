#![allow(non_camel_case_types)]

extern crate libc;
use libc::{uint32_t, c_uchar, size_t, c_void, c_uint};
use utils::{to_void_raw_ctx, to_void_raw_data, to_void_raw_dest};

#[repr(C)]
#[allow(non_snake_case)]
pub struct CubeHashContext {
    pub buf: [c_uchar; 32usize],
    pub ptr: size_t,
    pub state: [uint32_t; 32usize],
}

impl ::std::default::Default for CubeHashContext {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

pub type cubehash224 = [u8;28];
pub type cubehash256 = [u8;32];
pub type cubehash384 = [u8;48];
pub type cubehash512 = [u8;64];

extern {
    pub fn sph_cubehash224_init(cc: *mut c_void) -> ();
    pub fn sph_cubehash224(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_cubehash224_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_cubehash224_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_cubehash256_init(cc: *mut c_void) -> ();
    pub fn sph_cubehash256(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_cubehash256_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_cubehash256_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_cubehash384_init(cc: *mut c_void) -> ();
    pub fn sph_cubehash384(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_cubehash384_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_cubehash384_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_cubehash512_init(cc: *mut c_void) -> ();
    pub fn sph_cubehash512(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_cubehash512_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_cubehash512_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();
}


pub fn cubehash224_init(cc: &mut CubeHashContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_cubehash224_init(void_raw_cc) };
}

pub fn cubehash224_load(cc: &mut CubeHashContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_cubehash224(void_raw_cc, void_raw_data, len) };
}

pub fn cubehash224_close(cc: &mut CubeHashContext, dest: &mut cubehash224) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_cubehash224_close(void_raw_cc, void_raw_dest); };
}

pub fn cubehash224_init_load_close(data: &str) -> cubehash224 {
    let mut dest: cubehash224 = [0;28];
    let mut cc                = CubeHashContext::default();

    cubehash224_init(&mut cc);
    cubehash224_load(&mut cc, data);
    cubehash224_close(&mut cc, &mut dest);

    return dest;
}

pub fn cubehash256_init(cc: &mut CubeHashContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_cubehash256_init(void_raw_cc) };
}

pub fn cubehash256_load(cc: &mut CubeHashContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_cubehash256(void_raw_cc, void_raw_data, len) };
}

pub fn cubehash256_close(cc: &mut CubeHashContext, dest: &mut cubehash256) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_cubehash256_close(void_raw_cc, void_raw_dest); };
}

pub fn cubehash256_init_load_close(data: &str) -> cubehash256 {
    let mut dest: cubehash256 = [0;32];
    let mut cc                = CubeHashContext::default();

    cubehash256_init(&mut cc);
    cubehash256_load(&mut cc, data);
    cubehash256_close(&mut cc, &mut dest);

    return dest;
}

pub fn cubehash384_init(cc: &mut CubeHashContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_cubehash384_init(void_raw_cc) };
}

pub fn cubehash384_load(cc: &mut CubeHashContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_cubehash384(void_raw_cc, void_raw_data, len) };
}

pub fn cubehash384_close(cc: &mut CubeHashContext, dest: &mut cubehash384) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_cubehash384_close(void_raw_cc, void_raw_dest); };
}

pub fn cubehash384_init_load_close(data: &str) -> cubehash384 {
    let mut dest: cubehash384 = [0;48];
    let mut cc                = CubeHashContext::default();

    cubehash384_init(&mut cc);
    cubehash384_load(&mut cc, data);
    cubehash384_close(&mut cc, &mut dest);

    return dest;
}

pub fn cubehash512_init(cc: &mut CubeHashContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_cubehash512_init(void_raw_cc) };
}

pub fn cubehash512_load(cc: &mut CubeHashContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_cubehash512(void_raw_cc, void_raw_data, len) };
}

pub fn cubehash512_close(cc: &mut CubeHashContext, dest: &mut cubehash512) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_cubehash512_close(void_raw_cc, void_raw_dest); };
}

pub fn cubehash512_init_load_close(data: &str) -> cubehash512 {
    let mut dest: cubehash512 = [0;64];
    let mut cc                = CubeHashContext::default();

    cubehash512_init(&mut cc);
    cubehash512_load(&mut cc, data);
    cubehash512_close(&mut cc, &mut dest);

    return dest;
}
