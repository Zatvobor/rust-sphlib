extern crate sphlib;
extern crate libc;

use sphlib::{sph_panama, utils};

#[test]
fn will_be_hash() {
    let dest   = sph_panama::panama_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("aa0cc954d757d7ac7779ca3342334ca471abd47d5952ac91ed837ecd5b16922b", actual.to_string());
}
