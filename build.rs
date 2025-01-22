use std::{env, path::PathBuf};

fn main() {
    // these headers cause problems with circular includes, so we don't generate bindings for them
    let disallowed_headers = vec!["rastarole.h".into()];

    println!("cargo:rerun-if-changed=rasta-sys");

    let mut dst = cmake::build("librasta");
    dst.push("lib");

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=dylib=rasta");

    let mut bindings = bindgen::Builder::default().clang_arg("-Ilibrasta/src/include/");
    for header in std::fs::read_dir("librasta/src/include/rasta").expect("Failed to read directory")
    {
        let header = header.unwrap();
        if !disallowed_headers.contains(&header.file_name().to_string_lossy().into_owned()) {
            bindings = bindings.header(header.path().to_string_lossy());
        }
    }

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .generate()
        .unwrap()
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
