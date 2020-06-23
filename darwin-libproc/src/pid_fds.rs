use std::io;

use crate::pid_info::pid_info_list;

/// Returns filled `Vec` or `proc_fdinfo` structs for `pid` given.
pub fn list_fds(
    pid: libc::pid_t,
) -> io::Result<Vec<darwin_libproc_sys::proc_fdinfo>> {
    pid_info_list(pid, darwin_libproc_sys::PROC_PIDLISTFDS, 0)
}
