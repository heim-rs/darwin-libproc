//! Idiomatic and safe wrappers for `libproc` of macOS.

#![cfg(target_os = "macos")]
#![doc(html_root_url = "https://docs.rs/darwin-libproc/0.1.0")]
#![deny(
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

mod list_pids;
mod name;
mod pid_path;
mod version;

pub use self::list_pids::{
    all_pids, pgrp_only_pids, ppid_only_pids, ruid_only_pids, tty_only_pids,
    uid_only_pids,
};
pub use self::name::name;
pub use self::pid_path::pid_path;
pub use self::version::version;
