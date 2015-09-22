extern crate sphlib;
extern crate libc;

use sphlib::{sph_ripemd, utils};

#[test]
fn will_be_hash() {
    let dest   = sph_ripemd::ripemd_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("9f73aa9b372a9dacfb86a6108852e2d9", actual.to_string());
}

#[test]
fn will_be_128_hash() {
    let dest   = sph_ripemd::ripemd128_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("cdf26213a150dc3ecb610f18f6b38b46", actual.to_string());
}

#[test]
fn will_be_160_hash() {
    let dest   = sph_ripemd::ripemd160_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("9c1185a5c5e9fc54612808977ee8f548b2258d31", actual.to_string());
}
