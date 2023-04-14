use std::collections::HashMap;
use std::process::exit;
use g_rust::extension::extension::Extension;
use g_rust::extension::parsers::incoming::{Chat, Users};
use g_rust::extension::parsers::subparsers::User;
use g_rust::protocol::hmessage::HMessage;

#[derive(Debug, Default)]
struct ChatLogger {
    user_name_by_index: HashMap<i32, String>
}

fn main() {
    let mut chat_logger: Extension<ChatLogger> = Extension::new();
    chat_logger.intercept(on_users);
    chat_logger.intercept(on_chat);
    chat_logger.run();
}

fn on_users(ext: &mut Extension<ChatLogger>, _msg: &mut HMessage, users: &mut Users) {
    for user in users.users.iter() {
        let (index, name) = match user {
            User::Player { room_index, name, .. } => (room_index, name),
            User::Pet { room_index, name, .. } => (room_index, name),
            User::OldBot { room_index, name, .. } => (room_index, name),
            User::Bot { room_index, name, .. } => (room_index, name),
        };
        ext.globals.user_name_by_index.insert(*index, name.clone());
    }
}

fn on_chat(ext: &mut Extension<ChatLogger>, _msg: &mut HMessage, chat: &mut Chat) {
    let name = ext.globals.user_name_by_index
        .get(&chat.user_index)
        .unwrap_or(&String::from("<Unknown>"))
        .clone();
    println!("{name}: {}", chat.text);
}