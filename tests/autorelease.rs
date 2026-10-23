use foundation::autoreleasepool;

#[test]
fn returns_the_value_of_the_body() {
    assert_eq!(autoreleasepool(|| 7), 7);
}

#[test]
fn nested_pools_drain_in_order() {
    let value = autoreleasepool(|| autoreleasepool(|| "inner").len());
    assert_eq!(value, 5);
}

#[test]
fn a_panic_in_the_body_drains_the_pool() {
    let result = std::panic::catch_unwind(|| autoreleasepool(|| panic!("stop")));
    assert!(result.is_err());
    assert_eq!(autoreleasepool(|| 1), 1);
}
