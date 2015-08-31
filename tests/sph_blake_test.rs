extern crate sphlib;
extern crate libc;

use sphlib::{sph_blake, utils};


#[test]
fn will_be_224_hash() {
    let dest   = sph_blake::blake224("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("7dc5313b1c04512a174bd6503b89607aecbee0903d40a8a569c94eed", actual.to_string());
}

#[test]
fn will_be_256_hash() {
    let dest   = sph_blake::blake256("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("716f6e863f744b9ac22c97ec7b76ea5f5908bc5b2f67c61510bfc4751384ea7a", actual.to_string());
}

#[test]
fn will_be_384_hash() {
    let dest   = sph_blake::blake384("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("c6cbd89c926ab525c242e6621f2f5fa73aa4afe3d9e24aed727faaadd6af38b620bdb623dd2b4788b1c8086984af8706", actual.to_string());
}

#[test]
fn will_be_512_hash() {
    let dest   = sph_blake::blake512("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("a8cfbbd73726062df0c6864dda65defe58ef0cc52a5625090fa17601e1eecd1b628e94f396ae402a00acc9eab77b4d4c2e852aaaa25a636d80af3fc7913ef5b8", actual.to_string());
}
