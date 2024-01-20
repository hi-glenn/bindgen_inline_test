// include!("bindings.rs");
extern "C" {
    fn add(x: i32, y: i32) -> i32;
}

#[test]
fn test_add() {
    unsafe {
        let result = add(2, 3);
        println!("Result: {}", result);
    }

    // let r1 = unsafe { inc(1) };
    // assert_eq!(r1, 2);
}
