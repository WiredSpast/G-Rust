use std::thread;
use g_rust::extension::extension::{Extension};

// #[test]
// fn extension() {
//     let ext = Extension::new();
//     println!("a");
//     ext.run();
//     println!("b");
// }

#[test]
fn testtt() {
    println!("a");
    thread::spawn(move || {
        println!("b");
    });
    println!("c");
}

#[test]
fn test_connection() {
    let ext = Extension::new_custom_args(vec!["-p".to_string(), "9092".to_string()]);
    ext.run();
}