include!("bindings.rs");

#[test]
fn test_inc() {
    let r1 = unsafe { inc(1) };

    assert_eq!(r1, 2);
}
