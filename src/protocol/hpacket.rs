use core::mem::size_of;
use super::hdirection::HDirection;

pub trait FromBytes {
    fn from_be_bytes_wrapper(bytes: Vec<u8>) -> (Self, usize) where Self: Sized;
}

pub trait ToBytes {
    fn to_be_bytes_wrapper(&self) -> Vec<u8>;
}

fn to_sized_array<T: Clone, const N: usize>(v: Vec<T>) -> [T; N] {
    v[..N].to_vec().try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

macro_rules! impl_bytes {
    ($($ty:ident)+) => ($(
        impl FromBytes for $ty {
            fn from_be_bytes_wrapper(bytes: Vec<u8>) -> (Self, usize) {
                let bytes_array: [u8; size_of::<$ty>()] = to_sized_array(bytes);
                (Self::from_be_bytes(bytes_array), size_of::<$ty>())
            }
        }

        impl ToBytes for $ty {
            fn to_be_bytes_wrapper(&self) -> Vec<u8> {
                self.to_be_bytes().to_vec()
            }
        }
    )+)
}

impl_bytes! { u8 i8 u16 i16 u32 i32 u64 i64 u128 i128 /*usize isize*/ f32 f64 }

impl FromBytes for bool {
    fn from_be_bytes_wrapper(bytes: Vec<u8>) -> (Self, usize) {
        (bytes[0] != 0, 1)
    }
}

impl ToBytes for bool {
    fn to_be_bytes_wrapper(&self) -> Vec<u8> {
        if *self { vec ![1] } else { vec![0] }
    }
}

impl FromBytes for String {
    fn from_be_bytes_wrapper(bytes: Vec<u8>) -> (Self, usize) {
        let s_size = u16::from_be_bytes_wrapper(bytes.clone()).0 as usize;
        (String::from_utf8(bytes[2..2+s_size].to_vec()).expect("Couldn't read string"), 2+s_size)
    }
}

impl ToBytes for String {
    fn to_be_bytes_wrapper(&self) -> Vec<u8> {
        let bytes = self.as_bytes();
        let len = bytes.len() as u16;
        let mut res = len.to_be_bytes_wrapper();
        res.extend(bytes);
        res
    }
}

#[derive(Debug, Clone)]
pub struct HPacket {
    is_edited: bool,
    packet_in_bytes: Vec<u8>,
    pub read_index: usize,

    pub identifier: String,
    pub identifier_direction: HDirection
}

impl Default for HPacket {
    fn default() -> Self {
        HPacket {
            is_edited: false,
            packet_in_bytes: vec![0, 0, 0, 2, 0, 0],
            read_index: 6,
            identifier: String::from(""),
            identifier_direction: HDirection::None
        }
    }
}

impl HPacket {
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        let mut res = HPacket::default();
        res.packet_in_bytes = bytes.clone();
        res.fix_length();
        res
    }

    pub fn from_packet(packet: HPacket) -> Self {
        let mut res = HPacket::default();
        res.packet_in_bytes = packet.packet_in_bytes.clone();
        res.is_edited = packet.is_edited;
        res
    }

    pub fn from_header_id(header_id: u16) -> Self {
        let mut res = HPacket::default();
        res.replace(4, header_id);
        res.is_edited = false;
        res
    }

    pub fn from_header_id_and_bytes(header_id: u16, bytes: Vec<u8>) -> Self {
        let mut res = Self::from_header_id(header_id);
        // TODO this.append_bytes(bytes);
        res.is_edited = false;
        res
    }

    pub fn from_identifier_and_direction(identifier: String, direction: HDirection) -> HPacket {
        let mut res = HPacket::default();
        res.identifier = identifier.clone();
        res.identifier_direction = direction.clone();
        res
    }

    // TODO from_string

    pub fn eof(&self) -> i8 {
        if self.read_index < self.packet_in_bytes.len() { 0 }
        else if self.read_index > self.packet_in_bytes.len() { 2 }
        else { 1 }
    }

    // TODO can_complete

    pub fn can_send_to_client(&self) -> bool {
        self.identifier_direction == HDirection::ToClient
    }

    pub fn can_send_to_server(&self) -> bool {
        self.identifier_direction == HDirection::ToServer
    }

    // TODO complete_packet

    pub fn is_complete(&self) -> bool {
        self.identifier == ""
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        self.packet_in_bytes.clone()
    }

    pub fn reset_read_index(&mut self) {
        self.read_index = 6;
    }

    pub fn is_corrupted(&mut self) -> bool {
        !(self.bytes_length() >= 6 && self.length() == self.bytes_length() - 4)
    }

    pub fn length(&mut self) -> usize {
        self.read_at::<i32>(0) as usize
    }

    pub fn bytes_length(&self) -> usize {
        self.packet_in_bytes.len()
    }

    pub fn header_id(&mut self) -> i16 {
        self.read_at::<i16>(4)
    }

    fn fix_length(&mut self) {
        self.replace_internal(0, self.bytes_length() as i32 - 4);
    }

    pub fn read_bytes(&mut self, length: usize) -> Vec<u8> {
        self.read_index += length;
        self.packet_in_bytes[self.read_index-length..self.read_index].to_vec()
    }

    pub fn read_bytes_at(&mut self, length: usize, index: usize) -> Vec<u8> {
        self.packet_in_bytes[index..index+length].to_vec()
    }

    pub fn read<T: FromBytes>(&mut self) -> T {
        let bytes = self.packet_in_bytes[self.read_index..].to_vec();
        let (res, size) = T::from_be_bytes_wrapper(bytes);
        self.read_index += size;
        res
    }

    pub fn read_at<T: FromBytes>(&mut self, index: usize) -> T {
        if index >= self.bytes_length() {
            panic!("Read index '{}' out of bounds [0-{}[", index, self.bytes_length());
        }

        let bytes = self.packet_in_bytes[index..].to_vec();
        T::from_be_bytes_wrapper(bytes).0
    }

    pub fn append<T: ToBytes>(&mut self, v: T) {
        let bytes = v.to_be_bytes_wrapper().to_vec();
        self.packet_in_bytes.extend(bytes);
        self.is_edited = true;
        self.fix_length();
    }

    fn replace_internal<T: ToBytes + FromBytes>(&mut self, index: usize, v: T) {
        let old_bytes = self.packet_in_bytes[index..].to_vec();
        let (_, old_size) = T::from_be_bytes_wrapper(old_bytes.clone());
        let mut res = self.packet_in_bytes[..index].to_vec();
        res.extend(v.to_be_bytes_wrapper());
        res.extend(old_bytes[old_size..].to_vec());
        self.packet_in_bytes = res.clone();
    }

    pub fn replace<T: ToBytes + FromBytes>(&mut self, index: usize, v: T) {
        self.replace_internal(index, v);
        self.is_edited = true;
        self.fix_length();
    }

    pub fn insert<T: ToBytes>(&mut self, index: usize, v: T) {
        let mut res = self.packet_in_bytes[..index].to_vec();
        res.extend(v.to_be_bytes_wrapper());
        res.extend(self.packet_in_bytes[index..].to_vec());
        self.packet_in_bytes = res.clone();
    }
}