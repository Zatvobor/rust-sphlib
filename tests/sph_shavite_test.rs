extern crate sphlib;
extern crate libc;

use sphlib::{sph_shavite, utils};

#[test]
fn will_be_224_hash() {
    let dest   = sph_shavite::shavite224_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("b33f761f0d3a86bb1051905aec7a691bd0b5a24c3721f67d8e48d839", actual.to_string());
}

#[test]
fn will_be_256_hash() {
    let dest   = sph_shavite::shavite256_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("08c5825af2e9e5947286a8fe208bd5f8c6a7c8e4da598947d7ff8eda0fcd2bd7", actual.to_string());
}

#[test]
fn will_be_384_hash() {
    let dest   = sph_shavite::shavite384_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("814b55553ce7c0841f8ff0321e6287f9f50a8e0cae811932385ecc1b7c386b4eb14edb79c8381babf09276b69d1bb3ee", actual.to_string());
}

#[test]
fn will_be_512_hash() {
    let dest   = sph_shavite::shavite512_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("a485c1b2578459d1efc5dddd840bb0b4a650ac82fe68f58c4442ccda747da006b2d1dc6b4a4eb7d84ff91e1f466fef429d259acd995dddcad16fa545c7a6e5ba", actual.to_string());
}
