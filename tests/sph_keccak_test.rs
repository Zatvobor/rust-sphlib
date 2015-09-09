extern crate sphlib;
extern crate libc;

use sphlib::{sph_keccak, utils};

#[test]
fn will_be_224_hash() {
    let dest   = sph_keccak::keccak224_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("f71837502ba8e10837bdd8d365adb85591895602fc552b48b7390abd", actual.to_string());
}

#[test]
fn will_be_256_hash() {
    let dest   = sph_keccak::keccak256_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470", actual.to_string());
}

#[test]
fn will_be_384_hash() {
    let dest   = sph_keccak::keccak384_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("2c23146a63a29acf99e73b88f8c24eaa7dc60aa771780ccc006afbfa8fe2479b2dd2b21362337441ac12b515911957ff", actual.to_string());
}

#[test]
fn will_be_512_hash() {
    let dest   = sph_keccak::keccak512_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("0eab42de4c3ceb9235fc91acffe746b29c29a8c366b7c60e4e67c466f36a4304c00fa9caf9d87976ba469bcbe06713b435f091ef2769fb160cdab33d3670680e", actual.to_string());
}
