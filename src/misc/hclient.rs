use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::protocol::vars::packetvariable::PacketVariable;

pub(crate) static CUR_CLIENT: Lazy<Mutex<HClient>> = Lazy::new(| | Mutex::new(HClient::Undefined));

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum HClient {
    Unity,
    Flash,
    Nitro,
    Undefined
}

impl PacketVariable for HClient {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let (key, size) = String::from_packet(bytes);
        (match key.as_str() {
            "UNITY" => HClient::Unity,
            "FLASH" => HClient::Flash,
            "NITRO" => HClient::Nitro,
            _ => HClient::Undefined
        }, size)
    }

    fn to_packet(&self) -> Vec<u8> {
        (match self {
            HClient::Unity => "UNITY",
            HClient::Flash => "FLASH",
            HClient::Nitro => "NITRO",
            HClient::Undefined => ""
        }).to_string().to_packet()
    }
}
