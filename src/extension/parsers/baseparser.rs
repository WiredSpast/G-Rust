use crate::protocol::hpacket::HPacket;

pub trait BaseParser {
    fn parse(packet: &mut HPacket) -> Self;
    fn append_to_packet(&self, packet: &mut HPacket);
}