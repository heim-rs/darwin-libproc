#[test]
fn test_pid_path() {
    assert!(darwin_libproc::pid_path(1).is_ok());
}
