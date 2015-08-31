#![allow(non_camel_case_types)]

extern crate libc;
use libc::{uint32_t, int32_t, uint64_t, int64_t, c_uchar, size_t, c_void, c_uint};

pub type sph_u32 = uint32_t;
pub type sph_s32 = int32_t;

#[repr(C)]
#[derive(Copy)]
#[allow(non_snake_case)]
pub struct BlakeSmallContext {
    pub buf: [c_uchar; 64usize],
    pub ptr: size_t,
    pub H: [sph_u32; 8usize],
    pub S: [sph_u32; 4usize],
    pub T0: sph_u32,
    pub T1: sph_u32,
}

impl ::std::clone::Clone for BlakeSmallContext {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for BlakeSmallContext {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

pub type sph_blake_small_context = BlakeSmallContext;
pub type sph_blake224_context = sph_blake_small_context;
pub type sph_blake256_context = sph_blake_small_context;

pub type sph_u64 = uint64_t;
pub type sph_s64 = int64_t;

#[repr(C)]
#[derive(Copy)]
#[allow(non_snake_case)]
pub struct BlakeBigContext {
    pub buf: [c_uchar; 128usize],
    pub ptr: size_t,
    pub H: [sph_u64; 8usize],
    pub S: [sph_u64; 4usize],
    pub T0: sph_u64,
    pub T1: sph_u64,
}

impl ::std::clone::Clone for BlakeBigContext {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for BlakeBigContext {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

pub type sph_blake_big_context = BlakeBigContext;
pub type sph_blake384_context = sph_blake_big_context;
pub type sph_blake512_context = sph_blake_big_context;


#[link(name="sphlib", kind="static")]
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

pub fn blake224(data: &str) -> [u8;28] {
    let mut dest: [u8;28] = [0;28];
    unsafe {
        let mut cc = BlakeSmallContext::default();
        let raw_cc = &mut cc as *mut BlakeSmallContext;
        let void_raw_cc = raw_cc as *mut c_void;
        // FIXME Figure out what's wrong here `let void_raw_cc = to_void_raw_small_ctx()`;
        sph_blake224_init(void_raw_cc);

        let (void_raw_data, len) = to_void_raw_data(data);
        sph_blake224(void_raw_cc, void_raw_data, len);

        let void_raw_dest = to_void_raw_dest(&mut dest);
        sph_blake224_close(void_raw_cc, void_raw_dest);
    }
    return dest;
}

pub fn blake256(data: &str) -> [u8;32] {
    let mut dest: [u8;32] = [0;32];
    unsafe {
        let mut cc = BlakeSmallContext::default();
        let raw_cc = &mut cc as *mut BlakeSmallContext;
        let void_raw_cc = raw_cc as *mut c_void;
        // FIXME Figure out what's wrong here `let void_raw_cc = to_void_raw_small_ctx()`;
        sph_blake256_init(void_raw_cc);

        let (void_raw_data, len) = to_void_raw_data(data);
        sph_blake256(void_raw_cc, void_raw_data, len);

        let void_raw_dest = to_void_raw_dest(&mut dest);
        sph_blake256_close(void_raw_cc, void_raw_dest);
    }
    return dest;
}

pub fn blake384(data: &str) -> [u8;48] {
    let mut dest: [u8;48] = [0;48];
    unsafe {
        let mut cc = BlakeBigContext::default();
        let raw_cc = &mut cc as *mut BlakeBigContext;
        let void_raw_cc = raw_cc as *mut c_void;
        // FIXME Figure out what's wrong here `let void_raw_cc = to_void_raw_small_ctx()`;
        sph_blake384_init(void_raw_cc);

        let (void_raw_data, len) = to_void_raw_data(data);
        sph_blake384(void_raw_cc, void_raw_data, len);

        let void_raw_dest = to_void_raw_dest(&mut dest);
        sph_blake384_close(void_raw_cc, void_raw_dest);
    }
    return dest;
}

pub fn blake512(data: &str) -> [u8;64] {
    let mut dest: [u8;64] = [0;64];
    unsafe {
        let mut cc = BlakeBigContext::default();
        let raw_cc = &mut cc as *mut BlakeBigContext;
        let void_raw_cc = raw_cc as *mut c_void;
        // FIXME Figure out what's wrong here `let void_raw_cc = to_void_raw_small_ctx()`;
        sph_blake512_init(void_raw_cc);

        let (void_raw_data, len) = to_void_raw_data(data);
        sph_blake512(void_raw_cc, void_raw_data, len);

        let void_raw_dest = to_void_raw_dest(&mut dest);
        sph_blake512_close(void_raw_cc, void_raw_dest);
    }
    return dest;
}


// private

#[allow(dead_code)]
fn to_void_raw_small_ctx() -> *mut c_void {
    let mut cc = BlakeSmallContext::default();
    let raw_cc = &mut cc as *mut BlakeSmallContext;
    let void_raw_cc = raw_cc as *mut c_void;

    return void_raw_cc;
}

fn to_void_raw_data(data: &str) -> (*const c_void, size_t) {
    let void_raw_data = data.as_ptr() as *const c_void;
    let len = data.len() as size_t;

    return (void_raw_data, len);
}

fn to_void_raw_dest(dest: &mut [u8]) -> *mut c_void {
    let raw_dest = dest.as_mut() as *mut [u8];
    let void_raw_dest = raw_dest as *mut c_void;

    return void_raw_dest;
}
