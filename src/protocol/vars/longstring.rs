use std::fmt::{Debug, Display, Formatter};
use std::ops::{Deref, DerefMut};
use encoding::all::ISO_8859_1;
use encoding::{DecoderTrap, EncoderTrap, Encoding};
use crate::protocol::vars::packetvariable::PacketVariable;

pub struct LongString(pub String);

impl LongString {
    pub fn from(s: &str) -> LongString {
        LongString(String::from(s))
    }
}

impl PacketVariable for LongString {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let s_size = u32::from_packet(bytes.clone()).0 as usize;
        (Self(ISO_8859_1.decode(&bytes[4..4+s_size], DecoderTrap::Ignore).expect("Couldn't read string")), 4+s_size)
    }

    fn to_packet(&self) -> Vec<u8> {
        let bytes = ISO_8859_1.encode(&self.0, EncoderTrap::Ignore).expect("Couldn't encode string");
        let len = bytes.len() as u32;
        let mut res = len.to_packet();
        res.extend(bytes);
        res
    }
}

impl Deref for LongString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LongString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Debug for LongString{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for LongString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}