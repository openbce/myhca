extern crate bindgen;

use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=pci");
    println!("cargo:rerun-if-changed=wrappers/*");

    // Build binding builder
    let bindings = bindgen::Builder::default()
        .header("wrappers/pci.h")
        .blacklist_type("u8")
        .blacklist_type("u16")
        .blacklist_type("u32")
        .blacklist_type("u64")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the src/pci.rs file.
    let out_path = PathBuf::from("src/wrappers".to_string());
    bindings
        .write_to_file(out_path.join("pci.rs"))
        .expect("Couldn't write bindings!");
}
