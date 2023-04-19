use std::net::TcpStream;
use std::fmt::{Debug, Error};
use std::io::{Read, Write};
use super::messages::OutgoingMessageIds;
use super::consoleformat::ConsoleColour;
use crate::protocol::hpacket::HPacket;

#[derive(Debug)]
pub(crate) struct GEarthConnection {
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
        let mut take = self.socket.take(length);
        let mut bytes = Vec::new();
        let res = take.read_to_end(&mut bytes);
        if res.is_ok() && res.unwrap() == length as usize {
            Ok(bytes)
        } else {
            Err(Error)
        }
    }

    pub fn write(mut self, bytes: Vec<u8>) {
        self.socket.write(&bytes[..]).expect("Couldn't write to socket");
    }

    pub fn write_to_console(self, s: String) {
        self.write_to_console_formatted(s, ConsoleColour::White)
    }

    pub fn write_to_console_formatted(self, s: String, colour: ConsoleColour) {
        let mut packet = HPacket::from_header_id(OutgoingMessageIds::EXTENSION_CONSOLE_LOG);
        packet.append(format!("[{colour}] {s}"));
        self.write(packet.get_bytes());
    }
}
