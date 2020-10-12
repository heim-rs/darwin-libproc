use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=dylib=proc");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .whitelist_type("proc_.*")
        .whitelist_function("proc_.*")
        .whitelist_var("proc_.*")
        .whitelist_var("PROC_.*")
        .whitelist_type("rusage_.*")
        .whitelist_var("RUSAGE_.*")
        .layout_tests(true)
        .generate()
        .expect("unable to generate libproc bindings");

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out.join("bindings.rs"))
            .expect("unable to write bindings")
}
