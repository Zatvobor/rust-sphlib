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
