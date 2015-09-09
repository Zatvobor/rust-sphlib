#![allow(non_camel_case_types)]

extern crate libc;
use libc::{uint64_t, c_uchar, size_t, c_void, c_uint};
use std::mem::{zeroed};
use utils::{to_void_raw_ctx, to_void_raw_data, to_void_raw_dest};

#[repr(C)]
#[allow(non_snake_case)]
pub struct SkeinContext {
    pub buf: [c_uchar; 64usize],
    pub ptr: size_t,
    pub h0: uint64_t,
    pub h1: uint64_t,
    pub h2: uint64_t,
    pub h3: uint64_t,
    pub h4: uint64_t,
    pub h5: uint64_t,
    pub h6: uint64_t,
    pub h7: uint64_t,
    pub bcount: uint64_t,
}

impl ::std::default::Default for SkeinContext {
    fn default() -> Self { unsafe { zeroed() } }
}

pub type skein224 = [u8;28];
pub type skein256 = [u8;32];
pub type skein384 = [u8;48];
pub type skein512 = [u8;64];

extern {
    pub fn sph_skein224_init(cc: *mut c_void) -> ();
    pub fn sph_skein224(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_skein224_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_skein224_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_skein256_init(cc: *mut c_void) -> ();
    pub fn sph_skein256(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_skein256_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_skein256_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_skein384_init(cc: *mut c_void) -> ();
    pub fn sph_skein384(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_skein384_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_skein384_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_skein512_init(cc: *mut c_void) -> ();
    pub fn sph_skein512(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_skein512_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_skein512_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();
}


pub fn skein224_init(cc: &mut SkeinContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_skein224_init(void_raw_cc) };
}

pub fn skein224_load(cc: &mut SkeinContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_skein224(void_raw_cc, void_raw_data, len) };
}

pub fn skein224_close(cc: &mut SkeinContext, dest: &mut skein224) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_skein224_close(void_raw_cc, void_raw_dest); };
}

pub fn skein224_init_load_close(data: &str) -> skein224 {
    let mut dest: skein224 = [0;28];
    let mut cc              = SkeinContext::default();

    skein224_init(&mut cc);
    skein224_load(&mut cc, data);
    skein224_close(&mut cc, &mut dest);

    return dest;
}

pub fn skein256_init(cc: &mut SkeinContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_skein256_init(void_raw_cc) };
}

pub fn skein256_load(cc: &mut SkeinContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_skein256(void_raw_cc, void_raw_data, len) };
}

pub fn skein256_close(cc: &mut SkeinContext, dest: &mut skein256) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_skein256_close(void_raw_cc, void_raw_dest); };
}

pub fn skein256_init_load_close(data: &str) -> skein256 {
    let mut dest: skein256 = [0;32];
    let mut cc              = SkeinContext::default();

    skein256_init(&mut cc);
    skein256_load(&mut cc, data);
    skein256_close(&mut cc, &mut dest);

    return dest;
}

pub fn skein384_init(cc: &mut SkeinContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_skein384_init(void_raw_cc) };
}

pub fn skein384_load(cc: &mut SkeinContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_skein384(void_raw_cc, void_raw_data, len) };
}

pub fn skein384_close(cc: &mut SkeinContext, dest: &mut skein384) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_skein384_close(void_raw_cc, void_raw_dest); };
}

pub fn skein384_init_load_close(data: &str) -> skein384 {
    let mut dest: skein384 = [0;48];
    let mut cc              = SkeinContext::default();

    skein384_init(&mut cc);
    skein384_load(&mut cc, data);
    skein384_close(&mut cc, &mut dest);

    return dest;
}

pub fn skein512_init(cc: &mut SkeinContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_skein512_init(void_raw_cc) };
}

pub fn skein512_load(cc: &mut SkeinContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_skein512(void_raw_cc, void_raw_data, len) };
}

pub fn skein512_close(cc: &mut SkeinContext, dest: &mut skein512) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_skein512_close(void_raw_cc, void_raw_dest); };
}

pub fn skein512_init_load_close(data: &str) -> skein512 {
    let mut dest: skein512 = [0;64];
    let mut cc              = SkeinContext::default();

    skein512_init(&mut cc);
    skein512_load(&mut cc, data);
    skein512_close(&mut cc, &mut dest);

    return dest;
}
