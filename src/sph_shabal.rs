#![allow(non_camel_case_types)]

extern crate libc;
use libc::{uint32_t, c_uchar, size_t, c_void, c_uint};
use std::mem::{zeroed};
use utils::{to_void_raw_ctx, to_void_raw_data, to_void_raw_dest};

#[repr(C)]
#[allow(non_snake_case)]
pub struct ShabalContext {
    pub buf: [c_uchar; 64usize],
    pub ptr: size_t,
    pub A: [uint32_t; 12usize],
    pub B: [uint32_t; 16usize],
    pub C: [uint32_t; 16usize],
    pub Whigh: uint32_t,
    pub Wlow: uint32_t,
}

impl ::std::default::Default for ShabalContext {
    fn default() -> Self { unsafe { zeroed() } }
}

pub type shabal192 = [u8;24];
pub type shabal224 = [u8;28];
pub type shabal256 = [u8;32];
pub type shabal384 = [u8;48];
pub type shabal512 = [u8;64];

extern {
    pub fn sph_shabal192_init(cc: *mut c_void) -> ();
    pub fn sph_shabal192(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_shabal192_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_shabal192_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_shabal224_init(cc: *mut c_void) -> ();
    pub fn sph_shabal224(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_shabal224_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_shabal224_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_shabal256_init(cc: *mut c_void) -> ();
    pub fn sph_shabal256(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_shabal256_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_shabal256_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_shabal384_init(cc: *mut c_void) -> ();
    pub fn sph_shabal384(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_shabal384_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_shabal384_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_shabal512_init(cc: *mut c_void) -> ();
    pub fn sph_shabal512(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_shabal512_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_shabal512_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();
}


pub fn shabal192_init(cc: &mut ShabalContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_shabal192_init(void_raw_cc) };
}

pub fn shabal192_load(cc: &mut ShabalContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_shabal192(void_raw_cc, void_raw_data, len) };
}

pub fn shabal192_close(cc: &mut ShabalContext, dest: &mut shabal192) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_shabal192_close(void_raw_cc, void_raw_dest); };
}

pub fn shabal192_init_load_close(data: &str) -> shabal192 {
    let mut dest: shabal192 = [0;24];
    let mut cc              = ShabalContext::default();

    shabal192_init(&mut cc);
    shabal192_load(&mut cc, data);
    shabal192_close(&mut cc, &mut dest);

    return dest;
}

pub fn shabal224_init(cc: &mut ShabalContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_shabal224_init(void_raw_cc) };
}

pub fn shabal224_load(cc: &mut ShabalContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_shabal224(void_raw_cc, void_raw_data, len) };
}

pub fn shabal224_close(cc: &mut ShabalContext, dest: &mut shabal224) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_shabal224_close(void_raw_cc, void_raw_dest); };
}

pub fn shabal224_init_load_close(data: &str) -> shabal224 {
    let mut dest: shabal224 = [0;28];
    let mut cc               = ShabalContext::default();

    shabal224_init(&mut cc);
    shabal224_load(&mut cc, data);
    shabal224_close(&mut cc, &mut dest);

    return dest;
}

pub fn shabal256_init(cc: &mut ShabalContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_shabal256_init(void_raw_cc) };
}

pub fn shabal256_load(cc: &mut ShabalContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_shabal256(void_raw_cc, void_raw_data, len) };
}

pub fn shabal256_close(cc: &mut ShabalContext, dest: &mut shabal256) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_shabal256_close(void_raw_cc, void_raw_dest); };
}

pub fn shabal256_init_load_close(data: &str) -> shabal256 {
    let mut dest: shabal256 = [0;32];
    let mut cc               = ShabalContext::default();

    shabal256_init(&mut cc);
    shabal256_load(&mut cc, data);
    shabal256_close(&mut cc, &mut dest);

    return dest;
}

pub fn shabal384_init(cc: &mut ShabalContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_shabal384_init(void_raw_cc) };
}

pub fn shabal384_load(cc: &mut ShabalContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_shabal384(void_raw_cc, void_raw_data, len) };
}

pub fn shabal384_close(cc: &mut ShabalContext, dest: &mut shabal384) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_shabal384_close(void_raw_cc, void_raw_dest); };
}

pub fn shabal384_init_load_close(data: &str) -> shabal384 {
    let mut dest: shabal384 = [0;48];
    let mut cc              = ShabalContext::default();

    shabal384_init(&mut cc);
    shabal384_load(&mut cc, data);
    shabal384_close(&mut cc, &mut dest);

    return dest;
}

pub fn shabal512_init(cc: &mut ShabalContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_shabal512_init(void_raw_cc) };
}

pub fn shabal512_load(cc: &mut ShabalContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_shabal512(void_raw_cc, void_raw_data, len) };
}

pub fn shabal512_close(cc: &mut ShabalContext, dest: &mut shabal512) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_shabal512_close(void_raw_cc, void_raw_dest); };
}

pub fn shabal512_init_load_close(data: &str) -> shabal512 {
    let mut dest: shabal512 = [0;64];
    let mut cc               = ShabalContext::default();

    shabal512_init(&mut cc);
    shabal512_load(&mut cc, data);
    shabal512_close(&mut cc, &mut dest);

    return dest;
}
