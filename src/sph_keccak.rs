#![allow(non_camel_case_types)]

extern crate libc;
use libc::{uint32_t, uint64_t, c_uchar, size_t, c_void, c_uint};
use std::mem::{zeroed, transmute};
use utils::{to_void_raw_ctx, to_void_raw_data, to_void_raw_dest};

#[repr(C)]
#[allow(non_snake_case)]
pub struct KeccakContext {
    pub buf: [c_uchar; 144usize],
    pub ptr: size_t,
    pub lim: size_t,
    pub u: UnionContext,
}

impl ::std::default::Default for KeccakContext {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct UnionContext {
    pub _bindgen_data_: [u64; 25usize],
}

#[allow(non_snake_case)]
impl UnionContext {
    pub unsafe fn wide(&mut self) -> *mut [uint64_t; 25usize] {
        let raw: *mut u8 = transmute(&self._bindgen_data_);
        transmute(raw.offset(0))
    }
    pub unsafe fn narrow(&mut self) -> *mut [uint32_t; 50usize] {
        let raw: *mut u8 = transmute(&self._bindgen_data_);
        transmute(raw.offset(0))
    }
}

impl ::std::default::Default for UnionContext {
    fn default() -> Self { unsafe { zeroed() } }
}

pub type keccak224 = [u8;28];
pub type keccak256 = [u8;32];
pub type keccak384 = [u8;48];
pub type keccak512 = [u8;64];

extern {
    pub fn sph_keccak224_init(cc: *mut c_void) -> ();
    pub fn sph_keccak224(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_keccak224_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_keccak224_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_keccak256_init(cc: *mut c_void) -> ();
    pub fn sph_keccak256(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_keccak256_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_keccak256_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_keccak384_init(cc: *mut c_void) -> ();
    pub fn sph_keccak384(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_keccak384_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_keccak384_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_keccak512_init(cc: *mut c_void) -> ();
    pub fn sph_keccak512(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_keccak512_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_keccak512_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();
}


pub fn keccak224_init(cc: &mut KeccakContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_keccak224_init(void_raw_cc) };
}

pub fn keccak224_load(cc: &mut KeccakContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_keccak224(void_raw_cc, void_raw_data, len) };
}

pub fn keccak224_close(cc: &mut KeccakContext, dest: &mut keccak224) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_keccak224_close(void_raw_cc, void_raw_dest); };
}

pub fn keccak224_init_load_close(data: &str) -> keccak224 {
    let mut dest: keccak224 = [0;28];
    let mut cc              = KeccakContext::default();

    keccak224_init(&mut cc);
    keccak224_load(&mut cc, data);
    keccak224_close(&mut cc, &mut dest);

    return dest;
}

pub fn keccak256_init(cc: &mut KeccakContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_keccak256_init(void_raw_cc) };
}

pub fn keccak256_load(cc: &mut KeccakContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_keccak256(void_raw_cc, void_raw_data, len) };
}

pub fn keccak256_close(cc: &mut KeccakContext, dest: &mut keccak256) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_keccak256_close(void_raw_cc, void_raw_dest); };
}

pub fn keccak256_init_load_close(data: &str) -> keccak256 {
    let mut dest: keccak256 = [0;32];
    let mut cc              = KeccakContext::default();

    keccak256_init(&mut cc);
    keccak256_load(&mut cc, data);
    keccak256_close(&mut cc, &mut dest);

    return dest;
}

pub fn keccak384_init(cc: &mut KeccakContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_keccak384_init(void_raw_cc) };
}

pub fn keccak384_load(cc: &mut KeccakContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_keccak384(void_raw_cc, void_raw_data, len) };
}

pub fn keccak384_close(cc: &mut KeccakContext, dest: &mut keccak384) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_keccak384_close(void_raw_cc, void_raw_dest); };
}

pub fn keccak384_init_load_close(data: &str) -> keccak384 {
    let mut dest: keccak384 = [0;48];
    let mut cc              = KeccakContext::default();

    keccak384_init(&mut cc);
    keccak384_load(&mut cc, data);
    keccak384_close(&mut cc, &mut dest);

    return dest;
}

pub fn keccak512_init(cc: &mut KeccakContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_keccak512_init(void_raw_cc) };
}

pub fn keccak512_load(cc: &mut KeccakContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_keccak512(void_raw_cc, void_raw_data, len) };
}

pub fn keccak512_close(cc: &mut KeccakContext, dest: &mut keccak512) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_keccak512_close(void_raw_cc, void_raw_dest); };
}

pub fn keccak512_init_load_close(data: &str) -> keccak512 {
    let mut dest: keccak512 = [0;64];
    let mut cc              = KeccakContext::default();

    keccak512_init(&mut cc);
    keccak512_load(&mut cc, data);
    keccak512_close(&mut cc, &mut dest);

    return dest;
}
