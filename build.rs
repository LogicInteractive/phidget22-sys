extern crate bindgen;

use std::env;
use std::path::Path;

fn main() {
    println!(
        "cargo:rustc-link-search={}",
        Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("thirdparty")
            .join("libphidget22")
            .join("windows")
            .join("x64")
            .display()
    );
    println!("cargo:rustc-link-lib=phidget22");

    bindgen::Builder::default()
        .clang_arg("-Ithirdparty/libphidget22/windows")
        .header("src/wrapper.h")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");

    println!("cargo:rerun-if-changed=thirdparty/libphidget22/windows/phidget22.h");

    let input_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("thirdparty")
        .join("libphidget22")
        .join("windows")
        .join("x64")
        .join("phidget22.dll");
    let output_path = Path::new(&env::var("OUT_DIR").unwrap()).join("phidget22.dll");
    let _ = std::fs::copy(input_path, output_path);

    println!(
        "cargo:rustc-link-search=native={}",
        &env::var("OUT_DIR").unwrap()
    );
}
