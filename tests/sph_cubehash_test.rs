extern crate sphlib;
extern crate libc;

use sphlib::{sph_cubehash, utils};

#[test]
fn will_be_224_hash() {
    let dest   = sph_cubehash::cubehash224_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("f9802aa6955f4b7cf3b0f5a378fa0c9f138e0809d250966879c873ab", actual.to_string());
}

#[test]
fn will_be_256_hash() {
    let dest   = sph_cubehash::cubehash256_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("44c6de3ac6c73c391bf0906cb7482600ec06b216c7c54a2a8688a6a42676577d", actual.to_string());
}

#[test]
fn will_be_384_hash() {
    let dest   = sph_cubehash::cubehash384_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("98ae93ebf4e58958497f610a22c8cf60f2292319283ca6459daed1707be06e7591c5f2d84bd3339e66c770e485bfa1fb", actual.to_string());
}

#[test]
fn will_be_512_hash() {
    let dest   = sph_cubehash::cubehash512_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("4a1d00bbcfcb5a9562fb981e7f7db3350fe2658639d948b9d57452c22328bb32f468b072208450bad5ee178271408be0b16e5633ac8a1e3cf9864cfbfc8e043a", actual.to_string());
}
