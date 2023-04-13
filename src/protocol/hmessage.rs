use crate::protocol::hdirection::HDirection;
use crate::protocol::hpacket::HPacket;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HMessage {
    packet: HPacket,
    index: i32,
    direction: HDirection,
    pub blocked: bool
}

impl HMessage {
    pub fn from_string(s: String) -> Self {
        let parts: Vec<&str> = s.split("\t").collect();

        Self {
            packet: HPacket::from_string(parts[3..].join("\t")),
            index: parts[1].parse::<i32>().unwrap(),
            direction: parts[2].parse::<HDirection>().unwrap(),
            blocked: parts[0] == "1"
        }
    }

    pub fn from_message(message: Self) -> Self {
        Self {
            packet: HPacket::from_packet(message.packet),
            index: message.index,
            direction: message.direction,
            blocked: message.blocked
        }
    }

    pub fn from_packet_dir_index(packet: HPacket, direction: HDirection, index: i32) -> Self {
        Self {
            packet,
            direction,
            index,
            blocked: false
        }
    }

    pub fn get_packet(&mut self) -> &mut HPacket {
        &mut self.packet
    }

    pub fn get_index(&mut self) -> i32 {
        self.index
    }

    pub fn get_destination(&mut self) -> HDirection {
        self.direction.clone()
    }

    pub fn is_corrupted(&mut self) -> bool {
        self.packet.is_corrupted()
    }

    pub fn stringify(&mut self) -> String {
        (if self.blocked { "1" } else { "0" }).to_string() + "\t" + self.index.to_string().as_str() + "\t" + self.direction.to_string().as_str() + "\t" + self.packet.stringify().as_str()
    }
}