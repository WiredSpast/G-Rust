use crate::protocol::hdirection::HDirection;
use crate::protocol::hpacket::HPacket;
use crate::protocol::vars::packetvariable::PacketVariable;
use crate::extension::parsers::baseparser::BaseParser;

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq, Eq)]
pub struct PacketInfo {
    pub header_id: i32,
    pub hash: String,
    pub name: String,
    pub structure: String,
    pub destination: HDirection,
    pub source: String
}