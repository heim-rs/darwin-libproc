//! Native `libproc` bindings for Rust.
//!
//! This crate provides the raw `libproc` API exposing kernel data about processes of macOS.

#![cfg(target_os = "macos")]
#![doc(html_root_url = "https://docs.rs/darwin-libproc-sys/0.0.1")]
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

extern "C" {
    pub fn proc_listpids(
        r#type: u32,
        typeinfo: u32,
        buffer: *mut libc::c_void,
        buffersize: libc::c_int,
    ) -> libc::c_int;

    pub fn proc_listpids_path(
        r#type: u32,
        typeinfo: u32,
        path: *const libc::c_char,
        pathflags: u32,
        buffer: *mut libc::c_void,
        buffersize: libc::c_int,
    ) -> libc::c_int;

    pub fn proc_pidinfo(
        pid: libc::c_int,
        flavor: libc::c_int,
        arg: u64,
        buffer: *mut libc::c_void,
        buffersize: libc::c_int,
    ) -> libc::c_int;

    pub fn proc_pidfdinfo(
        pid: libc::c_int,
        fd: libc::c_int,
        flavor: libc::c_int,
        buffer: *mut libc::c_void,
        buffersize: libc::c_int,
    ) -> libc::c_int;

    pub fn proc_name(
        pid: libc::c_int,
        buffer: *mut libc::c_void,
        buffersize: u32,
    ) -> libc::c_int;

    pub fn proc_regionfilename(
        pid: libc::c_int,
        address: u64,
        buffer: *mut libc::c_void,
        buffersize: u32,
    ) -> libc::c_int;

    pub fn proc_kmsgbuf(
        buffer: *mut libc::c_void,
        buffersize: u32,
    ) -> libc::c_int;

    pub fn proc_pidpath(
        pid: libc::c_int,
        buffer: *mut libc::c_void,
        buffersize: u32,
    ) -> libc::c_int;

    pub fn proc_libversion(
        major: *mut libc::c_int,
        minor: *mut libc::c_int,
    ) -> libc::c_int;
}
