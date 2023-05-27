use integ_test_example::get_process_id;

#[test]
fn test_if_process_id_is_returned() {
    assert!(get_process_id() > 0)
}

/// 仅运行被忽略的测试
/// cargo test -- --ignored
#[test]
#[ignore]
fn process_id() {
    assert!(get_process_id() > 0)
}
