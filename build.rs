extern crate bindgen;
use std::env;
use std::path::PathBuf;

fn main() {
    pkg_config::probe_library("libldns").unwrap();
    println!("cargo:rustc-link-lib=dylib=ldns");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // Refer to https://www.nlnetlabs.nl/documentation/ldns/annotated.html
    let bindings = bindgen::Builder::default()
        //TODO: use pkg-config to determine includes for `clang`
        .clang_arg("-I/usr/local/include")
        .header("wrapper.h")
        .allowlist_type("_ldns_sha256_CTX")
        .allowlist_type("_ldns_sha2_buffer_union")
        .allowlist_type("_ldns_sha512_CTX")
        .allowlist_type("dnssec_zone_rr_iter")
        .allowlist_type("ldns_.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
