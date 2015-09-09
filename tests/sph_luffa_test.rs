extern crate sphlib;
extern crate libc;

use sphlib::{sph_luffa, utils};

#[test]
fn will_be_224_hash() {
    let dest   = sph_luffa::luffa224_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("dbb8665871f4154d3e4396aefbba417cb7837dd683c332ba6be87e02", actual.to_string());
}

#[test]
fn will_be_256_hash() {
    let dest   = sph_luffa::luffa256_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("dbb8665871f4154d3e4396aefbba417cb7837dd683c332ba6be87e02a2712d6f", actual.to_string());
}

#[test]
fn will_be_384_hash() {
    let dest   = sph_luffa::luffa384_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("117d3ad49024dfe2994f4e335c9b330b48c537a13a9b7fa465938e1a02ff862bcdf33838bc0f371b045d26952d3ea0c5", actual.to_string());
}

#[test]
fn will_be_512_hash() {
    let dest   = sph_luffa::luffa512_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("6e7de4501189b3ca58f3ac114916654bbcd4922024b4cc1cd764acfe8ab4b7805df133eab345ffdb1c414564c924f48e0a301824e2ac4c34bd4efde2e43da90e", actual.to_string());
}
