extern crate sphlib;
extern crate libc;

use sphlib::{sph_md4, utils};

#[test]
fn will_be_hash() {
    let dest   = sph_md4::md4_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("31d6cfe0d16ae931b73c59d7e0c089c0", actual.to_string());
}
