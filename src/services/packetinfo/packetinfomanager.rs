use std::collections::HashMap;
use crate::protocol::hdirection::HDirection;
use crate::protocol::hpacket::HPacket;
use crate::protocol::vars::packetvariable::PacketVariable;
use crate::services::packetinfo::packetinfo::PacketInfo;

#[derive(Debug, Clone)]
pub struct PacketInfoManager {
    header_id_to_message_incoming: HashMap<i32, Vec<PacketInfo>>,
    header_id_to_message_outgoing: HashMap<i32, Vec<PacketInfo>>,

    hash_to_message_incoming: HashMap<String, Vec<PacketInfo>>,
    hash_to_message_outgoing: HashMap<String, Vec<PacketInfo>>,

    name_to_message_incoming: HashMap<String, Vec<PacketInfo>>,
    name_to_message_outgoing: HashMap<String, Vec<PacketInfo>>,

    packet_info_list: Vec<PacketInfo>
}

impl PacketVariable for PacketInfoManager {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);
        let size: i32 = packet.read();

        let mut list: Vec<PacketInfo> = Vec::new();
        for _ in 0..size {
            list.push(packet.read());
        }

        (PacketInfoManager {
            header_id_to_message_incoming: list.iter()
                .filter(| i | i.destination == HDirection::ToClient)
                .map(| i | (i.header_id, i.clone()))
                .fold(HashMap::new(), | mut res, (k, v)| {
                    res.entry(k).or_insert_with(|| Vec::new()).push(v);
                    res
                }),
            header_id_to_message_outgoing: list.iter()
                .filter(| i | i.destination == HDirection::ToServer)
                .map(| i | (i.header_id, i.clone()))
                .fold(HashMap::new(), | mut res, (k, v)| {
                    res.entry(k).or_insert_with(|| Vec::new()).push(v);
                    res
                }),

            hash_to_message_incoming: list.iter()
                .filter(| i | i.destination == HDirection::ToClient)
                .map(| i | (i.hash.clone(), i.clone()))
                .fold(HashMap::new(), | mut res, (k, v)| {
                    res.entry(k).or_insert_with(|| Vec::new()).push(v);
                    res
                }),
            hash_to_message_outgoing: list.iter()
                .filter(| i | i.destination == HDirection::ToServer)
                .map(| i | (i.hash.clone(), i.clone()))
                .fold(HashMap::new(), | mut res, (k, v)| {
                    res.entry(k).or_insert_with(|| Vec::new()).push(v);
                    res
                }),

            name_to_message_incoming: list.iter()
                .filter(| i | i.destination == HDirection::ToClient)
                .map(| i | (i.name.clone(), i.clone()))
                .fold(HashMap::new(), | mut res, (k, v)| {
                    res.entry(k).or_insert_with(|| Vec::new()).push(v);
                    res
                }),
            name_to_message_outgoing: list.iter()
                .filter(| i | i.destination == HDirection::ToServer)
                .map(| i | (i.name.clone(), i.clone()))
                .fold(HashMap::new(), | mut res, (k, v)| {
                    res.entry(k).or_insert_with(|| Vec::new()).push(v);
                    res
                }),

            packet_info_list: list
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        let mut packet = HPacket::from_header_id(0);
        packet.append(self.packet_info_list.len() as i32);
        for packet_info in self.packet_info_list.iter() {
            packet.append(packet_info.clone());
        }
        packet.get_bytes()[6..].to_vec()
    }
}

impl PacketInfoManager {
    pub fn get_all_packet_info_from_header_id(mut self, direction: HDirection, header_id: i32) -> Vec<PacketInfo> {
        if direction == HDirection::ToClient {
            self.header_id_to_message_incoming.entry(header_id).or_default().clone()
        } else {
            self.header_id_to_message_outgoing.entry(header_id).or_default().clone()
        }
    }

    pub fn get_all_packet_info_from_hash(mut self, direction: HDirection, hash: String) -> Vec<PacketInfo> {
        if direction == HDirection::ToClient {
            self.hash_to_message_incoming.entry(hash).or_default().clone()
        } else {
            self.hash_to_message_outgoing.entry(hash).or_default().clone()
        }
    }

    pub fn get_all_packet_info_from_name(mut self, direction: HDirection, name: String) -> Vec<PacketInfo> {
        if direction == HDirection::ToClient {
            self.name_to_message_incoming.entry(name).or_default().clone()
        } else {
            self.name_to_message_outgoing.entry(name).or_default().clone()
        }
    }

    pub fn get_packet_info_from_header_id(self, direction: HDirection, header_id: i32) -> Option<PacketInfo> {
        let all = self.get_all_packet_info_from_header_id(direction, header_id);
        if all.is_empty() {
            None
        } else {
            Some(all[0].clone())
        }
    }

    pub fn get_packet_info_from_hash(self, direction: HDirection, hash: String) -> Option<PacketInfo> {
        let all = self.get_all_packet_info_from_hash(direction, hash);
        if all.is_empty() {
            None
        } else {
            Some(all[0].clone())
        }
    }

    pub fn get_packet_info_from_name(self, direction: HDirection, name: String) -> Option<PacketInfo> {
        let all = self.get_all_packet_info_from_hash(direction, name);
        if all.is_empty() {
            None
        } else {
            Some(all[0].clone())
        }
    }

    pub fn get_packet_info_list(self) -> Vec<PacketInfo> {
        self.packet_info_list.clone()
    }
}