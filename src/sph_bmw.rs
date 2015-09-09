#![allow(non_camel_case_types)]

extern crate libc;
use libc::{uint32_t, uint64_t, c_uchar, size_t, c_void, c_uint};
use utils::{to_void_raw_ctx, to_void_raw_data, to_void_raw_dest};

#[repr(C)]
#[allow(non_snake_case)]
pub struct BmwSmallContext {
    pub buf: [c_uchar; 64usize],
    pub ptr: size_t,
    pub H: [uint32_t; 16usize],
    pub bit_count: uint64_t,
}

impl ::std::default::Default for BmwSmallContext {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct BmwBigContext {
    pub buf: [c_uchar; 128usize],
    pub ptr: size_t,
    pub H: [uint64_t; 16usize],
    pub bit_count: uint64_t,
}

impl ::std::default::Default for BmwBigContext {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

pub type bmw224 = [u8;28];
pub type bmw256 = [u8;32];
pub type bmw384 = [u8;48];
pub type bmw512 = [u8;64];

extern {
    pub fn sph_bmw224_init(cc: *mut c_void) -> ();
    pub fn sph_bmw224(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_bmw224_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_bmw224_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_bmw256_init(cc: *mut c_void) -> ();
    pub fn sph_bmw256(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_bmw256_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_bmw256_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_bmw384_init(cc: *mut c_void) -> ();
    pub fn sph_bmw384(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_bmw384_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_bmw384_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_bmw512_init(cc: *mut c_void) -> ();
    pub fn sph_bmw512(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_bmw512_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_bmw512_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();
}


pub fn bmw224_init(cc: &mut BmwSmallContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_bmw224_init(void_raw_cc) };
}

pub fn bmw224_load(cc: &mut BmwSmallContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_bmw224(void_raw_cc, void_raw_data, len) };
}

pub fn bmw224_close(cc: &mut BmwSmallContext, dest: &mut bmw224) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_bmw224_close(void_raw_cc, void_raw_dest); };
}

pub fn bmw224_init_load_close(data: &str) -> bmw224 {
    let mut dest: bmw224 = [0;28];
    let mut cc           = BmwSmallContext::default();

    bmw224_init(&mut cc);
    bmw224_load(&mut cc, data);
    bmw224_close(&mut cc, &mut dest);

    return dest;
}

pub fn bmw256_init(cc: &mut BmwSmallContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_bmw256_init(void_raw_cc) };
}

pub fn bmw256_load(cc: &mut BmwSmallContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_bmw256(void_raw_cc, void_raw_data, len) };
}

pub fn bmw256_close(cc: &mut BmwSmallContext, dest: &mut bmw256) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_bmw256_close(void_raw_cc, void_raw_dest); };
}

pub fn bmw256_init_load_close(data: &str) -> bmw256 {
    let mut dest: bmw256 = [0;32];
    let mut cc             = BmwSmallContext::default();

    bmw256_init(&mut cc);
    bmw256_load(&mut cc, data);
    bmw256_close(&mut cc, &mut dest);

    return dest;
}

pub fn bmw384_init(cc: &mut BmwBigContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_bmw384_init(void_raw_cc) };
}

pub fn bmw384_load(cc: &mut BmwBigContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_bmw384(void_raw_cc, void_raw_data, len) };
}

pub fn bmw384_close(cc: &mut BmwBigContext, dest: &mut bmw384) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_bmw384_close(void_raw_cc, void_raw_dest); };
}

pub fn bmw384_init_load_close(data: &str) -> bmw384 {
    let mut dest: bmw384 = [0;48];
    let mut cc           = BmwBigContext::default();

    bmw384_init(&mut cc);
    bmw384_load(&mut cc, data);
    bmw384_close(&mut cc, &mut dest);

    return dest;
}

pub fn bmw512_init(cc: &mut BmwBigContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_bmw512_init(void_raw_cc) };
}

pub fn bmw512_load(cc: &mut BmwBigContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_bmw512(void_raw_cc, void_raw_data, len) };
}

pub fn bmw512_close(cc: &mut BmwBigContext, dest: &mut bmw512) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_bmw512_close(void_raw_cc, void_raw_dest); };
}

pub fn bmw512_init_load_close(data: &str) -> bmw512 {
    let mut dest: bmw512 = [0;64];
    let mut cc           = BmwBigContext::default();

    bmw512_init(&mut cc);
    bmw512_load(&mut cc, data);
    bmw512_close(&mut cc, &mut dest);

    return dest;
}
