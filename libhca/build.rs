extern crate bindgen;

use std::path::PathBuf;

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search=c");

    println!("cargo:rustc-link-lib=pci");
    println!("cargo:rustc-link-lib=hca");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=c/hca.h");
    println!("cargo:rerun-if-changed=c/hca.c");

    cc::Build::new().file("c/hca.c").compile("libhca.a");

    // Build binding builder
    let bindings = bindgen::Builder::default()
        .header("c/hca.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the src/pci.rs file.
    let out_path = PathBuf::from("src".to_string());
    bindings
        .write_to_file(out_path.join("pci.rs"))
        .expect("Couldn't write bindings!");
}
