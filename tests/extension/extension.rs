use g_rust::extension::extension::{Extension};
use g_rust::extension::parsers::{incoming, outgoing};
use g_rust::extension::parsers::baseparser::BaseParser;
use g_rust::extension::parsers::incoming::UserUpdate;
use g_rust::misc::connectioninfo::ConnectionInfo;
use g_rust::misc::hostinfo::HostInfo;
use g_rust::protocol::hdirection::HDirection;
use g_rust::protocol::hmessage::HMessage;
use g_rust::protocol::hpacket::HPacket;

#[test]
fn test_connection() {
    let mut ext = Extension::new();
    ext.args = vec!["-p".to_string(), "9092".to_string()];
    ext.info.name = "G-Rust test";
    ext.on_init(on_init);
    ext.on_socket_disconnect(on_socket_disconnect);
    ext.on_connect(on_connect);
    ext.on_host_info_update(on_host_info_update);
    ext.on_click(on_click);
    ext.intercept_raw(incoming::Chat::get_direction(),"Chat", on_chat);
    ext.intercept(on_user_update);
    ext.run();
}

fn on_init(ext: &mut Extension) {
    println!("Extension initialized");
    ext.request_flags(on_flags);
}

fn on_connect(ext: &mut Extension, info: ConnectionInfo) {
    println!("{info:?}");
    let packet_info_manager = ext.get_packet_info_manager().unwrap();
    // println!("Chatinfo: {packet_info_manager:?}");
    let chat_info = packet_info_manager.clone().get_packet_info_from_name(HDirection::ToClient, "Chat".to_string());
    println!("Chatinfo: {chat_info:?}");
}

fn on_socket_disconnect(_ext: &mut Extension) {
    println!("Socket disconnected");
}

fn on_host_info_update(ext: &mut Extension, info: HostInfo) {
    println!("{ext:?}");
    println!("{info:?}");
    ext.info.name = "abc";
}

fn on_click(ext: &mut Extension) {
    ext.send(incoming::Chat {
        user_index: 0,
        text: "G-Rust says hi".to_string(),
        gesture: 0,
        style_id: 0,
        links: vec![],
        tracking_id: 2
    });
}

fn on_flags(ext: &mut Extension, flags: Vec<String>) {
    println!("Flags: {flags:?}");
}

fn on_chat(_ext: &mut Extension, msg: &mut HMessage) {
    let mut chat: incoming::Chat = msg.get_packet().read();
    chat.text = String::from("G-Rust says hi");
    msg.get_packet().replace(6, chat);
}

#[test]
fn test_intercept() {
    let mut ext = Extension::new();
    ext.args = vec!["-p".to_string(), "9092".to_string()];
    ext.info.name = "G-Rust test";
    ext.intercept(on_user_update);
    ext.run();
}

fn on_user_update(_ext: &mut Extension, msg: &mut HMessage, user_update: &mut UserUpdate) {
    println!("{user_update:?}");
}

#[test]
fn test_hdirection_as_byte() {
    println!("{}", HDirection::ToClient as u8);
    println!("{}", HDirection::ToServer as u8);
    println!("{}", HDirection::None as u8);
}

#[test]
fn test_get_packet_name() {
    println!("{}", incoming::Chat::get_packet_name());
    println!("{:?}", incoming::Chat::get_direction());
    println!("{}", outgoing::Chat::get_packet_name());
    println!("{:?}", outgoing::Chat::get_direction());
}