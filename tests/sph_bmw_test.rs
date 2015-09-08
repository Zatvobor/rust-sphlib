extern crate sphlib;
extern crate libc;

use sphlib::{sph_bmw, utils};

#[test]
fn will_be_224_hash() {
    let dest   = sph_bmw::bmw224_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("e57c183da7e2cd3e90258ca04499b222420f9b6797bbab131b4d286e", actual.to_string());
}

#[test]
fn will_be_256_hash() {
    let dest   = sph_bmw::bmw256_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("82cac4bf6f4c2b41fbcc0e0984e9d8b76d7662f8e1789cdfbd85682acc55577a", actual.to_string());
}

#[test]
fn will_be_384_hash() {
    let dest   = sph_bmw::bmw384_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("1db2643911391720e712a8c24457ee456fabfd555f479156e4b24278d6f6bcfb03fab1ec2a2626b79f2880216bc29b29", actual.to_string());
}

#[test]
fn will_be_512_hash() {
    let dest   = sph_bmw::bmw512_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("6a725655c42bc8a2a20549dd5a233a6a2beb01616975851fd122504e604b46af7d96697d0b6333db1d1709d6df328d2a6c786551b0cce2255e8c7332b4819c0e", actual.to_string());
}
