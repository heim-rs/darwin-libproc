use std::env;
use std::path::PathBuf;

fn main() {
    match env::var("CARGO_CFG_TARGET_OS") {
        Ok(os) if os == "macos" => {}
        Ok(other) => panic!(
            "darwin_libproc_sys crate can't be compiled for {} target",
            other
        ),
        Err(e) => panic!("CARGO_CFG_TARGET_OS env var is not defined: {:?}", e),
    }

    println!("cargo:rustc-link-lib=dylib=proc");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .whitelist_function("proc_.*")
        .whitelist_var("proc_.*")
        .whitelist_var("PROC_.*")
        .whitelist_type("proc_.*")
        .whitelist_type("rusage_.*")
        .whitelist_var("RUSAGE_.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
