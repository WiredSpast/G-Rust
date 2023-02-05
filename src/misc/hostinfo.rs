use std::collections::HashMap;
use crate::protocol::hpacket::HPacket;
use crate::protocol::vars::packetvariable::PacketVariable;

#[derive(Debug, Clone)]
pub struct HostInfo {
    pub packet_logger: String,
    pub version: String,
    pub attributes: HashMap<String, String>
}

impl PacketVariable for HostInfo {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);
        let (packet_logger, version) = packet.read();
        let c: i32 = packet.read();
        let mut attributes = HashMap::new();
        for _ in 0..c {
            attributes.insert(packet.read(), packet.read());
        }

        (HostInfo {
            packet_logger,
            version,
            attributes
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        let mut packet = HPacket::from_header_id(0);
        packet.append((self.packet_logger.clone(), self.version.clone(), self.attributes.len() as i32));
        for (key, value) in self.attributes.iter() {
            packet.append((key.to_string(), value.to_string()));
        }

        packet.get_bytes()[6..].to_vec()
    }
}