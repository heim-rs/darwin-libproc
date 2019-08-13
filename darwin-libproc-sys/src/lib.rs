//! Native `libproc` bindings for Rust.
//!
//! This crate provides the raw `libproc` API exposing kernel data about processes of macOS.

#![cfg(target_os = "macos")]
#![doc(html_root_url = "https://docs.rs/darwin-libproc-sys/0.0.1")]
#![deny(
    unused,
    unused_imports,
    future_incompatible,
    missing_debug_implementations,
    nonstandard_style,
    dead_code,
    deprecated,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_results
)]
#![allow(missing_docs)]

// Declared at `bsd/sys/param.h`
pub const MAXPATHLEN: usize = libc::PATH_MAX as usize;
// Declared at `bsd/sys/proc_info.h`
pub const PROC_PIDPATHINFO_MAXSIZE: usize = 4 * MAXPATHLEN;

// Declared at `bsd/sys/proc_info.h`,
// ex. http://fxr.watson.org/fxr/source/bsd/sys/proc_info.h?v=xnu-2050.18.24#L48
/// Return all processes.
pub const PROC_ALL_PIDS: u32 = 1;
/// Return all processes in a given group.
pub const PROC_PGRP_ONLY: u32 = 2;
/// Return all processes attached to a given TTY.
pub const PROC_TTY_ONLY: u32 = 3;
/// Return all processes with the given UID.
pub const PROC_UID_ONLY: u32 = 4;
/// Return all processes with the given RUID.
pub const PROC_RUID_ONLY: u32 = 5;
/// Return all processes with the given PPID.
pub const PROC_PPID_ONLY: u32 = 6;

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
