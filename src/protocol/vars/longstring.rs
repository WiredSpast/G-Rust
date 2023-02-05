use std::fmt::{Debug, Display, Formatter};
use std::ops::{Deref, DerefMut};
use crate::protocol::vars::packetvariable::PacketVariable;

pub struct LongString(String);

impl LongString {
    pub fn from(s: &str) -> LongString {
        LongString(String::from(s))
    }
}

impl PacketVariable for LongString {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let s_size = u32::from_packet(bytes.clone()).0 as usize;
        (Self(String::from_utf8(bytes[4..4+s_size].to_vec()).expect("Couldn't read string")), 4+s_size)
    }

    fn to_packet(&self) -> Vec<u8> {
        let bytes = self.0.as_bytes();
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