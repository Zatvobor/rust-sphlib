extern crate sphlib;
extern crate libc;

use sphlib::{sph_md2, utils};

#[test]
fn will_be_hash() {
    let dest   = sph_md2::md2_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("8350e5a3e24c153df2275c9f80692773", actual.to_string());
}
