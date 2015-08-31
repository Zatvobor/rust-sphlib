extern crate gcc;

fn main() {
    let files = ["src/c/blake.c", "src/c/bmw.c", "src/c/cubehash.c", "src/c/echo.c", "src/c/fugue.c", "src/c/groestl.c", "src/c/hamsi.c", "src/c/haval.c", "src/c/jh.c", "src/c/keccak.c", "src/c/luffa.c", "src/c/md2.c", "src/c/md4.c", "src/c/md5.c", "src/c/panama.c", "src/c/radiogatun.c", "src/c/ripemd.c", "src/c/sha0.c", "src/c/sha1.c", "src/c/sha2.c", "src/c/sha2big.c", "src/c/shabal.c", "src/c/shavite.c", "src/c/simd.c", "src/c/skein.c", "src/c/tiger.c", "src/c/whirlpool.c"];
    // to control the programs and flags used for building,
    // see https://github.com/alexcrichton/gcc-rs#external-configuration-via-environment-variables
    gcc::compile_library("libsph.a", &files);
}
