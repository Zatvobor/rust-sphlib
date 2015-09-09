extern crate sphlib;
extern crate libc;

use sphlib::{sph_echo, utils};

#[test]
fn will_be_224_hash() {
    let dest   = sph_echo::echo224_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("17da087595166f733fff7cdb0bca6438f303d0e00c48b5e7a3075905", actual.to_string());
}

#[test]
fn will_be_256_hash() {
    let dest   = sph_echo::echo256_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("4496cd09d425999aefa75189ee7fd3c97362aa9e4ca898328002d20a4b519788", actual.to_string());
}

#[test]
fn will_be_384_hash() {
    let dest   = sph_echo::echo384_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("134040763f840559b84b7a1ae5d6d64fc3659821a789cc64a7f1444c09ee7f81a54d72beee8273bae5ef18ec43aa5f34", actual.to_string());
}

#[test]
fn will_be_512_hash() {
    let dest   = sph_echo::echo512_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("158f58cc79d300a9aa292515049275d051a28ab931726d0ec44bdd9faef4a702c36db9e7922fff077402236465833c5cc76af4efc352b4b44c7fa15aa0ef234e", actual.to_string());
}
