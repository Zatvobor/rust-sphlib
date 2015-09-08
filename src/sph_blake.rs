#![allow(non_camel_case_types)]

extern crate libc;
use libc::{uint32_t, uint64_t, c_uchar, size_t, c_void, c_uint};
use utils::{to_void_raw_ctx, to_void_raw_data, to_void_raw_dest};

#[repr(C)]
#[allow(non_snake_case)]
pub struct BlakeSmallContext {
    pub buf: [c_uchar; 64usize],
    pub ptr: size_t,
    pub H: [uint32_t; 8usize],
    pub S: [uint32_t; 4usize],
    pub T0: uint32_t,
    pub T1: uint32_t,
}

impl ::std::default::Default for BlakeSmallContext {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct BlakeBigContext {
    pub buf: [c_uchar; 128usize],
    pub ptr: size_t,
    pub H: [uint64_t; 8usize],
    pub S: [uint64_t; 4usize],
    pub T0: uint64_t,
    pub T1: uint64_t,
}

impl ::std::default::Default for BlakeBigContext {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

pub type blake224 = [u8;28];
pub type blake256 = [u8;32];
pub type blake384 = [u8;48];
pub type blake512 = [u8;64];

extern {
    pub fn sph_blake224_init(cc: *mut c_void) -> ();
    pub fn sph_blake224(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_blake224_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_blake224_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_blake256_init(cc: *mut c_void) -> ();
    pub fn sph_blake256(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_blake256_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_blake256_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_blake384_init(cc: *mut c_void) -> ();
    pub fn sph_blake384(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_blake384_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_blake384_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_blake512_init(cc: *mut c_void) -> ();
    pub fn sph_blake512(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_blake512_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_blake512_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();
}


pub fn blake224_init(cc: &mut BlakeSmallContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_blake224_init(void_raw_cc) };
}

pub fn blake224_load(cc: &mut BlakeSmallContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_blake224(void_raw_cc, void_raw_data, len) };
}

pub fn blake224_close(cc: &mut BlakeSmallContext, dest: &mut blake224) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_blake224_close(void_raw_cc, void_raw_dest); };
}

pub fn blake224_init_load_close(data: &str) -> blake224 {
    let mut dest: blake224 = [0;28];
    let mut cc             = BlakeSmallContext::default();

    blake224_init(&mut cc);
    blake224_load(&mut cc, data);
    blake224_close(&mut cc, &mut dest);

    return dest;
}

pub fn blake256_init(cc: &mut BlakeSmallContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_blake256_init(void_raw_cc) };
}

pub fn blake256_load(cc: &mut BlakeSmallContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_blake256(void_raw_cc, void_raw_data, len) };
}

pub fn blake256_close(cc: &mut BlakeSmallContext, dest: &mut blake256) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_blake256_close(void_raw_cc, void_raw_dest); };
}

pub fn blake256_init_load_close(data: &str) -> blake256 {
    let mut dest: blake256 = [0;32];
    let mut cc             = BlakeSmallContext::default();

    blake256_init(&mut cc);
    blake256_load(&mut cc, data);
    blake256_close(&mut cc, &mut dest);

    return dest;
}

pub fn blake384_init(cc: &mut BlakeBigContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_blake384_init(void_raw_cc) };
}

pub fn blake384_load(cc: &mut BlakeBigContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_blake384(void_raw_cc, void_raw_data, len) };
}

pub fn blake384_close(cc: &mut BlakeBigContext, dest: &mut blake384) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_blake384_close(void_raw_cc, void_raw_dest); };
}

pub fn blake384_init_load_close(data: &str) -> blake384 {
    let mut dest: blake384 = [0;48];
    let mut cc             = BlakeBigContext::default();

    blake384_init(&mut cc);
    blake384_load(&mut cc, data);
    blake384_close(&mut cc, &mut dest);

    return dest;
}

pub fn blake512_init(cc: &mut BlakeBigContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_blake512_init(void_raw_cc) };
}

pub fn blake512_load(cc: &mut BlakeBigContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_blake512(void_raw_cc, void_raw_data, len) };
}

pub fn blake512_close(cc: &mut BlakeBigContext, dest: &mut blake512) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_blake512_close(void_raw_cc, void_raw_dest); };
}

pub fn blake512_init_load_close(data: &str) -> blake512 {
    let mut dest: blake512 = [0;64];
    let mut cc             = BlakeBigContext::default();

    blake512_init(&mut cc);
    blake512_load(&mut cc, data);
    blake512_close(&mut cc, &mut dest);

    return dest;
}
