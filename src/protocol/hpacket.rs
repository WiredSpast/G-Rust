use encoding::all::ISO_8859_1;
use encoding::{DecoderTrap, EncoderTrap, Encoding};
use crate::services::packetinfo::packetinfomanager::PacketInfoManager;
use super::hdirection::HDirection;
use super::vars::packetvariable::PacketVariable;

#[derive(Debug, Clone, Eq, PartialEq)]
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

    pub fn from_header_id(header_id: i16) -> Self {
        let mut res = HPacket::default();
        res.replace::<i16>(4, header_id);
        res.is_edited = false;
        res
    }

    pub fn from_header_id_and_bytes(header_id: i16, bytes: Vec<u8>) -> Self {
        let mut res = Self::from_header_id(header_id);
        res.append_bytes(bytes);
        res.is_edited = false;
        res
    }

    pub fn from_identifier_and_direction(identifier: String, direction: HDirection) -> HPacket {
        let mut res = HPacket::default();
        res.identifier = identifier.clone();
        res.identifier_direction = direction.clone();
        res
    }

    pub fn from_string(s: String) -> HPacket {
        let mut res = HPacket::default();
        res.is_edited = s.chars().nth(0).unwrap() == '1';
        res.packet_in_bytes = ISO_8859_1.encode(&s[1..], EncoderTrap::Ignore).unwrap();
        res
    }

    pub fn get_bytes_available(&mut self) -> usize {
        self.packet_in_bytes.len() - self.read_index
    }

    pub fn eof(&mut self) -> i8 {
        if self.read_index < self.packet_in_bytes.len() { 0 }
        else if self.read_index > self.packet_in_bytes.len() { 2 }
        else { 1 }
    }

    pub(crate) fn can_complete(&mut self, manager: PacketInfoManager) -> bool {
        if self.is_corrupted() || self.identifier == "" || self.identifier_direction == HDirection::None {
            return false;
        }

        let packet_info = manager.clone()
            .get_packet_info_from_name(self.identifier_direction.clone(), self.identifier.clone())
            .or(manager.clone().get_packet_info_from_hash(self.identifier_direction.clone(), self.identifier.clone()));

        return packet_info.is_some();
    }

    pub fn can_send_to_client(&mut self) -> bool {
        self.identifier_direction == HDirection::ToClient
    }

    pub fn can_send_to_server(&mut self) -> bool {
        self.identifier_direction == HDirection::ToServer
    }

    pub(crate) fn complete_packet(&mut self, manager: PacketInfoManager) {
        if self.is_corrupted() || self.identifier == "" || self.identifier_direction == HDirection::None {
            return;
        }

        let packet_info = manager.clone()
            .get_packet_info_from_name(self.identifier_direction.clone(), self.identifier.clone())
            .or(manager.clone().get_packet_info_from_hash(self.identifier_direction.clone(), self.identifier.clone()));

        if packet_info.is_none() {
            return;
        }

        self.replace(4, packet_info.unwrap().header_id as u16);
    }

    pub fn is_complete(&mut self) -> bool {
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
        self.replace_internal::<i32>(0, self.bytes_length() as i32 - 4);
    }

    pub fn read_bytes(&mut self, length: usize) -> Vec<u8> {
        self.read_index += length;
        self.packet_in_bytes[self.read_index-length..self.read_index].to_vec()
    }

    pub fn read_bytes_at(&mut self, length: usize, index: usize) -> Vec<u8> {
        self.packet_in_bytes[index..index+length].to_vec()
    }

    pub fn read<T: PacketVariable>(&mut self) -> T {
        let bytes = self.packet_in_bytes[self.read_index..].to_vec();
        let (res, size) = T::from_packet(bytes.clone());
        self.read_index += size;
        res
    }

    pub fn read_at<T: PacketVariable>(&mut self, index: usize) -> T {
        let bytes = self.packet_in_bytes[index..].to_vec();
        T::from_packet(bytes.clone()).0
    }

    pub fn append_bytes(&mut self, bytes: Vec<u8>) {
        self.packet_in_bytes.extend(bytes);
        self.is_edited = true;
        self.fix_length();
    }

    pub fn append<T: PacketVariable>(&mut self, v: T) {
        let bytes = v.to_packet();
        self.packet_in_bytes.extend(bytes);
        self.is_edited = true;
        self.fix_length();
    }

    pub fn replace_bytes(&mut self, index: usize, bytes: Vec<u8>) {
        let mut res = self.packet_in_bytes[..index].to_vec();
        res.extend(bytes.clone());
        res.extend(self.packet_in_bytes[index+bytes.len()..].to_vec());
        self.packet_in_bytes = res.clone();
        self.is_edited = true;
        self.fix_length();
    }

    fn replace_internal<T: PacketVariable>(&mut self, index: usize, v: T) {
        let old_bytes = self.packet_in_bytes[index..].to_vec();
        let (_, old_size) = T::from_packet(old_bytes.clone());
        let mut res = self.packet_in_bytes[..index].to_vec();
        res.extend(v.to_packet());
        res.extend(old_bytes[old_size..].to_vec());
        self.packet_in_bytes = res.clone();
    }

    pub fn replace<T: PacketVariable>(&mut self, index: usize, v: T) {
        self.replace_internal(index, v);
        self.is_edited = true;
        self.fix_length();
    }

    pub fn insert_bytes(&mut self, index: usize, bytes: Vec<u8>) {
        let mut res = self.packet_in_bytes[..index].to_vec();
        res.extend(bytes);
        res.extend(self.packet_in_bytes[index..].to_vec());
        self.packet_in_bytes = res.clone();
        self.is_edited = true;
        self.fix_length();
    }

    pub fn insert<T: PacketVariable>(&mut self, index: usize, v: T) {
        let mut res = self.packet_in_bytes[..index].to_vec();
        res.extend(v.to_packet());
        res.extend(self.packet_in_bytes[index..].to_vec());
        self.packet_in_bytes = res.clone();
    }

    pub fn stringify(&self) -> String {
        if self.is_edited { "1" } else { "0" }.to_string() + &*ISO_8859_1.decode(&self.packet_in_bytes[..], DecoderTrap::Ignore).unwrap()
    }
}
