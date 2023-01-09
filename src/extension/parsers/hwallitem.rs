use std::collections::HashMap;
use crate::protocol::hpacket::HPacket;
use crate::protocol::vars::legacy::{LegacyId, LegacyStringId};
use crate::protocol::vars::packetvariable::PacketVariable;
use super::baseparser::BaseParser;

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct HWallItem {
    pub id: LegacyStringId,
    pub type_id: i32,
    pub location: String,
    pub state: String,
    pub seconds_to_expiration: i32,
    pub usage_policy: i32,
    pub owner_id: LegacyId
}

impl HWallItem {
    const UPDATE_PACKET_NAME: &'static str = "ItemUpdate|UpdateItem";
    const ADD_PACKET_NAME: &'static str = "ItemAdd|AddItem";
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct HWallItems {
    pub owners: HashMap<LegacyId, String>,
    pub items: Vec<HWallItem>
}

impl HWallItems {
    const ITEMS_PACKET_NAME: &'static str = "Items";
}
