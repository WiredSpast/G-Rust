use std::convert::Infallible;
use std::str::FromStr;
use std::string::ParseError;
use crate::protocol::hmessage::HMessage;

#[derive(Debug, Clone, PartialEq)]
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