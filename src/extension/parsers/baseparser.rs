use std::fmt::Debug;
use crate::protocol::hdirection::HDirection;
use crate::protocol::hpacket::HPacket;
use crate::protocol::vars::packetvariable::PacketVariable;

pub trait BaseParser: PacketVariable + Clone + PartialEq + Debug {
    fn parse(packet: &mut HPacket) -> Self;
    fn append_to_packet(&self, packet: &mut HPacket);
    fn get_direction() -> HDirection;
    fn get_packet_name() -> String;
}