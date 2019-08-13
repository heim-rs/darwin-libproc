//! Flavors for proc_pidinfo()
//!
//! See `bsd/sys/proc_info.h` for originals

use std::mem;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_bsdinfo {
    // 64bit; emulated etc
    pub pbi_flags: u32,
    pub pbi_status: u32,
    pub pbi_xstatus: u32,
    pub pbi_pid: u32,
    pub pbi_ppid: u32,
    pub pbi_uid: libc::uid_t,
    pub pbi_gid: libc::gid_t,
    pub pbi_ruid: libc::uid_t,
    pub pbi_rgid: libc::gid_t,
    pub pbi_svuid: libc::uid_t,
    pub pbi_svgid: libc::gid_t,
    pub pbi_comm: [libc::c_char; libc::MAXCOMLEN + 1],
    // Empty if no name is registered
    pub pbi_name: [libc::c_char; 2 * libc::MAXCOMLEN + 1],
    pub pbi_nfiles: u32,
    pub pbi_pgid: u32,
    pub pbi_pjobc: u32,
    // Controlling tty dev
    pub e_tdev: u32,
    // tty process group id
    pub e_tpgid: u32,
    pub pbi_start: libc::timeval,
    pub pbi_nice: i32,
}

pub const PROC_PIDLISTFDS: libc::c_int = 1;
pub const PROC_PIDLISTFD_SIZE: libc::c_int = mem::size_of::<proc_bsdinfo>() as libc::c_int;

//#define PROC_PIDTASKALLINFO 2
//#define PROC_PIDTASKALLINFO_SIZE  (sizeof(struct proc_taskallinfo))
//
//#define PROC_PIDTBSDINFO 3
//#define PROC_PIDTBSDINFO_SIZE  (sizeof(struct proc_bsdinfo))
//
//#define PROC_PIDTASKINFO 4
//#define PROC_PIDTASKINFO_SIZE  (sizeof(struct proc_taskinfo))
//
//#define PROC_PIDTHREADINFO 5
//#define PROC_PIDTHREADINFO_SIZE  (sizeof(struct proc_threadinfo))
//
//#define PROC_PIDLISTTHREADS 6
//#define PROC_PIDLISTTHREADS_SIZE  (2* sizeof(uint32_t))
//
//
//#define PROC_PIDREGIONINFO 7
//#define PROC_PIDREGIONINFO_SIZE  (sizeof(struct proc_regioninfo))
//
//#define PROC_PIDREGIONPATHINFO 8
//#define PROC_PIDREGIONPATHINFO_SIZE  (sizeof(struct proc_regionwithpathinfo))
//
//#define PROC_PIDVNODEPATHINFO 9
//#define PROC_PIDVNODEPATHINFO_SIZE  (sizeof(struct proc_vnodepathinfo))
//
//#define PROC_PIDTHREADPATHINFO 10
//#define PROC_PIDTHREADPATHINFO_SIZE  (sizeof(struct proc_threadwithpathinfo))
//
//#define PROC_PIDPATHINFO 11
//#define PROC_PIDPATHINFO_SIZE  (MAXPATHLEN)
//#define PROC_PIDPATHINFO_MAXSIZE  (4*MAXPATHLEN)

extern "C" {
    pub fn proc_pidinfo(
        pid: libc::c_int,
        flavor: libc::c_int,
        arg: u64,
        buffer: *mut libc::c_void,
        buffersize: libc::c_int,
    ) -> libc::c_int;
}
