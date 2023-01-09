use crate::protocol::hpacket::HPacket;
use crate::protocol::vars::legacy::LegacyId;
use crate::protocol::vars::packetvariable::PacketVariable;
use super::baseparser::BaseParser;

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct HFriend {
    id: LegacyId,
    name: String,
    gender: i32,
    online: bool,
    following_allowed: bool,
    figure: String,
    category_id: i32,
    motto: String,
    real_name: String,
    facebook_id: String,
    persisted_message_user: bool,
    vip_member: bool,
    pocket_habbo_user: bool,
    relationship_status: i16
}