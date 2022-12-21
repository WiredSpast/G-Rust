use core::mem::size_of;
use std::collections::HashMap;
use std::hash::Hash;
use crate::protocol::hpacket::HPacket;
use crate::protocol::vars::legacy::LegacyLength;

pub trait PacketVariable {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized;
    fn to_packet(&self) -> Vec<u8>;
}

fn to_sized_array<T: Clone, const N: usize>(v: Vec<T>) -> [T; N] {
    v[..N].to_vec().try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

macro_rules! impl_packet_variable {
    ($($ty:ident)+) => ($(
        impl PacketVariable for $ty {
            fn from_packet(bytes: Vec<u8>) -> (Self, usize) {
                let bytes_array: [u8; size_of::<$ty>()] = to_sized_array(bytes);
                (Self::from_be_bytes(bytes_array), size_of::<$ty>())
            }


            fn to_packet(&self) -> Vec<u8> {
                self.to_be_bytes().to_vec()
            }
        }
    )+)
}

impl_packet_variable! { u8 i8 u16 i16 u32 i32 u64 i64 u128 i128 /*usize isize*/ f32 f64 }

impl PacketVariable for bool {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) {
        (bytes[0] != 0, 1)
    }

    fn to_packet(&self) -> Vec<u8> {
        if *self { vec![1] } else { vec![0] }
    }
}

impl PacketVariable for String {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) {
        let s_size = u16::from_packet(bytes.clone()).0 as usize;
        (String::from_utf8(bytes[2..2+s_size].to_vec()).expect("Couldn't read string"), 2+s_size)
    }

    fn to_packet(&self) -> Vec<u8> {
        let bytes = self.as_bytes();
        let len = bytes.len() as u16;
        let mut res = len.to_packet();
        res.extend(bytes);
        res
    }
}

impl<T: PacketVariable + Clone> PacketVariable for Vec<T> {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);
        let mut res: Vec<T> = Vec::new();

        let len: LegacyLength = packet.read();
        for _ in 0..*len {
            res.push(packet.read());
        }

        (res, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        let mut packet = HPacket::from_header_id(0);
        packet.append(LegacyLength(self.len() as i32));

        for element in self.iter() {
            packet.append(element.clone());
        }

        packet.get_bytes()[6..].to_vec()
    }
}

impl<T: PacketVariable + Clone + Eq + Hash, S: PacketVariable + Clone + Eq + Hash> PacketVariable for HashMap<T, S> {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);
        let mut res: HashMap<T, S> = HashMap::new();

        let len: LegacyLength = packet.read();
        for _ in 0..*len {
            res.insert(packet.read(), packet.read());
        }

        (res, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        let mut packet = HPacket::from_header_id(0);
        packet.append(LegacyLength(self.len() as i32));

        for (key, value) in self.iter() {
            packet.append(key.clone());
            packet.append(value.clone());
        }

        packet.get_bytes()[6..].to_vec()
    }
}

macro_rules! impl_packet_tuple_variable {
    ($($size:expr, $($ty:ident),+);+;) => ($(
        impl<$($ty: PacketVariable),+> PacketVariable for ($($ty),+) {
            fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
                let mut packet = HPacket::from_header_id_and_bytes(0, bytes);
                (($(
                    packet.read::<$ty>()
                ),+), packet.read_index - 6)
            }

            fn to_packet(&self) -> Vec<u8> {
                todo!()
            }
        }
    )+)
}

impl_packet_tuple_variable! {
    2,  T1, T2;
    3,  T1, T2, T3;
    4,  T1, T2, T3, T4;
    5,  T1, T2, T3, T4, T5;
    6,  T1, T2, T3, T4, T5, T6;
    7,  T1, T2, T3, T4, T5, T6, T7;
    8,  T1, T2, T3, T4, T5, T6, T7, T8;
    9,  T1, T2, T3, T4, T5, T6, T7, T8, T9;
    10, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10;
    11, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11;
    12, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12;
    13, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13;
    14, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14;
    15, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15;
    16, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16;
    17, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17;
    18, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18;
    19, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19;
    20, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20;
}