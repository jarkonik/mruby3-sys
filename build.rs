extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    Command::new("make")
        .env("CFLAGS", "-fPIE")
        .args(&["-C", "mruby"])
        .env("INSTALL_DIR", out_path.clone())
        .env("MRUBY_BUILD_DIR", out_path.clone())
        .status()
        .expect("mruby make failed");

    println!(
        "cargo:rustc-link-search={}/host/lib",
        out_path.to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=mruby");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I./mruby/include")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Cleanup lock file created by ruby build process.
    // Build scripts for Rust crates should not modify anything outside of OUT_DIR.
    Command::new("rm")
        .arg("mruby/build_config/default.rb.lock")
        .status()
        .expect("cleaning lock file failed");

    Command::new("patch")
        .args(&[out_path.join("bindings.rs"), "bindings.patch".into()])
        .status()
        .expect("patching bindings.rs failed");
}
