use std::collections::HashMap;
use std::env;
use std::fmt::{Debug, Error, Formatter};
use super::parsers::baseparser::BaseParser;
use crate::misc::connection::GEarthConnection;
use crate::misc::connectioninfo::ConnectionInfo;
use crate::misc::consoleformat::ConsoleColour;
use crate::misc::hclient::CUR_CLIENT;
use crate::misc::hostinfo::HostInfo;
use crate::misc::messages::*;
use crate::protocol::hdirection::HDirection;
use crate::protocol::hmessage::HMessage;
use crate::protocol::hpacket::HPacket;
use crate::protocol::vars::packetvariable::PacketVariable;
use crate::protocol::vars::longstring::LongString;
use crate::services::packetinfo::packetinfomanager::PacketInfoManager;

macro_rules! trigger_listeners {
    ($listener:expr$(, $args:expr)*) => {
        for listener in $listener.clone().iter() {
            (listener)($($args),*)
        }
    }
}

const PORT_FLAG: [&str; 2] = ["--port", "-p"];
const FILE_FLAG: [&str; 2] = ["--filename", "-f"];
const COOKIE_FLAG: [&str; 2] = ["--auth-token", "-c"];

#[derive(Debug, Clone)]
pub struct ExtensionInfo {
    pub name: String,
    pub description: String,
    pub author: String,
    pub version: String
}

impl Default for ExtensionInfo {
    fn default() -> Self {
        Self {
            name: env::var("CARGO_PKG_NAME").expect("Package name not defined in cargo.toml, required for G-Rust extensions"),
            description: env::var("CARGO_PKG_DESCRIPTION").expect("Package description not defined in cargo.toml, required for G-Rust extensions"),
            author: env::var("CARGO_PKG_AUTHORS").expect("Package author(s) not defined in cargo.toml, required for G-Rust extensions"),
            version: env::var("CARGO_PKG_VERSION").expect("Package version not defined in cargo.toml, required for G-Rust extensions")
        }
    }
}

pub struct Extension<W: Debug + Default> {
    pub info: ExtensionInfo,
    pub args: Vec<String>,
    pub globals: W,
    connection: Option<GEarthConnection>,
    packet_info_manager: Option<PacketInfoManager>,

    delayed_init: bool,
    host_info: Option<HostInfo>,

    on_init: Vec<fn(&mut Self)>,
    on_connect: Vec<fn(&mut Self, ConnectionInfo)>,
    on_start: Vec<fn(&mut Self)>,
    on_end: Vec<fn(&mut Self)>,
    on_click: Vec<fn(&mut Self)>,
    on_host_info_update: Vec<fn(&mut Self, HostInfo)>,
    on_socket_disconnect: Vec<fn(&mut Self)>,

    intercepts_by_id: HashMap<HDirection, HashMap<i16, Vec<Box<dyn Fn(&mut Self, &mut HMessage)>>>>,
    intercepts_by_name: HashMap<HDirection, HashMap<String, Vec<Box<dyn Fn(&mut Self, &mut HMessage)>>>>,

    flag_callback: Option<fn(&mut Self, Vec<String>)>
}

impl <W: Debug + Default> Debug for Extension<W> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Extension")
            .field("info", &self.info)
            .field("args", &self.args)
            .field("globals", &self.globals)
            .finish()
    }
}

impl <W: Debug + Default + 'static> Extension<W> {
    pub fn new() -> Self {
        Extension {
            info: ExtensionInfo::default(),
            connection: None,
            globals: W::default(),
            args: env::args().collect(),
            packet_info_manager: None,

            delayed_init: false,
            host_info: None,

            on_init: Vec::new(),
            on_connect: Vec::new(),
            on_start: Vec::new(),
            on_end: Vec::new(),
            on_click: Vec::new(),
            on_host_info_update: Vec::new(),
            on_socket_disconnect: Vec::new(),

            intercepts_by_id: HashMap::new(),
            intercepts_by_name: HashMap::new(),

            flag_callback: None
        }
    }

    fn get_argument(&mut self, flags: [&str; 2]) -> String {
        for i in 0..self.args.len() - 1 {
            for flag in flags {
                if self.args[i].to_lowercase() == flag.to_lowercase() {
                    return self.args[i + 1].clone();
                }
            }
        }

        String::new()
    }

    pub fn run(&mut self) {
        if self.connection.is_none() {
            self.connection = Some(GEarthConnection::new(self.get_argument(PORT_FLAG)));
            //let read_handle = thread::spawn(move || {
            self.read_loop();
            trigger_listeners!(self.on_socket_disconnect, self);
            //});

            //read_handle.join().unwrap();
        }
    }

    pub fn get_packet_info_manager(&self) -> Option<PacketInfoManager> {
        return self.packet_info_manager.clone()
    }

    pub fn get_host_info(&self) -> Option<HostInfo> {
        return self.host_info.clone()
    }

    fn read_loop(&mut self) {
        loop {
            let length_bytes: Result<Vec<u8>, Error> = self.connection.clone().unwrap().read(4);
            if length_bytes.is_err() {
                break;
            }
            let length = i32::from_packet(length_bytes.clone().unwrap()).0;
            let body_bytes: Result<Vec<u8>, Error> = self.connection.clone().unwrap().read(length as u64);
            if body_bytes.is_err() {
                break;
            }
            let mut bytes = length_bytes.unwrap();
            bytes.append(&mut body_bytes.unwrap());
            self.on_g_packet(HPacket::from_bytes(bytes));
        }
    }

    fn on_g_packet(&mut self, mut packet: HPacket) {
        match packet.header_id() {
            IncomingMessageIds::INFO_REQUEST => self.on_info_request_packet(),
            IncomingMessageIds::CONNECTION_START => self.on_connection_start_packet(packet),
            IncomingMessageIds::CONNECTION_END => self.on_connection_end_packet(),
            IncomingMessageIds::FLAGS_CHECK => self.on_flags_check_packet(packet),
            IncomingMessageIds::INIT => self.on_init_packet(packet),
            IncomingMessageIds::ON_DOUBLE_CLICK => self.on_double_click_packet(),
            IncomingMessageIds::PACKET_INTERCEPT => self.on_packet_intercept_packet(packet),
            IncomingMessageIds::UPDATE_HOST_INFO => self.on_update_host_info_packet(packet),
            _ => println!("Unknown incoming message")
        }
    }

    fn on_info_request_packet(&mut self) {
        let file = self.get_argument(FILE_FLAG);
        let cookie = self.get_argument(COOKIE_FLAG);
        let mut response = HPacket::from_header_id(OutgoingMessageIds::EXTENSION_INFO);
        response.append((
            self.info.name.clone(),
            self.info.author.clone(),
            self.info.version.clone(),
            self.info.description.clone(),
            !self.on_click.is_empty(), // onclick
            file != "", // file == null
            String::from(file), // file
            String::from(cookie), // cookie
            true, // can leave
            true // can delete
        ));
        self.connection.clone().unwrap().write(response.get_bytes());
    }

    fn on_connection_start_packet(&mut self, mut packet: HPacket) {
        let connection_info: ConnectionInfo = packet.read();
        self.packet_info_manager = packet.read();
        *CUR_CLIENT.lock().unwrap() = connection_info.client.clone();

        if self.delayed_init {
            trigger_listeners!(self.on_init.clone(), self);
            self.delayed_init = false;
        }

        trigger_listeners!(self.on_connect, self, connection_info.clone());
        trigger_listeners!(self.on_start, self);
    }

    fn on_connection_end_packet(&mut self) {
        self.packet_info_manager = None;
        trigger_listeners!(self.on_end, self);
    }

    fn on_flags_check_packet(&mut self, mut packet: HPacket) {
        if self.flag_callback.is_some() {
            let count: i32 = packet.read();
            let mut flags: Vec<String> = Vec::new();
            for _ in 0..count {
                flags.push(packet.read());
            }
            (self.flag_callback.unwrap())(self, flags);
        }
        self.flag_callback = None;
    }

    fn on_init_packet(&mut self, mut packet: HPacket) {
        (self.delayed_init, self.host_info) = packet.read();
        trigger_listeners!(self.on_host_info_update, self, self.host_info.clone().unwrap());
        if !self.delayed_init {
            trigger_listeners!(self.on_init, self);
        }

        self.connection.clone().unwrap().write_to_console_formatted(format!("Extension \"{}\" successfully initialized", self.info.name), ConsoleColour::Green);
    }

    fn on_double_click_packet(&mut self) {
        trigger_listeners!(self.on_click, self);
    }

    fn on_packet_intercept_packet(&mut self, mut packet: HPacket) {
        let string_message: LongString = packet.read();
        let mut h_message = HMessage::from_string(string_message.clone());

        self.modify_message(&mut h_message);

        let mut response_packet = HPacket::from_header_id(OutgoingMessageIds::MANIPULATED_PACKET);
        response_packet.append(LongString(h_message.stringify()));

        self.connection.clone().unwrap().write(response_packet.get_bytes());
    }

    fn modify_message(&mut self, msg: &mut HMessage) {
        let names_and_hashes: Vec<String> = if self.packet_info_manager.is_some() {
            let packet_infos = self.packet_info_manager.clone().unwrap()
                .get_all_packet_info_from_header_id(
                    msg.get_destination(),
                    msg.get_packet().header_id() as i32
                );
            packet_infos.iter()
                .flat_map(| packet_info | vec![packet_info.name.clone(), packet_info.hash.clone()])
                .filter(| n | n != "NULL")
                .collect()
        } else { Vec::new() };

        let mut matching_listeners_by_id: Vec<Box<dyn Fn(&mut Self, &mut HMessage)>> = Vec::new();
        let header_id = msg.get_packet().header_id();
        let intercepts_by_id = self.intercepts_by_id.get_mut(&msg.get_destination());
        if intercepts_by_id.is_some() {
            for (id, listeners) in intercepts_by_id.unwrap() {
                if *id == msg.get_packet().header_id() {
                    matching_listeners_by_id.append(listeners)
                }
            }
        }

        for listener in matching_listeners_by_id {
            (listener)(self, msg);
            self.intercept_raw(msg.get_destination(), header_id as i32, listener);
        }

        let mut matching_listeners_by_name: Vec<Box<dyn Fn(&mut Self, &mut HMessage)>> = Vec::new();
        let intercepts_by_name = self.intercepts_by_name.get_mut(&msg.get_destination());
        let mut name = String::default();
        if intercepts_by_name.is_some() {
            for (name_or_hash, listeners) in intercepts_by_name.unwrap() {
                if names_and_hashes.contains(name_or_hash) {
                    name = name_or_hash.clone();
                    matching_listeners_by_name.append(listeners)
                }
            }
        }

        for listener in matching_listeners_by_name {
            (listener)(self, msg);
            self.intercept_raw(msg.get_destination(), name.clone(), listener);
        }
    }

    fn on_update_host_info_packet(&mut self, mut packet: HPacket) {
        self.host_info = packet.read();
        trigger_listeners!(self.on_host_info_update, self, self.host_info.clone().unwrap());
    }

    pub fn write_to_console(&self, s: String) {
        self.connection.clone().expect("Extension not connected yet...").write_to_console(format!("[{}] {s}", self.info.name));
    }

    pub fn write_to_console_formatted(&self, s: String, colour: ConsoleColour) {
        self.connection.clone().expect("Extension not connected yet...").write_to_console_formatted(format!("[{}] {s}", self.info.name), colour);
    }

    pub fn on_init(&mut self, listener: fn(ext: &mut Self)) {
        self.on_init.push(listener);
    }

    pub fn on_socket_disconnect(&mut self, listener: fn(ext: &mut Self)) {
        self.on_socket_disconnect.push(listener);
    }

    pub fn on_connect(&mut self, listener: fn(ext: &mut Self, connection_info: ConnectionInfo)) {
        self.on_connect.push(listener);
    }

    pub fn on_start(&mut self, listener: fn(ext: &mut Self)) {
        self.on_start.push(listener);
    }

    pub fn on_end(&mut self, listener: fn(ext: &mut Self)) {
        self.on_end.push(listener);
    }

    pub fn on_host_info_update(&mut self, listener: fn(ext: &mut Self, host_info: HostInfo)) {
        self.on_host_info_update.push(listener);
    }

    pub fn on_click(&mut self, listener: fn(ext: &mut Self)) {
        self.on_click.push(listener);
    }

    pub fn request_flags(&mut self, callback: fn(ext: &mut Self, flags: Vec<String>)) {
        self.flag_callback = Some(callback);
        let request_packet = HPacket::from_header_id(OutgoingMessageIds::REQUEST_FLAGS);
        self.connection.clone().unwrap().write(request_packet.get_bytes());
    }

    pub fn intercept<T: BaseParser + 'static>(&mut self, listener: fn(ext: &mut Self, msg: &mut HMessage, object: &mut T)) {
        let wrapped_listener = move | ext: &mut Self, msg: &mut HMessage | {
            let mut original_packet = msg.get_packet().clone();
            let mut object: T = original_packet.read();
            let original_object = object.clone();
            (listener)(ext, msg, &mut object);

            if original_object != object {
                msg.get_packet().replace(6, object);
            }
        };
        self.intercept_raw(T::get_direction(), T::get_packet_name(), wrapped_listener);
    }

    pub fn intercept_raw<I: InterceptIndicator>(&mut self, direction: HDirection, indicator: I, listener: impl Fn(&mut Self, &mut HMessage) -> () + 'static) {
        let intercepts = if I::is_raw_habbo_header_id() {
            self.intercepts_by_id
                .entry(direction)
                .or_insert_with(|| HashMap::new())
                .entry(indicator.get_habbo_header_id(HDirection::None, &mut PacketInfoManager::default()).unwrap())
                .or_insert_with( || Vec::new())
        } else {
            self.intercepts_by_name
                .entry(direction)
                .or_insert_with(|| HashMap::new())
                .entry(indicator.get_habbo_header_name())
                .or_insert_with( || Vec::new())
        };

        intercepts.push(Box::new(listener));
    }

    pub fn send_to_client(&self, packet: HPacket) -> bool {
        self.send_internal(packet, HDirection::ToClient)
    }

    pub fn send_to_server(&self, packet: HPacket) -> bool {
        self.send_internal(packet, HDirection::ToServer)
    }

    fn send_internal(&self, mut packet: HPacket, direction: HDirection) -> bool {
        if packet.is_corrupted() || self.connection.clone().is_none() {
            return false;
        }

        if !packet.is_complete()
            && self.packet_info_manager.clone().is_some()
            && packet.can_complete(self.packet_info_manager.clone().unwrap()) {
            packet.complete_packet(self.packet_info_manager.clone().unwrap());
        }
        if !packet.is_complete() {
            return false;
        }

        let mut sending_packet = HPacket::from_header_id(OutgoingMessageIds::SEND_MESSAGE);
        sending_packet.append((direction as u8, packet.bytes_length() as i32));
        sending_packet.append_bytes(packet.get_bytes());

        self.connection.clone().unwrap().write(sending_packet.get_bytes());

        return true;
    }

    pub fn send<B: BaseParser>(&self, packet_object: B) -> bool {
        if self.packet_info_manager.is_none() {
            println!(
                "Couldn't send {} packet, packet info manager has not yet been initialized",
                B::get_packet_name()
            );
            return false;
        }

        let packet_info = self.packet_info_manager.clone().unwrap()
            .get_packet_info_from_name(B::get_direction(), B::get_packet_name());

        if packet_info.is_none() {
            println!(
                "Couldn't send {}, packet info not found",
                B::get_packet_name()
            );
            return false;
        }

        self.send_with_id(packet_object, packet_info.unwrap().header_id)
    }

    pub fn send_with_id<B: BaseParser>(&self, packet_object: B, header_id: i32) -> bool {
        let mut packet = HPacket::from_header_id(header_id as i16);
        packet_object.append_to_packet(&mut packet);
        self.send_internal(packet, B::get_direction())
    }
}

pub trait InterceptIndicator {
    fn get_habbo_header_id(&self, direction: HDirection, packet_info_manager: &mut PacketInfoManager) -> Option<i16>;

    fn matches_habbo_header_id(&self, direction: HDirection, packet_info_manager: &mut PacketInfoManager, header_id: i16) -> bool {
        let id = self.get_habbo_header_id(direction, packet_info_manager);
        id.is_some() && id.unwrap() == header_id
    }

    fn is_raw_habbo_header_id() -> bool {
        false
    }

    fn get_habbo_header_name(&self) -> String {
        String::new()
    }
}

impl InterceptIndicator for String {
    fn get_habbo_header_id(&self, direction: HDirection, packet_info_manager: &mut PacketInfoManager) -> Option<i16> {
        let mut packet_info = packet_info_manager.get_packet_info_from_name(direction.clone(), self.clone());
        if packet_info.is_some() {
            return Some(packet_info.unwrap().header_id as i16);
        }
        packet_info = packet_info_manager.get_packet_info_from_hash(direction.clone(), self.clone());
        if packet_info.is_some() {
            return Some(packet_info.unwrap().header_id as i16);
        }
        None
    }

    fn get_habbo_header_name(&self) -> String {
        self.clone()
    }
}

impl InterceptIndicator for &str {
    fn get_habbo_header_id(&self, direction: HDirection, packet_info_manager: &mut PacketInfoManager) -> Option<i16> {
        self.to_string().get_habbo_header_id(direction, packet_info_manager)
    }

    fn get_habbo_header_name(&self) -> String {
        self.to_string()
    }
}

impl InterceptIndicator for i32 {
    fn get_habbo_header_id(&self, _direction: HDirection, _packet_info_manager: &mut PacketInfoManager) -> Option<i16> {
        Some(*self as i16)
    }

    fn is_raw_habbo_header_id() -> bool {
        true
    }
}