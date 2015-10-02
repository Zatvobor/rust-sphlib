#![allow(non_camel_case_types)]

extern crate libc;
use libc::{uint32_t, uint64_t, c_uchar, size_t, c_void, c_uint};
use std::mem::{zeroed};
use utils::{to_void_raw_ctx, to_void_raw_data, to_void_raw_dest};

#[repr(C)]
#[allow(non_snake_case)]
pub struct ShaSmallContext {
    pub buf: [c_uchar; 64usize],
    pub val: [uint32_t; 5usize],
    pub count: uint64_t,
}

impl ::std::default::Default for ShaSmallContext {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct ShaMediumContext {
    pub buf: [c_uchar; 64usize],
    pub val: [uint32_t; 8usize],
    pub count: uint64_t,
}

impl ::std::default::Default for ShaMediumContext {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct ShaBigContext {
    pub buf: [c_uchar; 128usize],
    pub val: [uint64_t; 8usize],
    pub count: uint64_t,
}

impl ::std::default::Default for ShaBigContext {
    fn default() -> Self { unsafe { zeroed() } }
}

pub type sha0   = [u8;20];
pub type sha1   = [u8;20];
pub type sha224 = [u8;28];
pub type sha256 = [u8;32];
pub type sha384 = [u8;48];
pub type sha512 = [u8;64];

extern {
    pub fn sph_sha0_init(cc: *mut c_void) -> ();
    pub fn sph_sha0(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_sha0_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_sha0_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();
    pub fn sph_sha0_comp(msg: *mut uint32_t, val: *mut uint32_t) -> ();

    pub fn sph_sha1_init(cc: *mut c_void) -> ();
    pub fn sph_sha1(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_sha1_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_sha1_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();
    pub fn sph_sha1_comp(msg: *mut uint32_t, val: *mut uint32_t) -> ();

    pub fn sph_sha224_init(cc: *mut c_void) -> ();
    pub fn sph_sha224(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_sha224_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_sha224_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();
    pub fn sph_sha224_comp(msg: *mut uint32_t, val: *mut uint32_t) -> ();

    pub fn sph_sha256_init(cc: *mut c_void) -> ();
    pub fn sph_sha256_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_sha256_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_sha384_init(cc: *mut c_void) -> ();
    pub fn sph_sha384(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_sha384_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_sha384_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();
    pub fn sph_sha384_comp(msg: *mut uint64_t, val: *mut uint64_t) -> ();

    pub fn sph_sha512_init(cc: *mut c_void) -> ();
    pub fn sph_sha512_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_sha512_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();
}


pub fn sha0_init(cc: &mut ShaSmallContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_sha0_init(void_raw_cc) };
}

pub fn sha0_load(cc: &mut ShaSmallContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_sha0(void_raw_cc, void_raw_data, len) };
}

pub fn sha0_close(cc: &mut ShaSmallContext, dest: &mut sha0) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_sha0_close(void_raw_cc, void_raw_dest); };
}

pub fn sha0_init_load_close(data: &str) -> sha0 {
    let mut dest: sha0 = [0;20];
    let mut cc        = ShaSmallContext::default();

    sha0_init(&mut cc);
    sha0_load(&mut cc, data);
    sha0_close(&mut cc, &mut dest);

    return dest;
}

pub fn sha1_init(cc: &mut ShaSmallContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_sha1_init(void_raw_cc) };
}

pub fn sha1_load(cc: &mut ShaSmallContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_sha1(void_raw_cc, void_raw_data, len) };
}

pub fn sha1_close(cc: &mut ShaSmallContext, dest: &mut sha1) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_sha1_close(void_raw_cc, void_raw_dest); };
}

pub fn sha1_init_load_close(data: &str) -> sha1 {
    let mut dest: sha1 = [0;20];
    let mut cc        = ShaSmallContext::default();

    sha1_init(&mut cc);
    sha1_load(&mut cc, data);
    sha1_close(&mut cc, &mut dest);

    return dest;
}

pub fn sha224_init(cc: &mut ShaMediumContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_sha224_init(void_raw_cc) };
}

pub fn sha224_load(cc: &mut ShaMediumContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_sha224(void_raw_cc, void_raw_data, len) };
}

pub fn sha224_close(cc: &mut ShaMediumContext, dest: &mut sha224) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_sha224_close(void_raw_cc, void_raw_dest); };
}

pub fn sha224_init_load_close(data: &str) -> sha224 {
    let mut dest: sha224 = [0;28];
    let mut cc           = ShaMediumContext::default();

    sha224_init(&mut cc);
    sha224_load(&mut cc, data);
    sha224_close(&mut cc, &mut dest);

    return dest;
}

pub fn sha256_init(cc: &mut ShaMediumContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_sha256_init(void_raw_cc) };
}

pub fn sha256_load(cc: &mut ShaMediumContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_sha224(void_raw_cc, void_raw_data, len) };
}

pub fn sha256_close(cc: &mut ShaMediumContext, dest: &mut sha256) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_sha256_close(void_raw_cc, void_raw_dest); };
}

pub fn sha256_init_load_close(data: &str) -> sha256 {
    let mut dest: sha256 = [0;32];
    let mut cc           = ShaMediumContext::default();

    sha256_init(&mut cc);
    sha256_load(&mut cc, data);
    sha256_close(&mut cc, &mut dest);

    return dest;
}

pub fn sha384_init(cc: &mut ShaBigContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_sha384_init(void_raw_cc) };
}

pub fn sha384_load(cc: &mut ShaBigContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_sha384(void_raw_cc, void_raw_data, len) };
}

pub fn sha384_close(cc: &mut ShaBigContext, dest: &mut sha384) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_sha384_close(void_raw_cc, void_raw_dest); };
}

pub fn sha384_init_load_close(data: &str) -> sha384 {
    let mut dest: sha384 = [0;48];
    let mut cc           = ShaBigContext::default();

    sha384_init(&mut cc);
    sha384_load(&mut cc, data);
    sha384_close(&mut cc, &mut dest);

    return dest;
}

pub fn sha512_init(cc: &mut ShaBigContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_sha512_init(void_raw_cc) };
}

pub fn sha512_load(cc: &mut ShaBigContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_sha384(void_raw_cc, void_raw_data, len) };
}

pub fn sha512_close(cc: &mut ShaBigContext, dest: &mut sha512) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_sha512_close(void_raw_cc, void_raw_dest); };
}

pub fn sha512_init_load_close(data: &str) -> sha512 {
    let mut dest: sha512 = [0;64];
    let mut cc              = ShaBigContext::default();

    sha512_init(&mut cc);
    sha512_load(&mut cc, data);
    sha512_close(&mut cc, &mut dest);

    return dest;
}
