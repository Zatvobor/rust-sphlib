extern crate sphlib;
extern crate libc;

use sphlib::{sph_fugue, utils};

#[test]
fn will_be_224_hash() {
    let dest   = sph_fugue::fugue224_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("e2cd30d51a913c4ed2388a141f90caa4914de43010849e7b8a7a9ccd", actual.to_string());
}

#[test]
fn will_be_256_hash() {
    let dest   = sph_fugue::fugue256_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("d6ec528980c130aad1d1acd28b9dd8dbdeae0d79eded1fca72c2af9f37c2246f", actual.to_string());
}

#[test]
fn will_be_384_hash() {
    let dest   = sph_fugue::fugue384_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("466d05f6812b58b8628e53816b2a99d173b804a964de971829159c3791ac8b524eebbf5fc73ba40ea8eea446d5424a30", actual.to_string());
}

#[test]
fn will_be_512_hash() {
    let dest   = sph_fugue::fugue512_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("3124f0cbb5a1c2fb3ce747ada63ed2ab3bcd74795cef2b0e805d5319fcc360b4617b6a7eb631d66f6d106ed0724b56fa8c1110f9b8df1c6898e7ca3c2dfccf79", actual.to_string());
}
