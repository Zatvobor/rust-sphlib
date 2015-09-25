#![allow(non_camel_case_types)]

extern crate libc;
use libc::{uint32_t, uint64_t, c_uchar, size_t, c_void, c_uint};
use std::mem::{zeroed};
use utils::{to_void_raw_ctx, to_void_raw_data, to_void_raw_dest};

#[repr(C)]
#[allow(non_snake_case)]
pub struct HavalContext {
    pub buf: [c_uchar; 128usize],
    pub s0: uint32_t,
    pub s1: uint32_t,
    pub s2: uint32_t,
    pub s3: uint32_t,
    pub s4: uint32_t,
    pub s5: uint32_t,
    pub s6: uint32_t,
    pub s7: uint32_t,
    pub olen: c_uint,
    pub passes: c_uint,
    pub count: uint64_t,
}

impl ::std::default::Default for HavalContext {
    fn default() -> Self { unsafe { zeroed() } }
}

pub type haval128 = [u8;16];
pub type haval160 = [u8;20];
pub type haval192 = [u8;24];
pub type haval224 = [u8;28];
pub type haval256 = [u8;32];

extern {
    pub fn sph_haval128_3_init(cc: *mut c_void) -> ();
    pub fn sph_haval128_3(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_haval128_3_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_haval128_3_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();
    pub fn sph_haval128_4_init(cc: *mut c_void) -> ();
    pub fn sph_haval128_4(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_haval128_4_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_haval128_4_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();
    pub fn sph_haval128_5_init(cc: *mut c_void) -> ();
    pub fn sph_haval128_5(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_haval128_5_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_haval128_5_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_haval160_3_init(cc: *mut c_void) -> ();
    pub fn sph_haval160_3(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_haval160_3_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_haval160_3_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();
    pub fn sph_haval160_4_init(cc: *mut c_void) -> ();
    pub fn sph_haval160_4(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_haval160_4_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_haval160_5_init(cc: *mut c_void) -> ();
    pub fn sph_haval160_5(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_haval160_5_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_haval160_5_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_haval192_3_init(cc: *mut c_void) -> ();
    pub fn sph_haval192_3(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_haval192_3_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_haval192_3_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();
    pub fn sph_haval192_4_init(cc: *mut c_void) -> ();
    pub fn sph_haval192_4(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_haval192_4_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_haval192_4_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();
    pub fn sph_haval192_5_init(cc: *mut c_void) -> ();
    pub fn sph_haval192_5(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_haval192_5_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_haval192_5_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_haval224_3_init(cc: *mut c_void) -> ();
    pub fn sph_haval224_3(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_haval224_3_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_haval224_3_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();
    pub fn sph_haval224_4_init(cc: *mut c_void) -> ();
    pub fn sph_haval224_4(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_haval224_4_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_haval224_4_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();
    pub fn sph_haval224_5_init(cc: *mut c_void) -> ();
    pub fn sph_haval224_5(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_haval224_5_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_haval224_5_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_haval256_3_init(cc: *mut c_void) -> ();
    pub fn sph_haval256_3(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_haval256_3_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_haval256_3_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();
    pub fn sph_haval256_4_init(cc: *mut c_void) -> ();
    pub fn sph_haval256_4(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_haval256_4_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_haval256_4_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();
    pub fn sph_haval256_5_init(cc: *mut c_void) -> ();
    pub fn sph_haval256_5(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_haval256_5_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_haval256_5_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_haval_3_comp(msg: *mut uint32_t, val: *mut uint32_t) -> ();
    pub fn sph_haval_4_comp(msg: *mut uint32_t, val: *mut uint32_t) -> ();
    pub fn sph_haval_5_comp(msg: *mut uint32_t, val: *mut uint32_t) -> ();
}


pub fn haval128_3_init(cc: &mut HavalContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_haval128_3_init(void_raw_cc) };
}

pub fn haval128_3_load(cc: &mut HavalContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_haval128_3(void_raw_cc, void_raw_data, len) };
}

pub fn haval128_3_close(cc: &mut HavalContext, dest: &mut haval128) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_haval128_3_close(void_raw_cc, void_raw_dest); };
}

pub fn haval128_3_init_load_close(data: &str) -> haval128 {
    let mut dest: haval128 = [0;16];
    let mut cc             = HavalContext::default();

    haval128_3_init(&mut cc);
    haval128_3_load(&mut cc, data);
    haval128_3_close(&mut cc, &mut dest);

    return dest;
}

pub fn haval128_4_init(cc: &mut HavalContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_haval128_4_init(void_raw_cc) };
}

pub fn haval128_4_load(cc: &mut HavalContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_haval128_4(void_raw_cc, void_raw_data, len) };
}

pub fn haval128_4_close(cc: &mut HavalContext, dest: &mut haval128) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_haval128_4_close(void_raw_cc, void_raw_dest); };
}

pub fn haval128_4_init_load_close(data: &str) -> haval128 {
    let mut dest: haval128 = [0;16];
    let mut cc             = HavalContext::default();

    haval128_4_init(&mut cc);
    haval128_4_load(&mut cc, data);
    haval128_4_close(&mut cc, &mut dest);

    return dest;
}

pub fn haval128_5_init(cc: &mut HavalContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_haval128_5_init(void_raw_cc) };
}

pub fn haval128_5_load(cc: &mut HavalContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_haval128_5(void_raw_cc, void_raw_data, len) };
}

pub fn haval128_5_close(cc: &mut HavalContext, dest: &mut haval128) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_haval128_5_close(void_raw_cc, void_raw_dest); };
}

pub fn haval128_5_init_load_close(data: &str) -> haval128 {
    let mut dest: haval128 = [0;16];
    let mut cc             = HavalContext::default();

    haval128_5_init(&mut cc);
    haval128_5_load(&mut cc, data);
    haval128_5_close(&mut cc, &mut dest);

    return dest;
}


pub fn haval160_3_init(cc: &mut HavalContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_haval160_3_init(void_raw_cc) };
}

pub fn haval160_3_load(cc: &mut HavalContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_haval160_3(void_raw_cc, void_raw_data, len) };
}

pub fn haval160_3_close(cc: &mut HavalContext, dest: &mut haval160) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_haval160_3_close(void_raw_cc, void_raw_dest); };
}

pub fn haval160_3_init_load_close(data: &str) -> haval160 {
    let mut dest: haval160 = [0;20];
    let mut cc             = HavalContext::default();

    haval160_3_init(&mut cc);
    haval160_3_load(&mut cc, data);
    haval160_3_close(&mut cc, &mut dest);

    return dest;
}
pub fn haval160_4_init(cc: &mut HavalContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_haval160_4_init(void_raw_cc) };
}

pub fn haval160_4_load(cc: &mut HavalContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_haval160_4(void_raw_cc, void_raw_data, len) };
}

pub fn haval160_4_close(cc: &mut HavalContext, dest: &mut haval160) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_haval160_4_close(void_raw_cc, void_raw_dest); };
}

pub fn haval160_4_init_load_close(data: &str) -> haval160 {
    let mut dest: haval160 = [0;20];
    let mut cc             = HavalContext::default();

    haval160_4_init(&mut cc);
    haval160_4_load(&mut cc, data);
    haval160_4_close(&mut cc, &mut dest);

    return dest;
}

pub fn haval160_5_init(cc: &mut HavalContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_haval160_5_init(void_raw_cc) };
}

pub fn haval160_5_load(cc: &mut HavalContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_haval160_5(void_raw_cc, void_raw_data, len) };
}

pub fn haval160_5_close(cc: &mut HavalContext, dest: &mut haval160) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_haval160_5_close(void_raw_cc, void_raw_dest); };
}

pub fn haval160_5_init_load_close(data: &str) -> haval160 {
    let mut dest: haval160 = [0;20];
    let mut cc             = HavalContext::default();

    haval160_5_init(&mut cc);
    haval160_5_load(&mut cc, data);
    haval160_5_close(&mut cc, &mut dest);

    return dest;
}

pub fn haval192_3_init(cc: &mut HavalContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_haval192_3_init(void_raw_cc) };
}

pub fn haval192_3_load(cc: &mut HavalContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_haval192_3(void_raw_cc, void_raw_data, len) };
}

pub fn haval192_3_close(cc: &mut HavalContext, dest: &mut haval192) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_haval192_3_close(void_raw_cc, void_raw_dest); };
}

pub fn haval192_3_init_load_close(data: &str) -> haval192 {
    let mut dest: haval192 = [0;24];
    let mut cc             = HavalContext::default();

    haval192_3_init(&mut cc);
    haval192_3_load(&mut cc, data);
    haval192_3_close(&mut cc, &mut dest);

    return dest;
}

pub fn haval192_4_init(cc: &mut HavalContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_haval192_4_init(void_raw_cc) };
}

pub fn haval192_4_load(cc: &mut HavalContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_haval192_4(void_raw_cc, void_raw_data, len) };
}

pub fn haval192_4_close(cc: &mut HavalContext, dest: &mut haval192) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_haval192_4_close(void_raw_cc, void_raw_dest); };
}

pub fn haval192_4_init_load_close(data: &str) -> haval192 {
    let mut dest: haval192 = [0;24];
    let mut cc             = HavalContext::default();

    haval192_4_init(&mut cc);
    haval192_4_load(&mut cc, data);
    haval192_4_close(&mut cc, &mut dest);

    return dest;
}

pub fn haval192_5_init(cc: &mut HavalContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_haval192_5_init(void_raw_cc) };
}

pub fn haval192_5_load(cc: &mut HavalContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_haval192_5(void_raw_cc, void_raw_data, len) };
}

pub fn haval192_5_close(cc: &mut HavalContext, dest: &mut haval192) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_haval192_5_close(void_raw_cc, void_raw_dest); };
}

pub fn haval192_5_init_load_close(data: &str) -> haval192 {
    let mut dest: haval192 = [0;24];
    let mut cc             = HavalContext::default();

    haval192_5_init(&mut cc);
    haval192_5_load(&mut cc, data);
    haval192_5_close(&mut cc, &mut dest);

    return dest;
}

pub fn haval224_3_init(cc: &mut HavalContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_haval224_3_init(void_raw_cc) };
}

pub fn haval224_3_load(cc: &mut HavalContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_haval224_3(void_raw_cc, void_raw_data, len) };
}

pub fn haval224_3_close(cc: &mut HavalContext, dest: &mut haval224) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_haval224_3_close(void_raw_cc, void_raw_dest); };
}

pub fn haval224_3_init_load_close(data: &str) -> haval224 {
    let mut dest: haval224 = [0;28];
    let mut cc             = HavalContext::default();

    haval224_3_init(&mut cc);
    haval224_3_load(&mut cc, data);
    haval224_3_close(&mut cc, &mut dest);

    return dest;
}

pub fn haval224_4_init(cc: &mut HavalContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_haval224_4_init(void_raw_cc) };
}

pub fn haval224_4_load(cc: &mut HavalContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_haval224_4(void_raw_cc, void_raw_data, len) };
}

pub fn haval224_4_close(cc: &mut HavalContext, dest: &mut haval224) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_haval224_4_close(void_raw_cc, void_raw_dest); };
}

pub fn haval224_4_init_load_close(data: &str) -> haval224 {
    let mut dest: haval224 = [0;28];
    let mut cc             = HavalContext::default();

    haval224_4_init(&mut cc);
    haval224_4_load(&mut cc, data);
    haval224_4_close(&mut cc, &mut dest);

    return dest;
}

pub fn haval224_5_init(cc: &mut HavalContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_haval224_5_init(void_raw_cc) };
}

pub fn haval224_5_load(cc: &mut HavalContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_haval224_5(void_raw_cc, void_raw_data, len) };
}

pub fn haval224_5_close(cc: &mut HavalContext, dest: &mut haval224) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_haval224_5_close(void_raw_cc, void_raw_dest); };
}

pub fn haval224_5_init_load_close(data: &str) -> haval224 {
    let mut dest: haval224 = [0;28];
    let mut cc             = HavalContext::default();

    haval224_5_init(&mut cc);
    haval224_5_load(&mut cc, data);
    haval224_5_close(&mut cc, &mut dest);

    return dest;
}

pub fn haval256_3_init(cc: &mut HavalContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_haval256_3_init(void_raw_cc) };
}

pub fn haval256_3_load(cc: &mut HavalContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_haval256_3(void_raw_cc, void_raw_data, len) };
}

pub fn haval256_3_close(cc: &mut HavalContext, dest: &mut haval256) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_haval256_3_close(void_raw_cc, void_raw_dest); };
}

pub fn haval256_3_init_load_close(data: &str) -> haval256 {
    let mut dest: haval256 = [0;32];
    let mut cc             = HavalContext::default();

    haval256_3_init(&mut cc);
    haval256_3_load(&mut cc, data);
    haval256_3_close(&mut cc, &mut dest);

    return dest;
}
pub fn haval256_4_init(cc: &mut HavalContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_haval256_4_init(void_raw_cc) };
}

pub fn haval256_4_load(cc: &mut HavalContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_haval256_4(void_raw_cc, void_raw_data, len) };
}

pub fn haval256_4_close(cc: &mut HavalContext, dest: &mut haval256) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_haval256_4_close(void_raw_cc, void_raw_dest); };
}

pub fn haval256_4_init_load_close(data: &str) -> haval256 {
    let mut dest: haval256 = [0;32];
    let mut cc             = HavalContext::default();

    haval256_4_init(&mut cc);
    haval256_4_load(&mut cc, data);
    haval256_4_close(&mut cc, &mut dest);

    return dest;
}

pub fn haval256_5_init(cc: &mut HavalContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_haval256_5_init(void_raw_cc) };
}

pub fn haval256_5_load(cc: &mut HavalContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_haval256_5(void_raw_cc, void_raw_data, len) };
}

pub fn haval256_5_close(cc: &mut HavalContext, dest: &mut haval256) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_haval256_5_close(void_raw_cc, void_raw_dest); };
}

pub fn haval256_5_init_load_close(data: &str) -> haval256 {
    let mut dest: haval256 = [0;32];
    let mut cc             = HavalContext::default();

    haval256_5_init(&mut cc);
    haval256_5_load(&mut cc, data);
    haval256_5_close(&mut cc, &mut dest);

    return dest;
}
