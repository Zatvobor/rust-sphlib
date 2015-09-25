#![allow(non_camel_case_types)]

extern crate libc;
use libc::{uint32_t, uint64_t, c_uchar, size_t, c_void};
use std::mem::{zeroed};
use utils::{to_void_raw_ctx, to_void_raw_data, to_void_raw_dest};

#[repr(C)]
#[allow(non_snake_case)]
pub struct RipemdContext {
    pub buf: [c_uchar; 64usize],
    pub val: [uint32_t; 4usize],
    pub count: uint64_t,
}

impl ::std::default::Default for RipemdContext {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct Ripemd160Context {
    pub buf: [c_uchar; 64usize],
    pub val: [uint32_t; 5usize],
    pub count: uint64_t,
}

impl ::std::default::Default for Ripemd160Context {
    fn default() -> Self { unsafe { zeroed() } }
}

pub type ripemd    = [u8;16];
pub type ripemd160 = [u8;20];

extern {
    pub fn sph_ripemd_init(cc: *mut c_void) -> ();
    pub fn sph_ripemd(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_ripemd_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_ripemd_comp(msg: *mut uint32_t, val: *mut uint32_t) -> ();

    pub fn sph_ripemd128_init(cc: *mut c_void) -> ();
    pub fn sph_ripemd128(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_ripemd128_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_ripemd128_comp(msg: *mut uint32_t, val: *mut uint32_t) -> ();

    pub fn sph_ripemd160_init(cc: *mut c_void) -> ();
    pub fn sph_ripemd160(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_ripemd160_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_ripemd160_comp(msg: *mut uint32_t, val: *mut uint32_t) -> ();
}


pub fn ripemd_init(cc: &mut RipemdContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_ripemd_init(void_raw_cc) };
}

pub fn ripemd_load(cc: &mut RipemdContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_ripemd(void_raw_cc, void_raw_data, len) };
}

pub fn ripemd_close(cc: &mut RipemdContext, dest: &mut ripemd) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_ripemd_close(void_raw_cc, void_raw_dest); };
}

pub fn ripemd_init_load_close(data: &str) -> ripemd {
    let mut dest: ripemd = [0;16];
    let mut cc           = RipemdContext::default();

    ripemd_init(&mut cc);
    ripemd_load(&mut cc, data);
    ripemd_close(&mut cc, &mut dest);

    return dest;
}

pub fn ripemd128_init(cc: &mut RipemdContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_ripemd128_init(void_raw_cc) };
}

pub fn ripemd128_load(cc: &mut RipemdContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_ripemd128(void_raw_cc, void_raw_data, len) };
}

pub fn ripemd128_close(cc: &mut RipemdContext, dest: &mut ripemd) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_ripemd128_close(void_raw_cc, void_raw_dest); };
}

pub fn ripemd128_init_load_close(data: &str) -> ripemd {
    let mut dest: ripemd = [0;16];
    let mut cc           = RipemdContext::default();

    ripemd128_init(&mut cc);
    ripemd128_load(&mut cc, data);
    ripemd128_close(&mut cc, &mut dest);

    return dest;
}

pub fn ripemd160_init(cc: &mut Ripemd160Context) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_ripemd160_init(void_raw_cc) };
}

pub fn ripemd160_load(cc: &mut Ripemd160Context, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_ripemd160(void_raw_cc, void_raw_data, len) };
}

pub fn ripemd160_close(cc: &mut Ripemd160Context, dest: &mut ripemd160) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_ripemd160_close(void_raw_cc, void_raw_dest); };
}

pub fn ripemd160_init_load_close(data: &str) -> ripemd160 {
    let mut dest: ripemd160 = [0;20];
    let mut cc              = Ripemd160Context::default();

    ripemd160_init(&mut cc);
    ripemd160_load(&mut cc, data);
    ripemd160_close(&mut cc, &mut dest);

    return dest;
}
