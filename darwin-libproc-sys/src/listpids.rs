//! Flavors for proc_listpids()

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
}
