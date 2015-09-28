extern crate sphlib;
extern crate libc;

use sphlib::{sph_md, utils};

#[test]
fn will_be_md2_hash() {
    let dest   = sph_md::md2_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("8350e5a3e24c153df2275c9f80692773", actual.to_string());
}

#[test]
fn will_be_md4_hash() {
    let dest   = sph_md::md4_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("31d6cfe0d16ae931b73c59d7e0c089c0", actual.to_string());
}

#[test]
fn will_be_md5_hash() {
    let dest   = sph_md::md5_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("d41d8cd98f00b204e9800998ecf8427e", actual.to_string());
}
