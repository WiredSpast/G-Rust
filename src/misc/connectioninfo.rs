use crate::misc::hclient::HClient;
use crate::protocol::vars::packetvariable::PacketVariable;
use crate::protocol::hpacket::HPacket;
use crate::extension::parsers::baseparser::BaseParser;

#[derive(Clone, Eq, PartialEq, PacketVariable, BaseParser)]
pub struct ConnectionInfo {
    pub host: String,
    pub port: i32,
    pub hotel_version: String,
    pub client_identifier: String,
    pub client: HClient
}