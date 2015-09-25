extern crate sphlib;
extern crate libc;

use sphlib::{sph_md5, utils};

#[test]
fn will_be_hash() {
    let dest   = sph_md5::md5_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("d41d8cd98f00b204e9800998ecf8427e", actual.to_string());
}
