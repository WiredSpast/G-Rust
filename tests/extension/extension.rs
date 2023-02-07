use g_rust::extension::extension::{Extension};
use g_rust::misc::hclient::HClient;

#[test]
fn test_connection() {
    let mut ext = Extension::new();
    ext.args = vec!["-p".to_string(), "9092".to_string()];
    ext.info.name = "G-Rust test";
    ext.on_init(on_init);
    ext.on_socket_disconnect(on_socket_disconnect);
    ext.run();
}

fn on_init() {
    println!("Extension initialized");
}

fn on_socket_disconnect() {
    println!("Socket disconnected");
}