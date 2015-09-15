extern crate sphlib;
extern crate libc;

use sphlib::{sph_hamsi, utils};

#[test]
fn will_be_224_hash() {
    let dest   = sph_hamsi::hamsi224_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("b9f6eb1a9b990373f9d2cb125584333c69a3d41ae291845f05da221f", actual.to_string());
}

#[test]
fn will_be_256_hash() {
    let dest   = sph_hamsi::hamsi256_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("750e9ec469f4db626bee7e0c10ddaa1bd01fe194b94efbabebd24764dc2b13e9", actual.to_string());
}

#[test]
fn will_be_384_hash() {
    let dest   = sph_hamsi::hamsi384_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("3943cd34e3b96b197a8bf4bac7aa982d18530dd12f41136b26d7e88759255f21153f4a4bd02e523612b8427f9dd96c8d", actual.to_string());
}

#[test]
fn will_be_512_hash() {
    let dest   = sph_hamsi::hamsi512_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("5cd7436a91e27fc809d7015c3407540633dab391127113ce6ba360f0c1e35f404510834a551610d6e871e75651ea381a8ba628af1dcf2b2be13af2eb6247290f", actual.to_string());
}
