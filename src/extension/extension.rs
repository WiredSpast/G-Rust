use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{env, ptr, thread};
use std::borrow::{Borrow, BorrowMut};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use crate::misc::hostinfo::HostInfo;
use crate::protocol::hpacket::HPacket;
use crate::protocol::vars::packetvariable::PacketVariable;

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
pub struct Extension {
    connection: Option<GEarthConnection>,
    args: Vec<String>,

    delayed_init: bool,
    host_info: Option<HostInfo>,
    // incomingMessageListeners: Map<>,
}

impl Extension {
    pub fn new() -> Self {
        Extension {
            connection: None,
            args: env::args().collect(),

            delayed_init: false,
            host_info: None
        }
    }

    pub fn new_custom_args(args: Vec<String>) -> Self {
        Extension {
            connection: None,
            args,

            delayed_init: false,
            host_info: None
        }
    }

    fn get_argument(self, flags: [&str; 2]) -> String {
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
        println!("b");
        self.connection = Some(GEarthConnection::new(self.clone().get_argument(PORT_FLAG)));
        let read_handle = thread::spawn(move || {
            self.read_loop();
            println!("Socket closed");
        });

        read_handle.join().unwrap();
    }

    fn read_loop(self) {
        loop {
            let mut bytes = self.clone().connection.unwrap().read(4);
            let length = i32::from_packet(bytes.clone()).0;
            bytes.append(&mut self.clone().connection.unwrap().read(length as u64));
            self.clone().on_g_packet(HPacket::from_bytes(bytes));
        }
    }

    fn on_g_packet(self, mut packet: HPacket) {
        match packet.header_id() {
            IncomingMessageIds::INFO_REQUEST => self.on_info_request(),
            IncomingMessageIds::CONNECTION_START => self.on_connection_start(packet),
            IncomingMessageIds::CONNECTION_END => self.on_connection_end(),
            IncomingMessageIds::FLAGS_CHECK => self.on_flags_check(packet),
            IncomingMessageIds::INIT => self.on_init(packet),
            _ => println!("Unknown incoming message")
        }
    }

    fn on_info_request(mut self) {
        // let file = self.get_argument(FILE_FLAG);
        // let cookie = self.get_argument(COOKIE_FLAG);
        let mut response = HPacket::from_header_id(OutgoingMessageIds::EXTENSION_INFO);
        response.append((String::from("Rust test"), String::from("WiredSpast"), String::from("0.1"), String::from("Connection test"), false, false, String::from(""), String::from(""), true, true));
        self.connection.unwrap().write(response.get_bytes());
    }

    fn on_connection_start(self, mut packet: HPacket) {
        let (host, connection_port, hotel_version, client_identifier, client): (String, i32, String, String, String) = packet.read();
        // TODO packetInfoManager

    }

    fn on_connection_end(self) {
        // todo
    }

    fn on_flags_check(self, mut packet: HPacket) {
        // todo handle flag request callback
    }

    fn on_init(mut self, mut packet: HPacket) {
        (self.delayed_init, self.host_info) = packet.read();
        self.write_to_console("Abcdef".to_string());
    }

    fn write_to_console(self, s: String) {
        let mut packet = HPacket::from_header_id(OutgoingMessageIds::EXTENSION_CONSOLE_LOG);
        packet.append(format!("[black] {s}"));
        self.connection.unwrap().write(packet.get_bytes());
    }
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

    pub fn read(self, length: u64) -> Vec<u8> {
        self.socket.take(length).bytes().map(|r| r.unwrap()).collect()
    }

    pub fn write(mut self, bytes: Vec<u8>) {
        self.socket.write(&bytes[..]).expect("Couldn't write to socket");
        // self.socket.flush().expect("Couldn't flush socket");
    }
}