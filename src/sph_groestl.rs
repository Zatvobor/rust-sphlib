#![allow(non_camel_case_types)]

extern crate libc;
use libc::{uint32_t, uint64_t, c_uchar, size_t, c_void, c_uint};
use std::mem::{zeroed, transmute};
use utils::{to_void_raw_ctx, to_void_raw_data, to_void_raw_dest};

#[repr(C)]
#[allow(non_snake_case)]
pub struct GroestlSmallContext {
    pub buf: [c_uchar; 64usize],
    pub ptr: size_t,
    pub state: UnionSmallContext,
    pub count: uint64_t,
}

impl ::std::default::Default for GroestlSmallContext {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct UnionSmallContext {
    pub _bindgen_data_: [u64; 8usize],
}

impl UnionSmallContext {
    pub unsafe fn wide(&mut self) -> *mut [uint64_t; 8usize] {
        let raw: *mut u8 = transmute(&self._bindgen_data_);
        transmute(raw.offset(0))
    }
    pub unsafe fn narrow(&mut self) -> *mut [uint32_t; 16usize] {
        let raw: *mut u8 = transmute(&self._bindgen_data_);
        transmute(raw.offset(0))
    }
}

impl ::std::default::Default for UnionSmallContext {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct GroestlBigContext {
    pub buf: [c_uchar; 128usize],
    pub ptr: size_t,
    pub state: UnionBigContext,
    pub count: uint64_t,
}

impl ::std::default::Default for GroestlBigContext {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct UnionBigContext {
    pub _bindgen_data_: [u64; 16usize],
}

impl UnionBigContext {
    pub unsafe fn wide(&mut self) -> *mut [uint64_t; 16usize] {
        let raw: *mut u8 = transmute(&self._bindgen_data_);
        transmute(raw.offset(0))
    }
    pub unsafe fn narrow(&mut self) -> *mut [uint32_t; 32usize] {
        let raw: *mut u8 = transmute(&self._bindgen_data_);
        transmute(raw.offset(0))
    }
}

impl ::std::default::Default for UnionBigContext {
    fn default() -> Self { unsafe { zeroed() } }
}

pub type groestl224 = [u8;28];
pub type groestl256 = [u8;32];
pub type groestl384 = [u8;48];
pub type groestl512 = [u8;64];

extern {
    pub fn sph_groestl224_init(cc: *mut c_void) -> ();
    pub fn sph_groestl224(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_groestl224_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_groestl224_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_groestl256_init(cc: *mut c_void) -> ();
    pub fn sph_groestl256(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_groestl256_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_groestl256_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_groestl384_init(cc: *mut c_void) -> ();
    pub fn sph_groestl384(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_groestl384_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_groestl384_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_groestl512_init(cc: *mut c_void) -> ();
    pub fn sph_groestl512(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_groestl512_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_groestl512_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();
}


pub fn groestl224_init(cc: &mut GroestlSmallContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_groestl224_init(void_raw_cc) };
}

pub fn groestl224_load(cc: &mut GroestlSmallContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_groestl224(void_raw_cc, void_raw_data, len) };
}

pub fn groestl224_close(cc: &mut GroestlSmallContext, dest: &mut groestl224) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_groestl224_close(void_raw_cc, void_raw_dest); };
}

pub fn groestl224_init_load_close(data: &str) -> groestl224 {
    let mut dest: groestl224 = [0;28];
    let mut cc               = GroestlSmallContext::default();

    groestl224_init(&mut cc);
    groestl224_load(&mut cc, data);
    groestl224_close(&mut cc, &mut dest);

    return dest;
}

pub fn groestl256_init(cc: &mut GroestlSmallContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_groestl256_init(void_raw_cc) };
}

pub fn groestl256_load(cc: &mut GroestlSmallContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_groestl256(void_raw_cc, void_raw_data, len) };
}

pub fn groestl256_close(cc: &mut GroestlSmallContext, dest: &mut groestl256) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_groestl256_close(void_raw_cc, void_raw_dest); };
}

pub fn groestl256_init_load_close(data: &str) -> groestl256 {
    let mut dest: groestl256 = [0;32];
    let mut cc               = GroestlSmallContext::default();

    groestl256_init(&mut cc);
    groestl256_load(&mut cc, data);
    groestl256_close(&mut cc, &mut dest);

    return dest;
}

pub fn groestl384_init(cc: &mut GroestlBigContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_groestl384_init(void_raw_cc) };
}

pub fn groestl384_load(cc: &mut GroestlBigContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_groestl384(void_raw_cc, void_raw_data, len) };
}

pub fn groestl384_close(cc: &mut GroestlBigContext, dest: &mut groestl384) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_groestl384_close(void_raw_cc, void_raw_dest); };
}

pub fn groestl384_init_load_close(data: &str) -> groestl384 {
    let mut dest: groestl384 = [0;48];
    let mut cc               = GroestlBigContext::default();

    groestl384_init(&mut cc);
    groestl384_load(&mut cc, data);
    groestl384_close(&mut cc, &mut dest);

    return dest;
}

pub fn groestl512_init(cc: &mut GroestlBigContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_groestl512_init(void_raw_cc) };
}

pub fn groestl512_load(cc: &mut GroestlBigContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_groestl512(void_raw_cc, void_raw_data, len) };
}

pub fn groestl512_close(cc: &mut GroestlBigContext, dest: &mut groestl512) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_groestl512_close(void_raw_cc, void_raw_dest); };
}

pub fn groestl512_init_load_close(data: &str) -> groestl512 {
    let mut dest: groestl512 = [0;64];
    let mut cc               = GroestlBigContext::default();

    groestl512_init(&mut cc);
    groestl512_load(&mut cc, data);
    groestl512_close(&mut cc, &mut dest);

    return dest;
}
