extern crate sphlib;
extern crate libc;

use sphlib::{sph_simd, utils};

#[test]
fn will_be_224_hash() {
    let dest   = sph_simd::simd224_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("43e1d53656d7b85d10d5499e28afdef90bb497730d2853c8609b534b", actual.to_string());
}

#[test]
fn will_be_256_hash() {
    let dest   = sph_simd::simd256_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("8029e81e7320e13ed9001dc3d8021fec695b7a25cd43ad805260181c35fcaea8", actual.to_string());
}

#[test]
fn will_be_384_hash() {
    let dest   = sph_simd::simd384_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("5fdd62778fc213221890ad3bac742a4af107ce2692d6112e795b54b25dcd5e0f4bf3ef1b770ab34b38f074a5e0ecfcb5", actual.to_string());
}

#[test]
fn will_be_512_hash() {
    let dest   = sph_simd::simd512_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("51a5af7e243cd9a5989f7792c880c4c3168c3d60c4518725fe5757d1f7a69c6366977eaba7905ce2da5d7cfd07773725f0935b55f3efb954996689a49b6d29e0", actual.to_string());
}
