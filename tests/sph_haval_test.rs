extern crate sphlib;
extern crate libc;

use sphlib::{sph_haval, utils};

#[test]
fn will_be_128_3_hash() {
    let dest   = sph_haval::haval128_3_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("c68f39913f901f3ddf44c707357a7d70", actual.to_string());
}

#[test]
fn will_be_160_3_hash() {
    let dest   = sph_haval::haval160_3_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("d353c3ae22a25401d257643836d7231a9a95f953", actual.to_string());
}

#[test]
fn will_be_192_3_hash() {
    let dest   = sph_haval::haval192_3_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("e9c48d7903eaf2a91c5b350151efcb175c0fc82de2289a4e", actual.to_string());
}

#[test]
fn will_be_224_3_hash() {
    let dest   = sph_haval::haval224_3_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("c5aae9d47bffcaaf84a8c6e7ccacd60a0dd1932be7b1a192b9214b6d", actual.to_string());
}

#[test]
fn will_be_256_3_hash() {
    let dest   = sph_haval::haval256_3_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("4f6938531f0bc8991f62da7bbd6f7de3fad44562b8c6f4ebf146d5b4e46f7c17", actual.to_string());
}

#[test]
fn will_be_128_4_hash() {
    let dest   = sph_haval::haval128_4_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("ee6bbf4d6a46a679b3a856c88538bb98", actual.to_string());
}

#[test]
fn will_be_160_4_hash() {
    let dest   = sph_haval::haval160_4_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("1d33aae1be4146dbaaca0b6e70d7a11f10801525", actual.to_string());
}

#[test]
fn will_be_192_4_hash() {
    let dest   = sph_haval::haval192_4_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("4a8372945afa55c7dead800311272523ca19d42ea47b72da", actual.to_string());
}

#[test]
fn will_be_224_4_hash() {
    let dest   = sph_haval::haval224_4_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("3e56243275b3b81561750550e36fcd676ad2f5dd9e15f2e89e6ed78e", actual.to_string());
}

#[test]
fn will_be_256_4_hash() {
    let dest   = sph_haval::haval256_4_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("c92b2e23091e80e375dadce26982482d197b1a2521be82da819f8ca2c579b99b", actual.to_string());
}

#[test]
fn will_be_128_5_hash() {
    let dest   = sph_haval::haval128_5_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("184b8482a0c050dca54b59c7f05bf5dd", actual.to_string());
}

#[test]
fn will_be_160_5_hash() {
    let dest   = sph_haval::haval160_5_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("255158cfc1eed1a7be7c55ddd64d9790415b933b", actual.to_string());
}

#[test]
fn will_be_192_5_hash() {
    let dest   = sph_haval::haval192_5_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("4839d0626f95935e17ee2fc4509387bbe2cc46cb382ffe85", actual.to_string());
}

#[test]
fn will_be_224_5_hash() {
    let dest   = sph_haval::haval224_5_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("4a0513c032754f5582a758d35917ac9adf3854219b39e3ac77d1837e", actual.to_string());
}

#[test]
fn will_be_256_5_hash() {
    let dest   = sph_haval::haval256_5_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("be417bb4dd5cfb76c7126f4f8eeb1553a449039307b1a3cd451dbfdc0fbbe330", actual.to_string());
}
