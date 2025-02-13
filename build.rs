use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=link.x");

    println!("cargo:rustc-link-search=main");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");

    // https://github.com/rust-embedded/cortex-m-quickstart/pull/95
    println!("cargo:rustc-link-arg-bins=--nmagic");

    Ok(())
}
