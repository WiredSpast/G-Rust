use crate::misc::hclient::HClient;
use crate::protocol::vars::packetvariable::PacketVariable;
use crate::protocol::hpacket::HPacket;

#[derive(Clone, Eq, PartialEq, PacketVariable, Debug)]
pub struct ConnectionInfo {
    pub host: String,
    pub port: i32,
    pub hotel_version: String,
    pub client_identifier: String,
    pub client: HClient
}