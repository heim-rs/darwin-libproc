#[test]
fn test_all_pids() {
    let me = unsafe { libc::getpid() };
    let result = darwin_libproc::list_fds(me);

    assert!(result.is_ok());
    let pids = result.unwrap();
    assert!(pids.len() > 0);
}
