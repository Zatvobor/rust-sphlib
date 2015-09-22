#![allow(non_camel_case_types)]

extern crate libc;
use libc::{uint32_t, uint64_t, c_uchar, size_t, c_void, c_uint};
use std::mem::{zeroed};
use utils::{to_void_raw_ctx, to_void_raw_data, to_void_raw_dest};

#[repr(C)]
#[allow(non_snake_case)]
pub struct Radiogatun32Context {
    pub data: [c_uchar; 156usize],
    pub data_ptr: c_uint,
    pub a: [uint32_t; 19usize],
    pub b: [uint32_t; 39usize],
}

impl ::std::default::Default for Radiogatun32Context {
    fn default() -> Self { unsafe { zeroed() } }
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct Radiogatun64Context {
    pub data: [c_uchar; 312usize],
    pub data_ptr: c_uint,
    pub a: [uint64_t; 19usize],
    pub b: [uint64_t; 39usize],
}

impl ::std::default::Default for Radiogatun64Context {
    fn default() -> Self { unsafe { zeroed() } }
}

pub type radiogatun = [u8;32];

extern {
    pub fn sph_radiogatun32_init(cc: *mut c_void) -> ();
    pub fn sph_radiogatun32(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_radiogatun32_close(cc: *mut c_void, dst: *mut c_void) -> ();

    pub fn sph_radiogatun64_init(cc: *mut c_void) -> ();
    pub fn sph_radiogatun64(cc: *mut c_void, data: *const c_void, len: size_t) -> ();
    pub fn sph_radiogatun64_close(cc: *mut c_void, dst: *mut c_void) -> ();
}


pub fn radiogatun32_init(cc: &mut Radiogatun32Context) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_radiogatun32_init(void_raw_cc) };
}

pub fn radiogatun32_load(cc: &mut Radiogatun32Context, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_radiogatun32(void_raw_cc, void_raw_data, len) };
}

pub fn radiogatun32_close(cc: &mut Radiogatun32Context, dest: &mut radiogatun) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_radiogatun32_close(void_raw_cc, void_raw_dest); };
}

pub fn radiogatun32_init_load_close(data: &str) -> radiogatun {
    let mut dest: radiogatun = [0;32];
    let mut cc               = Radiogatun32Context::default();

    radiogatun32_init(&mut cc);
    radiogatun32_load(&mut cc, data);
    radiogatun32_close(&mut cc, &mut dest);

    return dest;
}

pub fn radiogatun64_init(cc: &mut Radiogatun64Context) {
    let void_raw_cc = to_void_raw_ctx(cc);
    unsafe { sph_radiogatun64_init(void_raw_cc) };
}

pub fn radiogatun64_load(cc: &mut Radiogatun64Context, data: &str) {
    let void_raw_cc             = to_void_raw_ctx(cc);
    let (void_raw_data, len)    = to_void_raw_data(data);
    unsafe { sph_radiogatun64(void_raw_cc, void_raw_data, len) };
}

pub fn radiogatun64_close(cc: &mut Radiogatun64Context, dest: &mut radiogatun) {
    let void_raw_cc     = to_void_raw_ctx(cc);
    let void_raw_dest   = to_void_raw_dest(dest);
    unsafe { sph_radiogatun64_close(void_raw_cc, void_raw_dest); };
}

pub fn radiogatun64_init_load_close(data: &str) -> radiogatun {
    let mut dest: radiogatun = [0;32];
    let mut cc               = Radiogatun64Context::default();

    radiogatun64_init(&mut cc);
    radiogatun64_load(&mut cc, data);
    radiogatun64_close(&mut cc, &mut dest);

    return dest;
}
