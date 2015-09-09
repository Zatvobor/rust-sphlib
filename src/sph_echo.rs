#![allow(non_camel_case_types)]

extern crate libc;
use libc::{uint32_t, uint64_t, c_uchar, size_t, c_void, c_uint};
use std::mem::{zeroed, transmute};
use utils::{to_void_raw_ctx, to_void_raw_data, to_void_raw_dest};

#[repr(C)]
#[allow(non_snake_case)]
pub struct EchoSmallContext {
    pub buf: [c_uchar; 192usize],
    pub ptr: size_t,
    pub u: UnionSmallContext,
    pub C0: uint32_t,
    pub C1: uint32_t,
    pub C2: uint32_t,
    pub C3: uint32_t,
}

impl ::std::default::Default for EchoSmallContext {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct UnionSmallContext {
    pub _bindgen_data_: [u64; 8usize],
}

#[allow(non_snake_case)]
impl UnionSmallContext {
    pub unsafe fn Vs(&mut self) -> *mut [[uint32_t; 4usize]; 4usize] {
        let raw: *mut u8 = transmute(&self._bindgen_data_);
        transmute(raw.offset(0))
    }
    pub unsafe fn Vb(&mut self) -> *mut [[uint64_t; 2usize]; 4usize] {
        let raw: *mut u8 = transmute(&self._bindgen_data_);
        transmute(raw.offset(0))
    }
}

impl ::std::default::Default for UnionSmallContext {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct EchoBigContext {
    pub buf: [c_uchar; 128usize],
    pub ptr: size_t,
    pub u: UnionBigContext,
    pub C0: uint32_t,
    pub C1: uint32_t,
    pub C2: uint32_t,
    pub C3: uint32_t,
}

impl ::std::default::Default for EchoBigContext {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct UnionBigContext {
    pub _bindgen_data_: [u64; 16usize],
}

#[allow(non_snake_case)]
impl UnionBigContext {
    pub unsafe fn Vs(&mut self) -> *mut [[uint32_t; 4usize]; 8usize] {
        let raw: *mut u8 = transmute(&self._bindgen_data_);
        transmute(raw.offset(0))
    }
    pub unsafe fn Vb(&mut self) -> *mut [[uint64_t; 2usize]; 8usize] {
        let raw: *mut u8 = transmute(&self._bindgen_data_);
        transmute(raw.offset(0))
    }
}

impl ::std::default::Default for UnionBigContext {
    fn default() -> Self { unsafe { zeroed() } }
}

pub type echo224 = [u8;28];
pub type echo256 = [u8;32];
pub type echo384 = [u8;48];
pub type echo512 = [u8;64];

extern {
    pub fn sph_echo224_init(cc: *mut c_void) -> ();
    pub fn sph_echo224(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_echo224_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_echo224_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_echo256_init(cc: *mut c_void) -> ();
    pub fn sph_echo256(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_echo256_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_echo256_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_echo384_init(cc: *mut c_void) -> ();
    pub fn sph_echo384(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_echo384_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_echo384_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();

    pub fn sph_echo512_init(cc: *mut c_void) -> ();
    pub fn sph_echo512(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_echo512_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_echo512_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();
}


pub fn echo224_init(cc: &mut EchoSmallContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_echo224_init(void_raw_cc) };
}

pub fn echo224_load(cc: &mut EchoSmallContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_echo224(void_raw_cc, void_raw_data, len) };
}

pub fn echo224_close(cc: &mut EchoSmallContext, dest: &mut echo224) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_echo224_close(void_raw_cc, void_raw_dest); };
}

pub fn echo224_init_load_close(data: &str) -> echo224 {
    let mut dest: echo224 = [0;28];
    let mut cc            = EchoSmallContext::default();

    echo224_init(&mut cc);
    echo224_load(&mut cc, data);
    echo224_close(&mut cc, &mut dest);

    return dest;
}

pub fn echo256_init(cc: &mut EchoSmallContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_echo256_init(void_raw_cc) };
}

pub fn echo256_load(cc: &mut EchoSmallContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_echo256(void_raw_cc, void_raw_data, len) };
}

pub fn echo256_close(cc: &mut EchoSmallContext, dest: &mut echo256) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_echo256_close(void_raw_cc, void_raw_dest); };
}

pub fn echo256_init_load_close(data: &str) -> echo256 {
    let mut dest: echo256 = [0;32];
    let mut cc            = EchoSmallContext::default();

    echo256_init(&mut cc);
    echo256_load(&mut cc, data);
    echo256_close(&mut cc, &mut dest);

    return dest;
}

pub fn echo384_init(cc: &mut EchoBigContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_echo384_init(void_raw_cc) };
}

pub fn echo384_load(cc: &mut EchoBigContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_echo384(void_raw_cc, void_raw_data, len) };
}

pub fn echo384_close(cc: &mut EchoBigContext, dest: &mut echo384) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_echo384_close(void_raw_cc, void_raw_dest); };
}

pub fn echo384_init_load_close(data: &str) -> echo384 {
    let mut dest: echo384 = [0;48];
    let mut cc            = EchoBigContext::default();

    echo384_init(&mut cc);
    echo384_load(&mut cc, data);
    echo384_close(&mut cc, &mut dest);

    return dest;
}

pub fn echo512_init(cc: &mut EchoBigContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_echo512_init(void_raw_cc) };
}

pub fn echo512_load(cc: &mut EchoBigContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_echo512(void_raw_cc, void_raw_data, len) };
}

pub fn echo512_close(cc: &mut EchoBigContext, dest: &mut echo512) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_echo512_close(void_raw_cc, void_raw_dest); };
}

pub fn echo512_init_load_close(data: &str) -> echo512 {
    let mut dest: echo512 = [0;64];
    let mut cc            = EchoBigContext::default();

    echo512_init(&mut cc);
    echo512_load(&mut cc, data);
    echo512_close(&mut cc, &mut dest);

    return dest;
}
