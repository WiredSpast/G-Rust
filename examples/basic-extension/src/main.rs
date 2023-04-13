use g_rust::extension::extension::Extension;
use g_rust::extension::parsers::incoming::Chat;
use g_rust::protocol::hmessage::HMessage;

fn main() {
    let mut ext = Extension::new();
    ext.info.name = String::from("G-Rust Test Extension");
    ext.intercept(on_chat);
    ext.run();
}

fn on_chat(_ext: &mut Extension, _msg: &mut HMessage, chat: &mut Chat) {
    println!("{}", chat.text);
    chat.text = String::from("G-Rust says hi");
}