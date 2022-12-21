use core::mem::size_of;
use std::fmt::{Debug, Formatter};

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

impl Debug for dyn PacketVariable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}