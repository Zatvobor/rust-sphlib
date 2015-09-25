extern crate sphlib;
extern crate libc;

use sphlib::{sph_tiger, utils};

#[test]
fn will_be_hash() {
    let dest   = sph_tiger::tiger_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("3293ac630c13f0245f92bbb1766e16167a4e58492dde73f3", actual.to_string());
}

// #[test]
// fn will_be_2_hash() {
//     let dest   = sph_tiger::tiger2_init_load_close("");
//     let actual = utils::to_hex_hash(&dest);
//     assert_eq!("4441be75f6018773c206c22745374b924aa8313fef919f41", actual.to_string());
// }
