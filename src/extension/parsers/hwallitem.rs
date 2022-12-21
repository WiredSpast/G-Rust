use std::collections::HashMap;
use crate::protocol::hotel::{CUR_HOTEL, Hotel};
use crate::protocol::hpacket::HPacket;
use crate::protocol::vars::legacy::LegacyId;
use crate::protocol::vars::packetvariable::PacketVariable;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct HWallItem {
    pub id: i64,
    pub type_id: i32,
    pub location: String,
    pub state: String,
    pub seconds_to_expiration: i32,
    pub usage_policy: i32,
    pub owner_id: LegacyId
}

impl PacketVariable for HWallItem {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);
        (HWallItem::new(&mut packet), packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        let mut packet = HPacket::from_header_id(0);
        self.append_to_packet(&mut packet);
        packet.get_bytes()[6..].to_vec()
    }
}

pub type HWallItems = (HashMap<LegacyId, String>, Vec<HWallItem>);

impl HWallItem {
    const UPDATE_PACKET: &'static str = "ItemUpdate|UpdateItem";
    const ADD_PACKET: &'static str = "ItemAdd|AddItem";
    const ITEMS_PACKET: &'static str = "Items";

    pub fn new(packet: &mut HPacket) -> Self {
        let mut res = Self::default();
        res.id = if *CUR_HOTEL.lock().unwrap() == Hotel::Unity {
            packet.read()
        } else {
            packet.read::<String>().parse::<i64>().unwrap()
        };

        (res.type_id, res.location, res.state, res.seconds_to_expiration, res.usage_policy, res.owner_id) = packet.read();

        res.clone()
    }

    pub fn append_to_packet(&self, packet: &mut HPacket) {
        if *CUR_HOTEL.lock().unwrap() == Hotel::Unity {
            packet.append(self.id);
        } else {
            packet.append(self.id.to_string());
        }
        packet.append(self.type_id);
        packet.append(self.location.clone());
        packet.append(self.state.clone());
        packet.append(self.seconds_to_expiration);
        packet.append(self.usage_policy);
        packet.append(self.owner_id.clone());
    }

    pub fn parse(packet: &mut HPacket) -> (HashMap<LegacyId, String>, Vec<Self>) {
        packet.read()
    }

    pub fn construct_packet(id: u16, owners: HashMap<LegacyId, String>, items: Vec<HWallItem>) -> HPacket {
        let mut packet = HPacket::from_header_id(id);
        packet.append(owners);
        packet.append(items);
        packet
    }
}
