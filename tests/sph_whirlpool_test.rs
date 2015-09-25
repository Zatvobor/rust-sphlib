extern crate sphlib;
extern crate libc;

use sphlib::{sph_whirlpool, utils};

#[test]
fn will_be_hash() {
    let dest   = sph_whirlpool::whirlpool_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("19fa61d75522a4669b44e39c1d2e1726c530232130d407f89afee0964997f7a73e83be698b288febcf88e3e03c4f0757ea8964e59b63d93708b138cc42a66eb3", actual.to_string());
}

#[test]
fn will_be_0_hash() {
    let dest   = sph_whirlpool::whirlpool0_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("b3e1ab6eaf640a34f784593f2074416accd3b8e62c620175fca0997b1ba2347339aa0d79e754c308209ea36811dfa40c1c32f1a2b9004725d987d3635165d3c8", actual.to_string());
}

#[test]
fn will_be_1_hash() {
    let dest   = sph_whirlpool::whirlpool1_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("470f0409abaa446e49667d4ebe12a14387cedbd10dd17b8243cad550a089dc0feea7aa40f6c2aaab71c6ebd076e43c7cfca0ad32567897dcb5969861049a0f5a", actual.to_string());
}
