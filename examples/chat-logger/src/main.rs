use std::collections::HashMap;
use std::process::exit;
use g_rust::extension::extension::Extension;
use g_rust::extension::parsers::incoming::{Chat, CloseConnection, UserRemove, Users};
use g_rust::extension::parsers::subparsers::User;
use g_rust::protocol::hmessage::HMessage;

#[derive(Debug, Default)]
struct ChatLogger {
    user_name_by_index: HashMap<i32, String>
}

fn main() {
    let mut chat_logger: Extension<ChatLogger> = Extension::new();
    chat_logger.info.name = String::from("Chat Logger");
    chat_logger.intercept(on_users);
    chat_logger.intercept(on_close_connection);
    chat_logger.intercept(on_user_remove);
    chat_logger.intercept(on_chat);
    chat_logger.run();
}

fn on_users(ext: &mut Extension<ChatLogger>, _: &mut HMessage, users: &mut Users) {
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

fn on_close_connection(ext: &mut Extension<ChatLogger>, _: &mut HMessage, _: &mut CloseConnection) {
    ext.globals.user_name_by_index.clear();
}

fn on_user_remove(ext: &mut Extension<ChatLogger>, _: &mut HMessage, user_remove: &mut UserRemove) {
    ext.globals.user_name_by_index.remove(&(*user_remove.id as i32));
}

fn on_chat(ext: &mut Extension<ChatLogger>, _: &mut HMessage, chat: &mut Chat) {
    let name = ext.globals.user_name_by_index
        .get(&chat.user_index)
        .unwrap_or(&String::from("<Unknown>"))
        .clone();
    println!("{name}: {}", chat.text);
}