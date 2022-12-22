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
    ($($size:expr, $($ty:ident:$n:tt),+);+;) => ($(
        impl<$($ty: PacketVariable + Clone),+> PacketVariable for ($($ty),+) {
            fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
                let mut packet = HPacket::from_header_id_and_bytes(0, bytes);
                (($(
                    packet.read::<$ty>()
                ),+), packet.read_index - 6)
            }

            fn to_packet(&self) -> Vec<u8> {
                let mut packet = HPacket::from_header_id(0);
                $(
                    packet.append(self.$n.clone());
                )+
                packet.get_bytes()[6..].to_vec()
            }
        }
    )+)
}

impl_packet_tuple_variable! {
    2,  T0:0, T1:1;
    3,  T0:0, T1:1, T2:2;
    4,  T0:0, T1:1, T2:2, T3:3;
    5,  T0:0, T1:1, T2:2, T3:3, T4:4;
    6,  T0:0, T1:1, T2:2, T3:3, T4:4, T5:5;
    7,  T0:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6;
    8,  T0:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7;
    9,  T0:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8;
    10, T0:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8, T9:9;
    11, T0:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8, T9:9, T10:10;
    12, T0:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8, T9:9, T10:10, T11:11;
    13, T0:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8, T9:9, T10:10, T11:11, T12:12;
    14, T0:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8, T9:9, T10:10, T11:11, T12:12, T13:13;
    15, T0:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8, T9:9, T10:10, T11:11, T12:12, T13:13, T14:14;
    16, T0:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8, T9:9, T10:10, T11:11, T12:12, T13:13, T14:14, T15:15;
    17, T0:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8, T9:9, T10:10, T11:11, T12:12, T13:13, T14:14, T15:15, T16:16;
    18, T0:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8, T9:9, T10:10, T11:11, T12:12, T13:13, T14:14, T15:15, T16:16, T17:17;
    19, T0:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8, T9:9, T10:10, T11:11, T12:12, T13:13, T14:14, T15:15, T16:16, T17:17, T18:18;
    20, T0:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8, T9:9, T10:10, T11:11, T12:12, T13:13, T14:14, T15:15, T16:16, T17:17, T18:18, T19:19;
}

macro_rules! impl_packet_array_variable {
    ($($size:expr),+) => ($(
        impl<T: PacketVariable + Clone> PacketVariable for [T; $size] {
            fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
                let mut packet = HPacket::from_header_id_and_bytes(0, bytes);
                let mut res: Vec<T> = Vec::new();
                for _ in 0..$size {
                    res.push(packet.read());
                }
                (to_sized_array::<T, $size>(res), packet.read_index - 6)
            }

            fn to_packet(&self) -> Vec<u8> {
                let mut packet = HPacket::from_header_id(0);

                for element in self.iter() {
                    packet.append(element.clone());
                }

                packet.get_bytes()[6..].to_vec()
            }
        }
    )+)
}

impl_packet_array_variable! { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20 }