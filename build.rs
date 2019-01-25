extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // We don't have any link dependencies
    //println!("cargo:rustc-link-lib=bz2");
    println!("cargo:rustc-env=TARGET=thumbv7em-none-eabihf");
    let bindings = bindgen::Builder::default()
        .clang_arg("-I/usr/include")
        .use_core()
        .ctypes_prefix("cty")
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
