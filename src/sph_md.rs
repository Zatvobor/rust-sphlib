#![allow(non_camel_case_types)]

extern crate libc;
use libc::{uint32_t, uint64_t, c_uchar, size_t, c_void, c_uint};
use std::mem::{zeroed, transmute};
use utils::{to_void_raw_ctx, to_void_raw_data, to_void_raw_dest};

#[repr(C)]
#[allow(non_snake_case)]
pub struct Md2Context {
    pub buf: [c_uchar; 16usize],
    pub u: UnionMd2Context,
    pub C: [c_uchar; 16usize],
    pub L: c_uint,
    pub count: c_uint,
}

impl ::std::default::Default for Md2Context {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct UnionMd2Context {
    pub _bindgen_data_: [u32; 12usize],
}

#[allow(non_snake_case)]
impl UnionMd2Context {
    pub unsafe fn X(&mut self) -> *mut [c_uchar; 48usize] {
        let raw: *mut u8 = transmute(&self._bindgen_data_);
        transmute(raw.offset(0))
    }
    pub unsafe fn W(&mut self) -> *mut [uint32_t; 12usize] {
        let raw: *mut u8 = transmute(&self._bindgen_data_);
        transmute(raw.offset(0))
    }
}

impl ::std::default::Default for UnionMd2Context {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct MdContext {
    pub buf: [c_uchar; 64usize],
    pub val: [uint32_t; 4usize],
    pub count: uint64_t,
}

impl ::std::default::Default for MdContext {
    fn default() -> Self { unsafe { zeroed() } }
}

pub type md2 = [u8;16];
pub type md4 = [u8;16];
pub type md5 = [u8;16];

extern {
    pub fn sph_md2_init(cc: *mut c_void) -> ();
    pub fn sph_md2(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_md2_close(cc: *mut c_void, dst: *mut c_void) -> ();

    pub fn sph_md4_init(cc: *mut c_void) -> ();
    pub fn sph_md4(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_md4_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_md4_comp(msg: *mut uint32_t, val: *mut uint32_t) -> ();

    pub fn sph_md5_init(cc: *mut c_void) -> ();
    pub fn sph_md5(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_md5_close(cc: *mut c_void, dst: *mut c_void) -> ();
    pub fn sph_md5_addbits_and_close(cc: *mut c_void, ub: c_uint, n: c_uint, dst: *mut c_void) -> ();
    pub fn sph_md5_comp(msg: *mut uint32_t, val: *mut uint32_t) -> ();
}


pub fn md2_init(cc: &mut Md2Context) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_md2_init(void_raw_cc) };
}

pub fn md2_load(cc: &mut Md2Context, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_md2(void_raw_cc, void_raw_data, len) };
}

pub fn md2_close(cc: &mut Md2Context, dest: &mut md2) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_md2_close(void_raw_cc, void_raw_dest); };
}

pub fn md2_init_load_close(data: &str) -> md2 {
    let mut dest: md2 = [0;16];
    let mut cc        = Md2Context::default();

    md2_init(&mut cc);
    md2_load(&mut cc, data);
    md2_close(&mut cc, &mut dest);

    return dest;
}

pub fn md4_init(cc: &mut MdContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_md4_init(void_raw_cc) };
}

pub fn md4_load(cc: &mut MdContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_md4(void_raw_cc, void_raw_data, len) };
}

pub fn md4_close(cc: &mut MdContext, dest: &mut md4) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_md4_close(void_raw_cc, void_raw_dest); };
}

pub fn md4_init_load_close(data: &str) -> md4 {
    let mut dest: md4 = [0;16];
    let mut cc        = MdContext::default();

    md4_init(&mut cc);
    md4_load(&mut cc, data);
    md4_close(&mut cc, &mut dest);

    return dest;
}

pub fn md5_init(cc: &mut MdContext) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_md5_init(void_raw_cc) };
}

pub fn md5_load(cc: &mut MdContext, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_md5(void_raw_cc, void_raw_data, len) };
}

pub fn md5_close(cc: &mut MdContext, dest: &mut md5) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_md5_close(void_raw_cc, void_raw_dest); };
}

pub fn md5_init_load_close(data: &str) -> md5 {
    let mut dest: md5 = [0;16];
    let mut cc        = MdContext::default();

    md5_init(&mut cc);
    md5_load(&mut cc, data);
    md5_close(&mut cc, &mut dest);

    return dest;
}
