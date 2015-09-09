extern crate sphlib;
extern crate libc;

use sphlib::{sph_jh, utils};

#[test]
fn will_be_224_hash() {
    let dest   = sph_jh::jh224_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("2c99df889b019309051c60fecc2bd285a774940e43175b76b2626630", actual.to_string());
}

#[test]
fn will_be_256_hash() {
    let dest   = sph_jh::jh256_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("46e64619c18bb0a92a5e87185a47eef83ca747b8fcc8e1412921357e326df434", actual.to_string());
}

#[test]
fn will_be_384_hash() {
    let dest   = sph_jh::jh384_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("2fe5f71b1b3290d3c017fb3c1a4d02a5cbeb03a0476481e25082434a881994b0ff99e078d2c16b105ad069b569315328", actual.to_string());
}

#[test]
fn will_be_512_hash() {
    let dest   = sph_jh::jh512_init_load_close("");
    let actual = utils::to_hex_hash(&dest);
    assert_eq!("90ecf2f76f9d2c8017d979ad5ab96b87d58fc8fc4b83060f3f900774faa2c8fabe69c5f4ff1ec2b61d6b316941cedee117fb04b1f4c5bc1b919ae841c50eec4f", actual.to_string());
}
