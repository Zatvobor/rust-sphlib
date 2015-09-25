extern crate sphlib;
extern crate libc;

use sphlib::{sph_radiogatun, utils};

#[test]
fn will_be_32_hash() {
    let dest   = sph_radiogatun::radiogatun32_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("f30028b54afab6b3e55355d277711109a19beda7091067e9a492fb5ed9f20117", actual.to_string());
}

#[test]
fn will_be_64_hash() {
    let dest   = sph_radiogatun::radiogatun64_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("64a9a7fa139905b57bdab35d33aa216370d5eae13e77bfcdd85513408311a584", actual.to_string());
}
