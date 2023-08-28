use std::env;
use std::path::Path;
fn main() {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let target_vendor = env::var("CARGO_CFG_TARGET_VENDOR").unwrap_or_default();

    let mut build256 = cc::Build::new();
    let (sha256_path, sha512_path) = if target_arch == "x86" {
        (
            Path::new("src").join("sha256_x86.S"),
            Path::new("src").join("sha512_x86.S"),
        )
    } else if target_arch == "x86_64" {
        (
            Path::new("src").join("sha256_x64.S"),
            Path::new("src").join("sha512_x64.S"),
        )
    } else if target_arch == "aarch64" && target_vendor == "apple" {
        build256.flag("-march=armv8-a+crypto");
        (
            Path::new("src").join("sha256_aarch64_apple.S"),
            Default::default(),
        )
    } else if target_arch == "aarch64" {
        build256.flag("-march=armv8-a+crypto");
        (
            Path::new("src").join("sha256_aarch64.S"),
            Default::default(),
        )
    } else {
        panic!("Unsupported target architecture");
    };

    if target_arch != "aarch64" {
        cc::Build::new()
            .flag("-c")
            .file(sha512_path)
            .compile("libsha512.a");
    }
    build256.flag("-c").file(sha256_path).compile("sha256");
}
