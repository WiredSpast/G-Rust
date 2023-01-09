use crate::protocol::hdirection::HDirection;
use crate::protocol::hpacket::HPacket;

pub struct HMessage {
    packet: HPacket,
    index: i64,
    direction: HDirection,
    pub blocked: bool
}

impl HMessage {
    pub fn from_string(s: String) -> Self {
        let parts: Vec<&str> = s.split("\t").collect();

        Self {
            packet: HPacket::from_string(parts[3..].join("\t")),
            index: parts[1].parse::<i64>().unwrap(),
            direction: parts[2].parse::<HDirection>().unwrap(),
            blocked: parts[0] == "1"
        }
    }
}