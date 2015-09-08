extern crate sphlib;
extern crate libc;

use sphlib::{sph_groestl, utils};

#[test]
fn will_be_224_hash() {
    let dest   = sph_groestl::groestl224_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("f2e180fb5947be964cd584e22e496242c6a329c577fc4ce8c36d34c3", actual.to_string());
}

#[test]
fn will_be_256_hash() {
    let dest   = sph_groestl::groestl256_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("1a52d11d550039be16107f9c58db9ebcc417f16f736adb2502567119f0083467", actual.to_string());
}

#[test]
fn will_be_384_hash() {
    let dest   = sph_groestl::groestl384_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("ac353c1095ace21439251007862d6c62f829ddbe6de4f78e68d310a9205a736d8b11d99bffe448f57a1cfa2934f044a5", actual.to_string());
}

#[test]
fn will_be_512_hash() {
    let dest   = sph_groestl::groestl512_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("6d3ad29d279110eef3adbd66de2a0345a77baede1557f5d099fce0c03d6dc2ba8e6d4a6633dfbd66053c20faa87d1a11f39a7fbe4a6c2f009801370308fc4ad8", actual.to_string());
}
