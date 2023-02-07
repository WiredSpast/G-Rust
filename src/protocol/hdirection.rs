use std::convert::Infallible;
use std::str::FromStr;
use crate::protocol::vars::packetvariable::PacketVariable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HDirection {
    ToClient,
    ToServer,
    None
}

impl Default for HDirection {
    fn default() -> Self {
        HDirection::None
    }
}

impl FromStr for HDirection {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            match s {
                "TOCLIENT" => HDirection::ToClient,
                "TOSERVER" => HDirection::ToServer,
                _ => HDirection::None
            }
        )
    }
}

impl ToString for HDirection {
    fn to_string(&self) -> String {
        match self {
            HDirection::ToClient => "TOCLIENT".to_string(),
            HDirection::ToServer => "TOSERVER".to_string(),
            HDirection::None => "NONE".to_string()
        }
    }
}

impl PacketVariable for HDirection {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        (
            if bool::from_packet(bytes).0 { HDirection::ToServer } else { HDirection::ToClient },
            1
        )
    }

    fn to_packet(&self) -> Vec<u8> {
        (self == &HDirection::ToServer).to_packet()
    }
}