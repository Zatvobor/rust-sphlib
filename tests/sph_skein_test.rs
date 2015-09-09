extern crate sphlib;
extern crate libc;

use sphlib::{sph_skein, utils};

#[test]
fn will_be_224_hash() {
    let dest   = sph_skein::skein224_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("1541ae9fc3ebe24eb758ccb1fd60c2c31a9ebfe65b220086e7819e25", actual.to_string());
}

#[test]
fn will_be_256_hash() {
    let dest   = sph_skein::skein256_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("39ccc4554a8b31853b9de7a1fe638a24cce6b35a55f2431009e18780335d2621", actual.to_string());
}

#[test]
fn will_be_384_hash() {
    let dest   = sph_skein::skein384_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("dd5aaf4589dc227bd1eb7bc68771f5baeaa3586ef6c7680167a023ec8ce26980f06c4082c488b4ac9ef313f8cbe70808", actual.to_string());
}

#[test]
fn will_be_512_hash() {
    let dest   = sph_skein::skein512_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("bc5b4c50925519c290cc634277ae3d6257212395cba733bbad37a4af0fa06af41fca7903d06564fea7a2d3730dbdb80c1f85562dfcc070334ea4d1d9e72cba7a", actual.to_string());
}
