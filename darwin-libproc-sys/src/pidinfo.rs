//! Flavors for proc_pidinfo()
//!
//! See `bsd/sys/proc_info.h` for originals

use std::mem;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct proc_bsdinfo {
    /// 64bit; emulated etc
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
    /// reserved
    pub rfu_1: u32,
    pub pbi_comm: [libc::c_char; libc::MAXCOMLEN],
    /// Empty if no name is registered
    pub pbi_name: [libc::c_char; 2 * libc::MAXCOMLEN],
    pub pbi_nfiles: u32,
    pub pbi_pgid: u32,
    pub pbi_pjobc: u32,
    /// Controlling tty dev
    pub e_tdev: u32,
    /// tty process group id
    pub e_tpgid: u32,
    pub pbi_nice: i32,
    pub pbi_start_tvsec: u64,
    pub pbi_start_tvusec: u64,
}

//struct proc_bsdshortinfo {
//        uint32_t                pbsi_pid;		/* process id */
//        uint32_t                pbsi_ppid;		/* process parent id */
//        uint32_t                pbsi_pgid;		/* process perp id */
//	uint32_t                pbsi_status;		/* p_stat value, SZOMB, SRUN, etc */
//	char                    pbsi_comm[MAXCOMLEN];	/* upto 16 characters of process name */
//	uint32_t                pbsi_flags;              /* 64bit; emulated etc */
//        uid_t                   pbsi_uid;		/* current uid on process */
//        gid_t                   pbsi_gid;		/* current gid on process */
//        uid_t                   pbsi_ruid;		/* current ruid on process */
//        gid_t                   pbsi_rgid;		/* current tgid on process */
//        uid_t                   pbsi_svuid;		/* current svuid on process */
//        gid_t                   pbsi_svgid;		/* current svgid on process */
//        uint32_t                pbsi_rfu;		/* reserved for future use*/
//};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct proc_taskinfo {
    /// virtual memory size (bytes)
    pub pti_virtual_size: u64,
    /// resident memory size (bytes)
    pub pti_resident_size: u64,
    /// total time
    pub pti_total_user: u64,
    pub pti_total_system: u64,
    /// existing threads only
    pub pti_threads_user: u64,
    pub pti_threads_system: u64,
    /// default policy for new threads
    pub pti_policy: i32,
    /// number of page faults
    pub pti_faults: i32,
    /// number of actual pageins
    pub pti_pageins: i32,
    /// number of copy-on-write faults
    pub pti_cow_faults: i32,
    /// number of messages sent
    pub pti_messages_sent: i32,
    /// number of messages received
    pub pti_messages_received: i32,
    /// number of mach system calls
    pub pti_syscalls_mach: i32,
    /// number of unix system calls
    pub pti_syscalls_unix: i32,
    /// number of context switches
    pub pti_csw: i32,
    /// number of threads in the task
    pub pti_threadnum: i32,
    /// number of running threads
    pub pti_numrunning: i32,
    /// task priority
    pub pti_priority: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct proc_taskallinfo {
    pub pbsd: proc_bsdinfo,
    pub ptinfo: proc_taskinfo,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct proc_fdinfo {
    pub proc_fd: i32,
    pub proc_fdtype: u32,
}

pub const PROC_PIDLISTFDS: libc::c_int = 2;
pub const PROC_PIDLISTFD_SIZE: libc::c_int =
    mem::size_of::<proc_fdinfo>() as libc::c_int; // Should be 8

pub const PROC_PIDTASKALLINFO: libc::c_int = 2;
pub const PROC_PIDTASKALLINFO_SIZE: libc::c_int =
    mem::size_of::<proc_taskallinfo>() as libc::c_int; // Should be 232

pub const PROC_PIDTBSDINFO: libc::c_int = 2;
pub const PROC_PIDTBSDINFO_SIZE: libc::c_int =
    mem::size_of::<proc_bsdinfo>() as libc::c_int; // Should be 136

pub const PROC_PIDTASKINFO: libc::c_int = 4;
pub const PROC_PIDTASKINFO_SIZE: libc::c_int =
    mem::size_of::<proc_taskinfo>() as libc::c_int; // Should be 96

//#define PROC_PIDTHREADINFO 5
//#define PROC_PIDTHREADINFO_SIZE  (sizeof(struct proc_threadinfo))
//

pub const PROC_PIDLISTTHREADS: libc::c_int = 6;
pub const PROC_PIDLISTTHREADS_SIZE: libc::c_int =
    2 * mem::size_of::<u32>() as libc::c_int;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_size() {
        assert_eq!(mem::size_of::<proc_fdinfo>(), 8);
        assert_eq!(PROC_PIDLISTFD_SIZE, 8);

        assert_eq!(mem::size_of::<proc_taskallinfo>(), 232);
        assert_eq!(PROC_PIDTASKALLINFO_SIZE, 232);

        assert_eq!(mem::size_of::<proc_bsdinfo>(), 136);
        assert_eq!(PROC_PIDTBSDINFO_SIZE, 136);
    }
}
