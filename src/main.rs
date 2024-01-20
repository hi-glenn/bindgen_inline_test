// include!("bindings.rs");
extern "C" {
    fn add(x: i32, y: i32) -> i32;
}

fn main(){
    unsafe {
        let result = add(2, 3);
        println!("Result: {}", result);
    }
}

// #[test]
// fn test_add() {
//     unsafe {
//         let result = add(2, 3);
//         println!("Result: {}", result);
//     }
// }
