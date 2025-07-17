use std::env;
use std::process::Command;

fn main() {
    // Only apply for Windows builds
    if env::var("CARGO_CFG_TARGET_OS").unwrap() != "windows" {
        return;
    }

    // Rebuild only if these change
    println!("cargo:rerun-if-changed=ytdownloader.rc");
    println!("cargo:rerun-if-changed=icon.ico");

    // Compile ytdownloader.rc into .res using windres
    let status = Command::new("x86_64-w64-mingw32-windres")
    .args([
        "ytdownloader.rc",
        "-O", "coff",
        "-o", "ytdownloader.res",
    ])
    .status()
    .expect("Failed to run windres");

    assert!(status.success(), "windres failed");

    // Link the .res file into the binary
    println!("cargo:rustc-link-arg-bin=ytdownloader=ytdownloader.res");
}
