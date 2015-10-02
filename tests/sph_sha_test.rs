extern crate sphlib;
extern crate libc;

use sphlib::{sph_sha, utils};

#[test]
fn will_be_sha0_hash() {
    let dest   = sph_sha::sha0_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("f96cea198ad1dd5617ac084a3d92c6107708c0ef", actual.to_string());
}

#[test]
fn will_be_sha1_hash() {
    let dest   = sph_sha::sha1_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("da39a3ee5e6b4b0d3255bfef95601890afd80709", actual.to_string());
}

#[test]
fn will_be_sha224_hash() {
    let dest   = sph_sha::sha224_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("d14a028c2a3a2bc9476102bb288234c415a2b01f828ea62ac5b3e42f", actual.to_string());
}

#[test]
fn will_be_sha256_hash() {
    let dest   = sph_sha::sha256_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855", actual.to_string());
}

#[test]
fn will_be_sha384_hash() {
    let dest   = sph_sha::sha384_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("38b060a751ac96384cd9327eb1b1e36a21fdb71114be07434c0cc7bf63f6e1da274edebfe76f65fbd51ad2f14898b95b", actual.to_string());
}

#[test]
fn will_be_sha512_hash() {
    let dest   = sph_sha::sha512_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e", actual.to_string());
}
