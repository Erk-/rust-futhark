extern crate cc;
extern crate bindgen;

use std::path::PathBuf;
use std::process::Command;

fn main() {
    Command::new("futhark-opencl")
        .arg("--library")
        .arg("./dotprod.fut")
        .output()
        .expect("failed to execute process");
    
    let bindings = bindgen::Builder::default()
        .header("dotprod.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("./src/");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    cc::Build::new()
        .file("dotprod.c")
        .flag("-fPIC")
        .flag("-w")
        .shared_flag(true)
        .compile("libdotprod.so");
    println!("cargo:rustc-link-lib=OpenCL");
}
