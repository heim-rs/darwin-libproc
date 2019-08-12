#![cfg(target_os = "macos")]
#![doc(html_root_url = "https://docs.rs/darwin-libproc/0.0.1")]
#![warn(
    unused,
    unused_imports,
    future_incompatible,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    dead_code,
    deprecated,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_results
)]

mod version;

pub use self::version::version;