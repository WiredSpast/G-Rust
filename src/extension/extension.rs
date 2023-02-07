use std::io::{Read, Write};
use std::net::TcpStream;
use std::{env, thread};
use std::fmt::Error;
use crate::extension::consoleformat::ConsoleColour;
use crate::misc::connectioninfo::ConnectionInfo;
use crate::misc::hostinfo::HostInfo;
use crate::misc::hclient::CUR_CLIENT;
use crate::protocol::hdirection::HDirection;
use crate::protocol::hmessage::HMessage;
use crate::protocol::hpacket::HPacket;
use crate::protocol::vars::longstring::LongString;
use crate::protocol::vars::packetvariable::PacketVariable;
use crate::services::packetinfo::packetinfomanager::PacketInfoManager;

struct IncomingMessageIds;
impl IncomingMessageIds {
    pub const ON_DOUBLE_CLICK: i16 = 1;
    pub const INFO_REQUEST: i16 = 2;
    pub const PACKET_INTERCEPT: i16 = 3;
    pub const FLAGS_CHECK: i16 = 4;
    pub const CONNECTION_START: i16 = 5;
    pub const CONNECTION_END: i16 = 6;
    pub const INIT: i16 = 7;
    pub const UPDATE_HOST_INFO: i16 = 10;
    pub const PACKET_TO_STRING_RESPONSE: i16 = 20;
    pub const STRING_TO_PACKET_RESPONSE: i16 = 21;
}

struct OutgoingMessageIds;
impl OutgoingMessageIds {
    pub const EXTENSION_INFO: i16 = 1;
    pub const MANIPULATED_PACKET: i16 = 2;
    pub const REQUEST_FLAGS: i16 = 3;
    pub const SEND_MESSAGE: i16 = 4;
    pub const PACKET_TO_STRING_REQUEST: i16 = 20;
    pub const STRING_TO_PACKET_REQUEST: i16 = 21;
    pub const EXTENSION_CONSOLE_LOG: i16 = 98;
}

const PORT_FLAG: [&str; 2] = ["--port", "-p"];
const FILE_FLAG: [&str; 2] = ["--filename", "-f"];
const COOKIE_FLAG: [&str; 2] = ["--auth-token", "-c"];

#[derive(Debug, Clone)]
pub struct ExtensionInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub author: &'static str,
    pub version: &'static str
}

impl ExtensionInfo {
    fn read_from_cargo() -> Self {
        ExtensionInfo {
            name: env!("CARGO_PKG_NAME"),
            description: env!("CARGO_PKG_DESCRIPTION"),
            author: env!("CARGO_PKG_AUTHORS"),
            version: env!("CARGO_PKG_VERSION")
        }
    }
}

#[derive(Debug, Clone)]
pub struct Extension {
    pub info: ExtensionInfo,
    pub args: Vec<String>,
    connection: Option<GEarthConnection>,
    packet_info_manager: Option<PacketInfoManager>,

    delayed_init: bool,
    host_info: Option<HostInfo>,

    on_init: Vec<fn()>,
    on_connect: Vec<fn(ConnectionInfo)>,
    on_start: Vec<fn()>,
    on_end: Vec<fn()>,
    on_click: Vec<fn()>,
    on_host_info_update: Vec<fn(HostInfo)>,
    on_socket_disconnect: Vec<fn()>,

    flag_callback: Option<fn(Vec<String>)>
}

impl Extension {
    pub fn new() -> Self {
        Extension {
            info: ExtensionInfo::read_from_cargo(),
            connection: None,
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

            flag_callback: None
        }
    }

    fn get_argument(&self, flags: [&str; 2]) -> String {
        for i in 0..self.args.len() - 1 {
            for flag in flags {
                if self.args[i].to_lowercase() == flag.to_lowercase() {
                    return self.args[i + 1].clone();
                }
            }
        }

        String::new()
    }

    pub fn run(mut self) {
        self.connection = Some(GEarthConnection::new(self.clone().get_argument(PORT_FLAG)));
        let read_handle = thread::spawn(move || {
            self.clone().read_loop();
            for listener in self.on_socket_disconnect.iter() {
                (listener)();
            }
        });

        read_handle.join().unwrap();
    }

    fn read_loop(self) {
        loop {
            let length_bytes: Result<Vec<u8>, Error> = self.clone().connection.unwrap().read(4);
            if length_bytes.is_err() {
                break;
            }
            let length = i32::from_packet(length_bytes.clone().unwrap()).0;
            let body_bytes: Result<Vec<u8>, Error> = self.clone().connection.unwrap().read(length as u64);
            if body_bytes.is_err() {
                break;
            }
            let mut bytes = length_bytes.unwrap();
            bytes.append(&mut body_bytes.unwrap());
            self.clone().on_g_packet(HPacket::from_bytes(bytes));
        }
    }

    fn on_g_packet(mut self, mut packet: HPacket) {
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
            String::from(self.info.name),
            String::from(self.info.author),
            String::from(self.info.version),
            String::from(self.info.description),
            !self.on_click.is_empty(), // onclick
            file != "", // file == null
            String::from(file), // file
            String::from(cookie), // cookie
            true, // can leave
            true // can delete
        ));
        self.clone().connection.unwrap().write(response.get_bytes());
    }

    fn on_connection_start_packet(mut self, mut packet: HPacket) {
        let connection_info: ConnectionInfo = packet.read();
        self.packet_info_manager = packet.read();
        *CUR_CLIENT.lock().unwrap() = connection_info.client.clone();

        if self.delayed_init {
            for listener in self.on_init.iter() {
                (listener)();
            }
            self.delayed_init = false;
        }

        for listener in self.on_connect.iter() {
            (listener)(connection_info.clone());
        }
        for listener in self.on_start.iter() {
            (listener)();
        }
    }

    fn on_connection_end_packet(self) {
        for listener in self.on_end.iter() {
            (listener)();
        }
    }

    fn on_flags_check_packet(mut self, mut packet: HPacket) {
        if self.flag_callback.is_some() {
            let count: i32 = packet.read();
            let mut flags: Vec<String> = Vec::new();
            for _ in 0..count {
                flags.push(packet.read());
            }
            (self.flag_callback.unwrap())(flags);
        }
        self.flag_callback = None;
    }

    fn on_init_packet(&mut self, mut packet: HPacket) {
        (self.delayed_init, self.host_info) = packet.read();
        for listener in self.on_host_info_update.iter() {
            (listener)(self.clone().host_info.unwrap());
        }
        if !self.delayed_init {
            for listener in self.on_init.iter() {
                (listener)();
            }
        }

        self.clone().connection.unwrap().write_to_console_formatted(format!("Extension \"{}\" successfully initialized", self.info.name), ConsoleColour::Green);
    }

    fn on_double_click_packet(self) {
        for listener in self.on_click.iter() {
            (listener)();
        }
    }

    fn on_packet_intercept_packet(self, mut packet: HPacket) {
        let string_message: LongString = packet.read();
        let h_message = HMessage::from_string(string_message.clone());

        // todo modify_message

        let mut response_packet = HPacket::from_header_id(OutgoingMessageIds::MANIPULATED_PACKET);
        response_packet.append(LongString(h_message.stringify()));

        self.connection.unwrap().write(response_packet.get_bytes());
    }

    fn on_update_host_info_packet(mut self, mut packet: HPacket) {
        self.host_info = packet.read();
        for listener in self.on_host_info_update.iter() {
            (listener)(self.clone().host_info.unwrap());
        }
    }

    pub fn write_to_console(self, s: String) {
        self.connection.expect("Extension not connected yet...").write_to_console(format!("[{}] {s}", self.info.name));
    }

    pub fn write_to_console_formatted(self, s: String, colour: ConsoleColour) {
        self.connection.expect("Extension not connected yet...").write_to_console_formatted(format!("[{}] {s}", self.info.name), colour);
    }

    pub fn on_init(&mut self, listener: fn()) {
        self.on_init.push(listener);
    }

    pub fn on_socket_disconnect(&mut self, listener: fn()) {
        self.on_socket_disconnect.push(listener);
    }

    pub fn on_connect(&mut self, listener: fn(ConnectionInfo)) {
        self.on_connect.push(listener);
    }

    pub fn on_start(&mut self, listener: fn()) {
        self.on_start.push(listener);
    }

    pub fn on_end(&mut self, listener: fn()) {
        self.on_end.push(listener);
    }

    pub fn on_host_info_update(&mut self, listener: fn(host_info: HostInfo)) {
        self.on_host_info_update.push(listener);
    }

    pub fn on_click(&mut self, listener: fn()) {
        self.on_click.push(listener);
    }

    // pub fn send_to_client(self, packet: HPacket) -> bool {
    //
    // }
    //
    // fn send(packet: HPacket, direction: HDirection) -> bool {
    //     if packet.is_corrupted() {
    //         return false;
    //     }
    //
    //     if !packet.is_complete() {
    //         // todo complete
    //     }
    //     if !packet.is_complete() {
    //         return false;
    //     }
    //
    //
    // }
}

#[derive(Debug)]
struct GEarthConnection {
    socket: Box<TcpStream>
}

impl Clone for GEarthConnection {
    fn clone(&self) -> Self {
        let socket = self.socket.try_clone().expect("Couldn't clone extension");
        GEarthConnection {
            socket: Box::new(socket)
        }
    }
}

impl GEarthConnection {
    pub fn new(port: String) -> Self {
        let con = TcpStream::connect(format!("127.0.0.1:{port}")).expect("Couldn't connect to G-Earth...");
        con.set_nodelay(true).expect("Couldn't set \"nodelay\" to true...");
        GEarthConnection {
            socket: Box::new(con)
        }
    }

    pub fn read(self, length: u64) -> Result<Vec<u8>, Error> {
        let bytes: Vec<u8> = self.socket.take(length).bytes().filter(| r | r.is_ok()).map(|r| r.unwrap()).collect();
        if bytes.len() == length as usize {
            Ok(bytes)
        } else {
            Err(Error)
        }
    }

    pub fn write(mut self, bytes: Vec<u8>) {
        self.socket.write(&bytes[..]).expect("Couldn't write to socket");
    }

    pub fn write_to_console(self, s: String) {
        self.write_to_console_formatted(s, ConsoleColour::Caret)
    }

    pub fn write_to_console_formatted(self, s: String, colour: ConsoleColour) {
        let mut packet = HPacket::from_header_id(OutgoingMessageIds::EXTENSION_CONSOLE_LOG);
        packet.append(format!("[{colour}] {s}"));
        self.write(packet.get_bytes());
    }
}