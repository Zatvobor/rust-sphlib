#![allow(non_camel_case_types)]

extern crate libc;
use libc::{uint32_t, uint64_t, c_uchar, size_t, c_void, c_uint};
use std::mem::{zeroed, transmute};
use utils::{to_void_raw_ctx, to_void_raw_data, to_void_raw_dest};

#[repr(C)]
#[allow(non_snake_case)]
pub struct JhContext {
    pub buf: [c_uchar; 64usize],
    pub ptr: size_t,
    pub H: UnionContext,
    pub block_count: uint64_t,
}

impl ::std::default::Default for JhContext {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct UnionContext {
    pub _bindgen_data_: [u64; 16usize],
}

impl UnionContext {
    pub unsafe fn wide(&mut self) -> *mut [uint64_t; 16usize] {
        let raw: *mut u8 = transmute(&self._bindgen_data_);
        transmute(raw.offset(0))
    }
    pub unsafe fn narrow(&mut self) -> *mut [uint32_t; 32usize] {
        let raw: *mut u8 = transmute(&self._bindgen_data_);
        transmute(raw.offset(0))
    }
}

impl ::std::default::Default for UnionContext {
    fn default() -> Self { unsafe { zeroed() } }
}

pub type jh224 = [u8;28];
pub type jh256 = [u8;32];
pub type jh384 = [u8;48];
pub type jh512 = [u8;64];

extern {
    pub fn sph_jh224_init(cc: *mut c_void) -> ();
    pub fn sph_jh224(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_jh224_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_jh224_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_jh256_init(cc: *mut c_void) -> ();
    pub fn sph_jh256(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_jh256_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_jh256_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_jh384_init(cc: *mut c_void) -> ();
    pub fn sph_jh384(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_jh384_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_jh384_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_jh512_init(cc: *mut c_void) -> ();
    pub fn sph_jh512(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_jh512_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_jh512_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();
}


pub fn jh224_init(cc: &mut JhContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_jh224_init(void_raw_cc) };
}

pub fn jh224_load(cc: &mut JhContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_jh224(void_raw_cc, void_raw_data, len) };
}

pub fn jh224_close(cc: &mut JhContext, dest: &mut jh224) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_jh224_close(void_raw_cc, void_raw_dest); };
}

pub fn jh224_init_load_close(data: &str) -> jh224 {
    let mut dest: jh224 = [0;28];
    let mut cc            = JhContext::default();

    jh224_init(&mut cc);
    jh224_load(&mut cc, data);
    jh224_close(&mut cc, &mut dest);

    return dest;
}

pub fn jh256_init(cc: &mut JhContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_jh256_init(void_raw_cc) };
}

pub fn jh256_load(cc: &mut JhContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_jh256(void_raw_cc, void_raw_data, len) };
}

pub fn jh256_close(cc: &mut JhContext, dest: &mut jh256) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_jh256_close(void_raw_cc, void_raw_dest); };
}

pub fn jh256_init_load_close(data: &str) -> jh256 {
    let mut dest: jh256 = [0;32];
    let mut cc            = JhContext::default();

    jh256_init(&mut cc);
    jh256_load(&mut cc, data);
    jh256_close(&mut cc, &mut dest);

    return dest;
}

pub fn jh384_init(cc: &mut JhContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_jh384_init(void_raw_cc) };
}

pub fn jh384_load(cc: &mut JhContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_jh384(void_raw_cc, void_raw_data, len) };
}

pub fn jh384_close(cc: &mut JhContext, dest: &mut jh384) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_jh384_close(void_raw_cc, void_raw_dest); };
}

pub fn jh384_init_load_close(data: &str) -> jh384 {
    let mut dest: jh384 = [0;48];
    let mut cc            = JhContext::default();

    jh384_init(&mut cc);
    jh384_load(&mut cc, data);
    jh384_close(&mut cc, &mut dest);

    return dest;
}

pub fn jh512_init(cc: &mut JhContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_jh512_init(void_raw_cc) };
}

pub fn jh512_load(cc: &mut JhContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_jh512(void_raw_cc, void_raw_data, len) };
}

pub fn jh512_close(cc: &mut JhContext, dest: &mut jh512) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_jh512_close(void_raw_cc, void_raw_dest); };
}

pub fn jh512_init_load_close(data: &str) -> jh512 {
    let mut dest: jh512 = [0;64];
    let mut cc            = JhContext::default();

    jh512_init(&mut cc);
    jh512_load(&mut cc, data);
    jh512_close(&mut cc, &mut dest);

    return dest;
}
