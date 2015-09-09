#![allow(non_camel_case_types)]

extern crate libc;
use libc::{uint32_t, c_uchar, size_t, c_void, c_uint};
use std::mem::{zeroed};
use utils::{to_void_raw_ctx, to_void_raw_data, to_void_raw_dest};

#[repr(C)]
#[allow(non_snake_case)]
pub struct ShaviteSmallContext {
    pub buf: [c_uchar; 64usize],
    pub ptr: size_t,
    pub h: [uint32_t; 8usize],
    pub count0: uint32_t,
    pub count1: uint32_t,
}

impl ::std::default::Default for ShaviteSmallContext {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct ShaviteBigContext {
    pub buf: [c_uchar; 128usize],
    pub ptr: size_t,
    pub h: [uint32_t; 16usize],
    pub count0: uint32_t,
    pub count1: uint32_t,
    pub count2: uint32_t,
    pub count3: uint32_t,
}

impl ::std::default::Default for ShaviteBigContext {
    fn default() -> Self { unsafe { zeroed() } }
}

pub type shavite224 = [u8;28];
pub type shavite256 = [u8;32];
pub type shavite384 = [u8;48];
pub type shavite512 = [u8;64];

extern  {
    pub fn sph_shavite224_init(cc: *mut c_void) -> ();
    pub fn sph_shavite224(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_shavite224_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_shavite224_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_shavite256_init(cc: *mut c_void) -> ();
    pub fn sph_shavite256(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_shavite256_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_shavite256_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_shavite384_init(cc: *mut c_void) -> ();
    pub fn sph_shavite384(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_shavite384_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_shavite384_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_shavite512_init(cc: *mut c_void) -> ();
    pub fn sph_shavite512(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_shavite512_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_shavite512_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();
}


pub fn shavite224_init(cc: &mut ShaviteSmallContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_shavite224_init(void_raw_cc) };
}

pub fn shavite224_load(cc: &mut ShaviteSmallContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_shavite224(void_raw_cc, void_raw_data, len) };
}

pub fn shavite224_close(cc: &mut ShaviteSmallContext, dest: &mut shavite224) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_shavite224_close(void_raw_cc, void_raw_dest); };
}

pub fn shavite224_init_load_close(data: &str) -> shavite224 {
    let mut dest: shavite224 = [0;28];
    let mut cc               = ShaviteSmallContext::default();

    shavite224_init(&mut cc);
    shavite224_load(&mut cc, data);
    shavite224_close(&mut cc, &mut dest);

    return dest;
}

pub fn shavite256_init(cc: &mut ShaviteSmallContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_shavite256_init(void_raw_cc) };
}

pub fn shavite256_load(cc: &mut ShaviteSmallContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_shavite256(void_raw_cc, void_raw_data, len) };
}

pub fn shavite256_close(cc: &mut ShaviteSmallContext, dest: &mut shavite256) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_shavite256_close(void_raw_cc, void_raw_dest); };
}

pub fn shavite256_init_load_close(data: &str) -> shavite256 {
    let mut dest: shavite256 = [0;32];
    let mut cc               = ShaviteSmallContext::default();

    shavite256_init(&mut cc);
    shavite256_load(&mut cc, data);
    shavite256_close(&mut cc, &mut dest);

    return dest;
}

pub fn shavite384_init(cc: &mut ShaviteBigContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_shavite384_init(void_raw_cc) };
}

pub fn shavite384_load(cc: &mut ShaviteBigContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_shavite384(void_raw_cc, void_raw_data, len) };
}

pub fn shavite384_close(cc: &mut ShaviteBigContext, dest: &mut shavite384) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_shavite384_close(void_raw_cc, void_raw_dest); };
}

pub fn shavite384_init_load_close(data: &str) -> shavite384 {
    let mut dest: shavite384 = [0;48];
    let mut cc               = ShaviteBigContext::default();

    shavite384_init(&mut cc);
    shavite384_load(&mut cc, data);
    shavite384_close(&mut cc, &mut dest);

    return dest;
}

pub fn shavite512_init(cc: &mut ShaviteBigContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_shavite512_init(void_raw_cc) };
}

pub fn shavite512_load(cc: &mut ShaviteBigContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_shavite512(void_raw_cc, void_raw_data, len) };
}

pub fn shavite512_close(cc: &mut ShaviteBigContext, dest: &mut shavite512) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_shavite512_close(void_raw_cc, void_raw_dest); };
}

pub fn shavite512_init_load_close(data: &str) -> shavite512 {
    let mut dest: shavite512 = [0;64];
    let mut cc               = ShaviteBigContext::default();

    shavite512_init(&mut cc);
    shavite512_load(&mut cc, data);
    shavite512_close(&mut cc, &mut dest);

    return dest;
}
