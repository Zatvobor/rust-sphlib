extern crate sphlib;
extern crate libc;

use sphlib::{sph_shabal, utils};

#[test]
fn will_be_192_hash() {
    let dest   = sph_shabal::shabal192_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("e10dc32232f98b039dbbcfa41269b9cdf67a73c841214c81", actual.to_string());
}

#[test]
fn will_be_224_hash() {
    let dest   = sph_shabal::shabal224_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("562b4fdbe1706247552927f814b66a3d74b465a090af23e277bf8029", actual.to_string());
}

#[test]
fn will_be_256_hash() {
    let dest   = sph_shabal::shabal256_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("aec750d11feee9f16271922fbaf5a9be142f62019ef8d720f858940070889014", actual.to_string());
}

#[test]
fn will_be_384_hash() {
    let dest   = sph_shabal::shabal384_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("ff093d67d22b06a674b5f384719150d617e0ff9c8923569a2ab60cda886df63c91a25f33cd71cc22c9eebc5cd6aee52a", actual.to_string());
}

#[test]
fn will_be_512_hash() {
    let dest   = sph_shabal::shabal512_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("fc2d5dff5d70b7f6b1f8c2fcc8c1f9fe9934e54257eded0cf2b539a2ef0a19ccffa84f8d9fa135e4bd3c09f590f3a927ebd603ac29eb729e6f2a9af031ad8dc6", actual.to_string());
}
