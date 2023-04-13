use std::cmp::max;
use std::collections::HashMap;
use crate::extension::parsers::stuffdata::StuffData;
use crate::protocol::hpacket::HPacket;
use crate::protocol::vars::legacy::{LegacyId, LegacyLength, LegacyDouble};
use crate::protocol::vars::packetvariable::PacketVariable;

#[derive(Clone, Debug, Default, PacketVariable, PartialEq, Eq)]
pub struct FurnitureProductItem {
    pub product_code: String,
    pub furniture_class_name: String
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct OutgoingIngredient {
    pub count: i32,
    pub furniture_class_name: String
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ExtendedProfileData {
    pub user_id: LegacyId,
    pub user_name: String,
    pub figure: String,
    pub motto: String,
    pub creation_date: String,
    pub achievement_score: i32,
    pub friend_count: i32, // Might be a LegacyLength
    pub is_friend: bool,
    pub is_friend_request_sent: bool,
    pub is_online: bool,
    pub guilds: Vec<HabboGroupEntryData>,
    pub last_access_since_in_seconds: i32,
    pub open_profile_window: bool,
    pub _unused1: Option<bool>,
    pub account_level: Option<i32>,
    pub _unused2: Option<i32>,
    pub star_gem_count: Option<i32>,
    pub _unused3: Option<bool>,
    pub _unused4: Option<bool>
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct HabboGroupEntryData {
    pub group_id: LegacyId,
    pub group_name: String,
    pub badge_code: String,
    pub primary_color: String,
    pub secondary_color: String,
    pub favourite: bool,
    pub owner_id: LegacyId,
    pub has_forum: bool
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct MemberData {
    pub role: i32,
    pub user_id: LegacyId,
    pub user_name: String,
    pub figure: String,
    pub member_since: String
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GuildCreationData {
    pub cost_in_credits: i32,
    pub owned_rooms: Vec<RoomEntryData>,
    pub badge_settings: Vec<GuildBadgeSettings>
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RoomEntryData {
    pub room_id: LegacyId,
    pub room_name: String,
    pub has_controllers: bool
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GuildBadgeSettings {
    pub part_id: i32,
    pub color_id: i32,
    pub position: i32
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GuildEditData {
    pub owned_rooms: Vec<RoomEntryData>,
    pub is_owner: bool,
    pub group_id: LegacyId,
    pub group_name: String,
    pub group_desc: String,
    pub base_room_id: LegacyId,
    pub primary_color_id: i32, // Might be just a LegacyId
    pub secondary_color_id: i32, // Might be just a LegacyId
    pub guild_type: i32,
    pub guild_rights_level: i32,
    pub locked: bool,
    pub url: String,
    pub badge_settings: Vec<GuildBadgeSettings>,
    pub badge_code: String,
    pub membership_count: i32
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct BadgePartData {
    pub id: i32, // Might be a LegacyId
    pub file_name: String,
    pub mask_file_name: String,
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GuildColorData {
    pub id: i32, // Might be a LegacyId
    pub color: String
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GuildMemberData {
    pub group_id: LegacyId,
    pub group_name: String,
    pub base_room_id: LegacyId,
    pub badge_code: String,
    pub total_entries: i32, // Might be a LegacyLength
    pub entries: Vec<MemberData>,
    pub allowed_to_manage: bool,
    pub page_size: i32,
    pub page_index: i32,
    pub search_type: i32,
    pub user_name_filter: String
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct HabboGroupDetailsData {
    pub group_id: LegacyId,
    pub is_guild: bool,
    pub group_type: i32,
    pub group_name: String,
    pub description: String,
    pub badge_code: String,
    pub room_id: LegacyId,
    pub room_name: String,
    pub status: i32,
    pub total_members: i32, // Might be a LegacyLength
    pub favourite: bool,
    pub creation_date: String,
    pub is_owner: bool,
    pub is_admin: bool,
    pub open_details: bool,
    pub members_can_decorate: bool,
    pub pending_member_count: i32, // Might be a LegacyLength
    pub has_board: bool
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PetData {
    pub id: LegacyId,
    pub name: String,
    pub figure_data: PetFigureData,
    pub level: i32
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PetFigureData {
    pub type_id: i32, // Might be a LegacyId
    pub palette_id: i32, // Might be a LegacyId
    pub color: String,
    pub breed_id: i32, // Might be a LegacyId
    pub custom_parts: Vec<(i32, i32, i32)>
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RelationshipStatusInfoData {
    pub relationship_status_type: i32,
    pub friend_count: i32,
    pub random_friend_id: LegacyId,
    pub random_friend_name: String,
    pub random_friend_figure: String
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ScrKickbackData {
    pub current_hc_streak: i32,
    pub first_subscription_date: String,
    pub kickback_percentage: f64,
    pub total_credits_missed: i32,
    pub total_credits_rewarded: i32,
    pub total_credits_spent: i32,
    pub credit_reward_for_streak_bonus: i32,
    pub credit_reward_for_monthly_spent: i32,
    pub time_until_payday: i32
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RoomQueueSet {
    pub name: String,
    pub target: i32, // Might be a LegacyId
    pub queue: HashMap<String, i32> // Might be <String, LegacyId
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct BaseForumData {
    pub group_id: LegacyId,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub total_threads: i32,
    pub leaderboard_score: i32,
    pub total_messages: i32,
    pub unread_messages: i32,
    pub last_message_id: i32, // Might be a LegacyId
    pub last_message_author_id: LegacyId,
    pub last_message_author_name: String,
    pub last_message_time_as_seconds_ago: i32
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ExtendedForumData {
    pub base: BaseForumData,
    pub read_permissions: i32,
    pub post_message_permissions: i32,
    pub post_thread_permissions: i32,
    pub moderate_permissions: i32,
    pub read_permission_error: String,
    pub post_message_permission_error: String,
    pub post_thread_permission_error: String,
    pub moderate_permission_error: String,
    pub report_permissions_error: String,
    pub can_change_settings: bool,
    pub is_staff: bool
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ThreadData {
    pub thread_id: LegacyId,
    pub thread_author_id: LegacyId,
    pub thread_author_name: String,
    pub header: String,
    pub is_sticky: bool,
    pub is_locked: bool,
    pub creation_time_as_seconds_ago: i32,
    pub n_messages: i32,
    pub n_unread_messages: i32,
    pub last_message_id: LegacyId,
    pub last_message_author_id: LegacyId,
    pub last_message_author_name: String,
    pub last_message_time_as_seconds_ago: i32,
    pub state: i8,
    pub admin_id: LegacyId,
    pub admin_name: String,
    pub admin_operation_time_as_seconds_ago: i32
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct MessageData {
    pub message_id: LegacyId,
    pub message_index: i32,
    pub author_id: LegacyId,
    pub author_name: String,
    pub author_figure: String,
    pub creation_time_as_seconds_ago: i32,
    pub message_text: String,
    pub state: i8,
    pub admin_id: LegacyId,
    pub admin_name: String,
    pub admin_operation_time_as_seconds_ago: i32,
    pub author_post_count: i32
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct AchievementResolutionData {
    pub achievement_id: LegacyId,
    pub level: i32,
    pub badge_id: String,
    pub required_level: i32,
    pub state: i32
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CategoriesWithVisitorCountData {
    pub categories: Vec<CategoriesWithVisitorCountDataEntry>
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CategoriesWithVisitorCountDataEntry {
    pub category_id: i32, // Might be a LegacyId
    pub current_user_count: i32,
    pub max_user_count: i32
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GuestRoomData {
    pub flat_id: LegacyId,
    pub room_name: String,
    pub owner_id: LegacyId,
    pub owner_name: String,
    pub door_mode: i32,
    pub user_count: i32,
    pub max_user_count: i32,
    pub description: String,
    pub trade_mode: i32,
    pub score: i32,
    pub ranking: i32,
    pub category_id: i32, // Might be a LegacyId
    pub tags: Vec<String>,
    pub official_room_pic_ref: Option<String>,
    pub habbo_group_id: Option<LegacyId>,
    pub group_name: Option<String>,
    pub group_badge_code: Option<String>,
    pub room_ad_name: Option<String>,
    pub room_ad_description: Option<String>,
    pub room_ad_expires_in_min: Option<i32>,
    pub show_owner: bool,
    pub allow_pets: bool,
    pub display_room_entry_ad: bool
}

impl PacketVariable for GuestRoomData {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);
        let (flat_id, room_name, owner_id, owner_name, door_mode, user_count, max_user_count,
            description, trade_mode, score, ranking, category_id, tags) = packet.read();

        let multi_use: i32 = packet.read();

        let official_room_pic_ref =
            if (multi_use & 1) > 0 { Some(packet.read()) }
            else { None };

        let (habbo_group_id, group_name, group_badge_code) =
            if (multi_use & 2) > 0 { packet.read() }
            else { (None, None, None) };

        let (room_ad_name, room_ad_description, room_ad_expires_in_min) =
            if (multi_use & 4) > 0 { packet.read() }
            else { (None, None, None) };

        let show_owner = (multi_use & 8) > 0;
        let allow_pets = (multi_use & 16) > 0;
        let display_room_entry_ad = (multi_use & 32) > 0;

        ( Self { flat_id, room_name, owner_id, owner_name, door_mode, user_count, max_user_count,
            description, trade_mode, score, ranking, category_id, tags, official_room_pic_ref,
            habbo_group_id, group_name, group_badge_code, room_ad_name, room_ad_description,
            room_ad_expires_in_min, show_owner, allow_pets, display_room_entry_ad },
          packet.read_index - 6
        )
    }

    fn to_packet(&self) -> Vec<u8> {
        let mut packet = HPacket::from_header_id(0);
        packet.append((self.flat_id, self.room_name.clone(), self.owner_id,
                       self.owner_name.clone(), self.door_mode, self.user_count,
                       self.max_user_count, self.description.clone(), self.trade_mode,
                       self.score, self.ranking, self.category_id, self.tags.clone()));

        let mut multi_use = 0;
        if self.official_room_pic_ref.is_some() {
            multi_use |= 1;
        }
        if self.habbo_group_id.is_some() && self.group_name.is_some() && self.group_badge_code.is_some() {
            multi_use |= 2;
        }
        if self.room_ad_name.is_some() && self.room_ad_description.is_some() && self.room_ad_expires_in_min.is_some() {
            multi_use |= 4;
        }
        if self.show_owner {
            multi_use |= 8;
        }
        if self.allow_pets {
            multi_use |= 16;
        }
        if self.display_room_entry_ad {
            multi_use |= 32;
        }

        packet.append(multi_use);
        if self.official_room_pic_ref.is_some() {
            packet.append(self.official_room_pic_ref.clone());
        }
        if self.habbo_group_id.is_some() && self.group_name.is_some() && self.group_badge_code.is_some() {
            packet.append((self.habbo_group_id, self.group_name.clone(), self.group_badge_code.clone()));
        }
        if self.room_ad_name.is_some() && self.room_ad_description.is_some() && self.room_ad_expires_in_min.is_some() {
            packet.append((self.room_ad_name.clone(), self.room_ad_description.clone(), self.room_ad_expires_in_min));
        }

        packet.get_bytes()[6..].to_vec()
    }
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RoomModerationSettings {
    pub who_can_mute: i32,
    pub who_can_kick: i32,
    pub who_can_ban: i32
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GuestRoomSearchResultData {
    pub search_type: i32,
    pub search_param: String,
    pub rooms: Vec<GuestRoomData>,
    pub ad: Option<OfficialRoomEntryData>
}

impl PacketVariable for GuestRoomSearchResultData {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        let (search_type, search_param, rooms, has_ad) = packet.read();
        let ad = if has_ad { packet.read() } else { None };

        (GuestRoomSearchResultData { search_type, search_param, rooms, ad }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        let mut packet = HPacket::from_header_id(0);

        packet.append((self.search_type, self.search_param.clone(), self.rooms.clone(),
                       self.ad.is_some(), self.ad.clone()));

        packet.get_bytes()[6..].to_vec()
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct OfficialRoomEntryData {
    pub index: i32,
    pub popup_caption: String,
    pub popup_desc: String,
    pub show_details: bool,
    pub pic_text: String,
    pub pic_ref: String,
    pub folder_id: i32, // Might be a LegacyId
    pub user_count: i32,
    pub entry_type: i32,
    pub tag: Option<String>,
    pub guest_room_data: Option<GuestRoomData>,
    pub open: Option<bool>
}

impl PacketVariable for OfficialRoomEntryData {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        let (index, popup_caption, popup_desc, show_details, pic_text,
            pic_ref, folder_id, user_count, entry_type) = packet.read();

        let (mut tag, mut guest_room_data, mut open) = (None, None, None);
        if entry_type == 1 {
            tag = packet.read();
        } else if entry_type == 2 {
            guest_room_data = packet.read();
        } else {
            open = packet.read();
        }

        (
            OfficialRoomEntryData { index, popup_caption, popup_desc, show_details, pic_text,
                pic_ref, folder_id, user_count, entry_type, tag, guest_room_data, open },
            packet.read_index - 6
        )
    }

    fn to_packet(&self) -> Vec<u8> {
        let mut packet = HPacket::from_header_id(0);

        packet.append((self.index, self.popup_caption.clone(), self.popup_desc.clone(),
                       self.show_details, self.pic_text.clone(), self.pic_ref.clone(),
                       self.folder_id, self.user_count, self.entry_type));
        if self.entry_type == 1 {
            packet.append(self.tag.clone());
        } else if self.entry_type == 2 {
            packet.append(self.guest_room_data.clone());
        } else {
            packet.append(self.open);
        }

        packet.get_bytes()[6..].to_vec()
    }
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct OfficialRoomsData {
    pub entries: Vec<OfficialRoomEntryData>
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PromotedRoomsData {
    pub entries: Vec<PromotedRoomCategoryData>
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PromotedRoomCategoryData {
    pub code: String,
    pub leader_figure: String,
    pub rooms: Vec<GuestRoomData>
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PopularRoomTagsData {
    pub tags: Vec<PopularTagData>
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PopularTagData {
    pub tag_name: String,
    pub user_count: i32
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RoomEventData {
    pub ad_id: LegacyId,
    pub owner_avatar_id: LegacyId,
    pub owner_avatar_name: String,
    pub flat_id: LegacyId,
    pub event_type: i32,
    pub event_name: String,
    pub event_description: String,
    pub creation_time: i32,
    pub expiration_date: i32,
    pub category_id: i32 // Might be a LegacyId
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct EventCategory {
    pub category_id: i32, // Might be a LegacyId
    pub category_name: String,
    pub visible: bool
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct FlatCategory {
    pub node_id: i32, // Might be a LegacyId
    pub node_name: String,
    pub visible: bool,
    pub automatic: bool,
    pub automatic_category_key: String,
    pub global_category_key: String,
    pub staff_only: bool
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RoomDimmerPresetsMessageData {
    pub id: LegacyId,
    pub dimmer_type: i32,
    pub color: String,
    pub light: i32
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct YoutubeDisplayPlaylist {
    pub playlist_id: String,
    pub title: String,
    pub description: String
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CfhChatlogData {
    pub call_id: LegacyId,
    pub caller_user_id: LegacyId,
    pub reported_user_id: LegacyId,
    pub chat_record_id: LegacyId,
    pub chat_record: ChatRecordData
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ChatRecordData {
    pub record_type: i8,
    pub context: HashMap<String, ChatRecordDataValue>,
    pub chat_log: Vec<ChatlineData>
}

#[derive(Clone, Debug, PartialEq)]
pub enum ChatRecordDataValue {
    String(String),
    Int(i32),
    Bool(bool)
}

impl PacketVariable for ChatRecordData {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        let record_type = packet.read();
        let mut context = HashMap::new();
        let length: i16 = packet.read();
        for _ in 0..length {
            let key = packet.read();
            let val = match packet.read::<i8>() {
                0 => ChatRecordDataValue::Bool(packet.read()),
                1 => ChatRecordDataValue::Int(packet.read()),
                2 => ChatRecordDataValue::String(packet.read()),
                _ => panic!("Unknown data type")
            };
            context.insert(key, val);
        }
        let chat_log = packet.read();

        (ChatRecordData { record_type, context, chat_log }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        let mut packet = HPacket::from_header_id(0);

        packet.append(self.record_type);
        packet.append(self.context.len() as i16);
        for (key, val) in self.context.clone() {
            packet.append(key);
            match val {
                ChatRecordDataValue::Bool(b) => packet.append(b),
                ChatRecordDataValue::Int(i) => packet.append(i),
                ChatRecordDataValue::String(s) => packet.append(s.clone())
            };
        }
        packet.append(self.chat_log.clone());

        packet.get_bytes()[6..].to_vec()
    }
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ChatlineData {
    pub timestamp: String,
    pub chatter_id: LegacyId,
    pub chatter_name: String,
    pub msg: String,
    pub has_highlighting: bool
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct IssueMessageData {
    pub issue_id: LegacyId,
    pub state: i32,
    pub category_id: LegacyId,
    pub reported_category_id: i32, // Might be a LegacyId
    pub issue_age_in_milliseconds: i32,
    pub priority: i32,
    pub grouping_id: i32, // Might be a LegacyId
    pub reporter_user_id: LegacyId,
    pub reporter_user_name: String,
    pub reported_user_id: LegacyId,
    pub reported_user_name: String,
    pub picker_user_id: LegacyId,
    pub picker_user_name: String,
    pub message: String,
    pub chat_record_id: LegacyId,
    pub patterns: Vec<PatternMatchData>
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PatternMatchData {
    pub pattern: String,
    pub start_index: i32,
    pub end_index: i32
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ModeratorInitData {
    pub issues: Vec<IssueMessageData>,
    pub message_templates: Vec<String>,
    pub _unused: Vec<String>,
    pub cfh_permission: bool,
    pub chatlogs_permission: bool,
    pub alert_permission: bool,
    pub kick_permission: bool,
    pub ban_permission: bool,
    pub room_alert_permission: bool,
    pub room_kick_permission: bool,
    pub room_message_templates: Vec<String>
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RoomModerationData {
    pub flat_id: LegacyId,
    pub user_count: i32,
    pub owner_in_room: bool,
    pub owner_id: LegacyId,
    pub owner_name: String,
    pub room: RoomData
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RoomData {
    pub exists: bool,
    pub name: Option<String>,
    pub desc: Option<String>,
    pub tags: Option<Vec<String>>
}

impl PacketVariable for RoomData {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        let exists = packet.read();
        (RoomData {
            exists,
            name: if exists { packet.read() } else { None },
            desc: if exists { packet.read() } else { None },
            tags: if exists { packet.read() } else { None },
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        let mut packet = HPacket::from_header_id(0);

        packet.append(self.exists);
        if self.exists {
            packet.append((self.name.clone(), self.desc.clone(), self.tags.clone()));
        }

        packet.get_bytes()[6..].to_vec()
    }
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ModeratorUserInfoData {
    pub user_id: LegacyId,
    pub user_name: String,
    pub figure: String,
    pub registration_age_in_minutes: i32,
    pub minutes_since_last_login: i32,
    pub online: bool,
    pub cfh_count: i32,
    pub abusive_cfh_count: i32,
    pub caution_count: i32,
    pub ban_count: i32,
    pub trading_lock_count: i32,
    pub trading_expiry_date: String,
    pub last_purchase_date: String,
    pub identity_id: LegacyId,
    pub identity_related_ban_count: i32,
    pub primary_email_address: String,
    pub user_classification: String,
    pub last_sanction_time: Option<String>,
    pub sanction_age_hours: Option<i32>
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RoomVisitsData {
    pub user_id: LegacyId,
    pub user_name: String,
    pub rooms: Vec<RoomVisitData>
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RoomVisitData {
    pub room_id: LegacyId,
    pub room_name: String,
    pub enter_hour: i32,
    pub enter_minute: i32
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct UserChatlogData {
    pub user_id: LegacyId,
    pub user_name: String,
    pub rooms: Vec<ChatRecordData>
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PollQuestion {
    pub question_id: LegacyId,
    pub sort_order: i32,
    pub question_type: i32,
    pub question_text: String,
    pub question_category: i32,
    pub question_answer_type: i32,
    pub question_answer_count: LegacyLength,
    pub question_choices: Vec<PollChoice>,
    pub children: Vec<PollQuestion>
}

impl PacketVariable for PollQuestion {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        let (question_id, sort_order, question_type, question_text, question_category,
            question_answer_type) = packet.read();
        let question_answer_count: LegacyLength = packet.read();
        let mut question_choices = Vec::new();
        let children = Vec::new();
        if question_type == 1 || question_type == 2 {
            for _ in 0..*question_answer_count {
                question_choices.push(packet.read())
            }
        }

        (PollQuestion {
            question_id, sort_order, question_type, question_text, question_category,
            question_answer_type, question_answer_count, question_choices, children
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        let mut packet = HPacket::from_header_id(0);

        packet.append((
            self.question_id, self.sort_order, self.question_type, self.question_text.clone(),
            self.question_category, self.question_answer_type, self.question_answer_count
        ));
        if self.question_type == 1 || self.question_type == 2 {
            for choice in self.question_choices.iter() {
                packet.append(choice.clone());
            }
        }

        packet.get_bytes()[6..].to_vec()
    }
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PollChoice {
    pub value: String,
    pub choice_text: String,
    pub choice_type: i32
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct QuestionData {
    pub id: LegacyId,
    pub number: i32,
    pub question_type: i32,
    pub content: String,
    pub selection_min: i32,
    pub selection_values: Vec<String>,
    pub selections: Vec<String>
}

impl PacketVariable for QuestionData {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        let (id, number, question_type, content) = packet.read();

        let mut selection_values = Vec::new();
        let mut selections = Vec::new();
        let mut selection_min = -1;

        if question_type == 1 || question_type == 2 {
            selection_min = packet.read();
            let count: i32 = packet.read();
            for _ in 0..count {
                selection_values.push(packet.read());
                selections.push(packet.read());
            }
        }

        (Self {
            id, number, question_type, content,
            selection_min, selection_values, selections
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        let mut packet = HPacket::from_header_id(0);

        packet.append((self.id, self.number, self.question_type, self.content.clone()));
        if self.question_type == 1 || self.question_type == 2 {
            let len = max(self.selections.len(), self.selection_values.len());
            packet.append((
                self.selection_min,
                len as i32
            ));
            for i in 0..len {
                packet.append((
                    self.selection_values.get(i).unwrap().clone(),
                    self.selections.get(i).unwrap().clone()
                ));
            }
        }

        packet.get_bytes()[6..].to_vec()
    }
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CatalogNodeData {
    pub visible: bool,
    pub icon: i32,
    pub page_id: LegacyId,
    pub page_name: String,
    pub localization: String,
    pub offer_ids: Vec<LegacyId>,
    pub children: Vec<CatalogNodeData>
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CatalogLocalizationData {
    pub images: Vec<String>,
    pub texts: Vec<String>
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CatalogPageMessageOfferData {
    pub offer_id: LegacyId,
    pub localization_id: String,
    pub is_rent: bool,
    pub price_in_credits: i32,
    pub price_in_activity_points: i32,
    pub activity_point_type: i32,
    pub giftable: bool,
    pub products: Vec<CatalogPageMessageProductData>,
    pub club_level: i32,
    pub bundle_purchase_allowed: bool,
    pub _unused: bool,
    pub preview_image: String
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CatalogPageMessageProductData {
    pub product_type: String,
    pub furni_class_id: LegacyId,
    pub extra_param: String,
    pub product_count: i32,
    pub unique_limited_item: bool,
    pub unique_limited_item_series_size: i32,
    pub unique_limited_items_left: i32
}

impl PacketVariable for CatalogPageMessageProductData {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        let product_type: String = packet.read();
        let mut furni_class_id = LegacyId(-1);
        let extra_param: String;
        let mut product_count = 1;
        let mut unique_limited_item = false;
        let mut unique_limited_item_series_size = -1;
        let mut unique_limited_items_left = -1;
        if product_type.ne(&String::from("b")) {
            (furni_class_id, extra_param, product_count, unique_limited_item) = packet.read();
            if unique_limited_item {
                (unique_limited_item_series_size, unique_limited_items_left) = packet.read();
            }
        } else {
            extra_param = packet.read();
        }

        (Self {
            product_type,
            furni_class_id,
            extra_param: extra_param.clone(),
            product_count,
            unique_limited_item,
            unique_limited_item_series_size,
            unique_limited_items_left
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        let mut packet = HPacket::from_header_id(0);

        packet.append(self.product_type.clone());

        if self.product_type.ne(&String::from("b")) {
            packet.append((
                self.furni_class_id, self.extra_param.clone(),
                self.product_count, self.unique_limited_item
            ));
            if self.unique_limited_item {
                packet.append((
                    self.unique_limited_item_series_size,
                    self.unique_limited_items_left
                ));
            }
        } else {
            packet.append(self.extra_param.clone());
        }

        packet.get_bytes()[6..].to_vec()
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FrontPageItem {
    pub position: i32,
    pub item_name: String,
    pub item_promo_image: String,
    pub item_type: i32,
    pub catalogue_page_location: String,
    pub product_offer_id: LegacyId,
    pub product_code: String,
    pub expiration_time: i32
}

impl PacketVariable for FrontPageItem {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        let mut res = Self::default();
        (res.position, res.item_name, res.item_promo_image, res.item_type) = packet.read();
        match res.item_type {
            0 => res.catalogue_page_location = packet.read(),
            1 => res.product_offer_id = packet.read(),
            2 => res.product_code = packet.read(),
            _ => panic!("Unknown item type")
        }
        res.expiration_time = packet.read();

        (res.clone(), packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        let mut packet = HPacket::from_header_id(0);

        packet.append((
            self.position, self.item_name.clone(),
            self.item_promo_image.clone(), self.item_type
        ));
        match self.item_type {
            0 => packet.append(self.catalogue_page_location.clone()),
            1 => packet.append(self.product_offer_id),
            2 => packet.append(self.product_code.clone()),
            _ => panic!("Unknown item type")
        }
        packet.append(self.expiration_time);

        packet.get_bytes()[6..].to_vec()
    }
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ClubGiftData {
    pub offer_id: LegacyId,
    pub is_vip: bool,
    pub days_required: i32,
    pub is_selectable: bool
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ClubOfferExtendData {
    pub original_price: i32,
    pub original_activity_point_price: i32,
    pub original_activity_point_type: i32,
    pub discount_credit_amount: i32
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PurchaseOKMessageOfferData {
    pub offer_id: LegacyId,
    pub localization_id: String,
    pub is_rent: bool,
    pub price_in_credits: i32,
    pub price_in_activity_points: i32,
    pub activity_point_type: i32,
    pub giftable: bool,
    pub products: Vec<CatalogPageMessageProductData>,
    pub club_level: i32,
    pub bundle_purchase_allowed: bool
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct SellablePetPaletteData {
    pub palette_type: i32,
    pub breed_id: LegacyId,
    pub palette_id: LegacyId,
    pub sellable: bool,
    pub rare: bool
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct SnowWarGameTokenOffer {
    pub offer_id: LegacyId,
    pub localization_id: String,
    pub price_in_credits: i32,
    pub price_in_activity_points: i32,
    pub activity_point_type: i32
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct HeightMapTile {
    pub height: f32,
    pub is_stacking_blocked: bool,
    pub is_room_tile: bool
}

impl PacketVariable for HeightMapTile {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        let val: i16 = packet.read();

        (Self {
            height: if val < 0 { -1.0 } else { (val & 16383) as f32 / 256.0 },
            is_stacking_blocked: (val & 16384) > 0,
            is_room_tile: val >= 0
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        if !self.is_room_tile {
            (-1i16).to_packet()
        } else {
            let mut val: i16 = 0;

            if self.is_stacking_blocked {
                val |= 16384;
            }
            val |= (self.height * 256.0) as i16 & 16383;

            val.to_packet()
        }
    }
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct HeightMapTileUpdate {
    pub x: i16,
    pub y: i16,
    pub tile: HeightMapTile
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct WallItem {
    pub id: LegacyId,
    pub type_id: i32,
    pub location: String,
    pub data_str: String,
    pub usage_policy: i32,
    pub owner_id: LegacyId,
    pub owner_name: Option<String>
}

impl PacketVariable for WallItem {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        let (id, type_id, location, data_str, usage_policy, owner_id) = packet.read();

        (Self {
            id, type_id, location, data_str, usage_policy, owner_id,
            owner_name: None
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        (
            self.id, self.type_id, self.location.clone(),
            self.data_str.clone(), self.usage_policy, self.owner_id
        ).to_packet()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FloorItem {
    pub id: LegacyId,
    pub type_id: i32,
    pub x: i32,
    pub y: i32,
    pub dir: i32,
    pub z: LegacyDouble,
    pub size_z: LegacyDouble,
    pub extra: i32,
    pub data: StuffData,
    pub expiry_time: i32,
    pub usage_policy: i32,
    pub owner_id: i32,
    pub owner_name: Option<String>,
    pub static_class: String
}

impl PacketVariable for FloorItem {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        let (id, type_id, x, y, dir, z, size_z, extra, data, expiry_time, usage_policy, owner_id) = packet.read();

        (Self {
            id, type_id, x, y, dir, z, size_z, extra, data, expiry_time, usage_policy, owner_id,
            owner_name: None,
            static_class: if *id < 0 { packet.read() } else { "".to_string()}
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        let mut packet = HPacket::from_header_id(0);
        packet.append((
            self.id, self.type_id, self.x, self.y, self.dir, self.z, self.size_z,
            self.extra, self.data.clone(), self.expiry_time, self.usage_policy, self.owner_id
        ));

        if *self.id < 0 {
            packet.append(self.static_class.clone());
        }

        packet.get_bytes()[6..].to_vec()
    }
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq, Eq)]
pub struct SlideObjectHeight {
    pub old_z: i32,
    pub new_z: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum User {
    Player {
        id: LegacyId,
        name: String,
        custom: String,
        figure: String,
        room_index: i32,
        x: i32,
        y: i32,
        z: LegacyDouble,
        dir: i32,
        sex: String,
        group_id: LegacyId,
        group_status: i32,
        group_name: String,
        swim_figure: String,
        achievement_score: i32,
        is_moderator: bool
    },
    Pet {
        id: LegacyId,
        name: String,
        custom: String,
        figure: String,
        room_index: i32,
        x: i32,
        y: i32,
        z: LegacyDouble,
        dir: i32,
        sub_type: i32,
        owner_id: LegacyId,
        owner_name: String,
        rarity_level: i32,
        has_saddle: bool,
        is_riding: bool,
        can_breed: bool,
        can_harvest: bool,
        can_revive: bool,
        has_breeding_permission: bool,
        pet_level: i32,
        pet_posture: String
    },
    OldBot {
        id: LegacyId,
        name: String,
        custom: String,
        figure: String,
        room_index: i32,
        x: i32,
        y: i32,
        z: LegacyDouble,
        dir: i32
    },
    Bot {
        id: LegacyId,
        name: String,
        custom: String,
        figure: String,
        room_index: i32,
        x: i32,
        y: i32,
        z: LegacyDouble,
        dir: i32,
        sex: String,
        owner_id: LegacyId,
        owner_name: String,
        bot_skills: Vec<i16>
    }
}

impl PacketVariable for User {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        let (id, name, custom, figure, room_index, x, y, z, dir) = packet.read();

        (match packet.read::<i32>() {
            1 => User::Player {
                id, name, custom, figure, room_index, x, y, z, dir,
                sex: packet.read(),
                group_id: packet.read(),
                group_status: packet.read(),
                group_name: packet.read(),
                swim_figure: packet.read(),
                achievement_score: packet.read(),
                is_moderator: packet.read()
            },
            2 => User::Pet {
                id, name, custom, figure, room_index, x, y, z, dir,
                sub_type: packet.read(),
                owner_id: packet.read(),
                owner_name: packet.read(),
                rarity_level: packet.read(),
                has_saddle: packet.read(),
                is_riding: packet.read(),
                can_breed: packet.read(),
                can_harvest: packet.read(),
                can_revive: packet.read(),
                has_breeding_permission: packet.read(),
                pet_level: packet.read(),
                pet_posture: packet.read()
            },
            3 => User::OldBot {
                id, name, custom, figure, room_index, x, y, z, dir
            },
            4 => User::Bot {
                id,
                name,
                custom,
                figure,
                room_index,
                x,
                y,
                z,
                dir,
                sex: packet.read(),
                owner_id: packet.read(),
                owner_name: packet.read(),
                bot_skills: packet.read()
            },
            _ => panic!("Unknown user type")
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        match self {
            User::Player { id, name, custom, figure,
                room_index, x, y, z, dir, sex,
                group_id, group_status, group_name, swim_figure,
                achievement_score, is_moderator } => {
                (
                    *id, name.clone(), custom.clone(), figure.clone(), *room_index, *x, *y, *z,
                    *dir, 1, sex.clone(), *group_id, *group_status, group_name.clone(),
                    swim_figure.clone(), *achievement_score, *is_moderator
                ).to_packet()
            },
            User::Pet { id, name, custom, figure,
                room_index, x, y, z, dir, sub_type,
                owner_id, owner_name, rarity_level, has_saddle,
                is_riding, can_breed, can_harvest, can_revive,
                has_breeding_permission, pet_level, pet_posture } => {
                (
                    *id, name.clone(), custom.clone(), figure.clone(), *room_index, *x, *y, *z,
                    *dir, 2, *sub_type, *owner_id, owner_name.clone(), *rarity_level, *has_saddle,
                    *is_riding, *can_breed, *can_harvest, *can_revive, *has_breeding_permission,
                    *pet_level, pet_posture.clone()
                ).to_packet()
            },
            User::OldBot { id, name, custom, figure,
                room_index, x, y, z, dir } => {
                (
                    *id, name.clone(), custom.clone(), figure.clone(),
                    *room_index, *x, *y, *z, *dir, 3
                ).to_packet()
            },
            User::Bot { id, name, custom, figure,
                room_index, x, y, z, dir,
                sex, owner_id, owner_name, bot_skills } => {
                (
                    *id, name.clone(), custom.clone(), figure.clone(),
                    *room_index, *x, *y, *z, *dir, 4,
                    sex.clone(), *owner_id, owner_name.clone(), bot_skills.clone()
                ).to_packet()
            }
        }
    }
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct UserUpdateMessageData {
    pub index: i32,
    pub x: i32,
    pub y: i32,
    pub z: LegacyDouble,
    pub dir_head: i32,
    pub dir: i32,
    pub actions: Vec<UserUpdateAction>
}

#[derive(Clone, Debug, PartialEq)]
pub enum UserUpdateAction {
    UserFlatControl {
        level: i32
    },
    SkipPositionUpdate,
    Move {
        x: i32,
        y: i32,
        z: f64
    },
    Sit {
        local_z: f64,
        can_stand_up: bool
    },
    Lay {
        local_z: f64,
    },
    Sign {
        id: i32
    },
    Gesture {
        gesture: Gesture
    },
    Posture {
        posture: Posture
    }
}

impl UserUpdateAction {
    fn identify(action: &str) -> Option<Self> {
        let split: Vec<&str> = action.split(" ").collect();

        match split[..] {
            [ "wf" ] => Some(Self::SkipPositionUpdate),
            [ key ] => Some(Self::Posture {
                posture: Posture::identify(key)
            }),
            [ "flatctrl", lvl ] => Some(Self::UserFlatControl {
                level: lvl.parse::<i32>().unwrap()
            }),
            [ "sit", z ] => Some(Self::Sit {
                local_z: z.parse::<f64>().unwrap(),
                can_stand_up: false
            }),
            [ "sit", z, stand ] => Some(Self::Sit {
                local_z: z.parse::<f64>().unwrap(),
                can_stand_up: stand == "1"
            }),
            [ "gst", key ] => Some(Self::Gesture {
                gesture: Gesture::identify(key)
            }),
            [ "sign", num ] => Some(Self::Sign {
                id: num.parse::<i32>().unwrap()
            }),
            [ "lay", z ] => Some(Self::Lay {
                local_z: z.parse::<f64>().unwrap()
            }),
            [ "mv", coords ] => {
                let split_coords: Vec<&str> = coords.split(',').collect();
                Some(Self::Move {
                    x: split_coords[0].parse::<i32>().unwrap(),
                    y: split_coords[1].parse::<i32>().unwrap(),
                    z: split_coords[2].parse::<f64>().unwrap()
                })
            },
            [ .. ] => None
        }
    }
}

impl ToString for UserUpdateAction {
    fn to_string(&self) -> String {
        match self {
            UserUpdateAction::UserFlatControl { level } =>
                format!("{level}"),
            UserUpdateAction::SkipPositionUpdate =>
                format!("wf"),
            UserUpdateAction::Move { x, y, z } =>
                format!("mv {x},{y},{z}"),
            UserUpdateAction::Sit { local_z, can_stand_up } =>
                format!("sit {local_z} {}", *can_stand_up as i32),
            UserUpdateAction::Lay { local_z } =>
                format!("lay {local_z}"),
            UserUpdateAction::Sign { id } =>
                format!("sign {id}"),
            UserUpdateAction::Gesture { gesture } =>
                format!("gst {}", gesture.to_string()),
            UserUpdateAction::Posture { posture } =>
                format!("{}", posture.to_string())
        }
    }
}

impl PacketVariable for Vec<UserUpdateAction> {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let (str, size) = String::from_packet(bytes);

        let mut actions = Vec::new();
        for s in str.split('/') {
            if s != "" {
                let action = UserUpdateAction::identify(s);
                if action.is_some() {
                    actions.push(action.unwrap())
                }
            }
        }

        (actions, size)
    }

    fn to_packet(&self) -> Vec<u8> {
        let mut res = "/".to_string();
        for action in self {
            res += action.to_string().as_str();
            res += "/"
        }

        res.to_packet()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Posture {
    Lay,
    Move,
    Stand,
    Swim,
    Float,
    Sit,
    SnowWarRun,
    SnowWarDieFront,
    SnowWarDieBack,
    SnowWarPick,
    SnowWarThrow,
    Other(String)
}

impl ToString for Posture {
    fn to_string(&self) -> String {
        match self {
            Self::Lay => "lay",
            Self::Move => "mv",
            Self::Stand => "std",
            Self::Swim => "swim",
            Self::Float => "float",
            Self::Sit => "sit",
            Self::SnowWarRun => "swrun",
            Self::SnowWarDieFront => "swdiefront",
            Self::SnowWarDieBack => "swdieback",
            Self::SnowWarPick => "swpick",
            Self::SnowWarThrow => "swthrow",
            Self::Other(key) => key
        }.to_string()
    }
}

impl Posture {
    fn identify(key: &str) -> Self {
        match key {
            "lay" => Self::Lay,
            "mv" => Self::Move,
            "std" => Self::Stand,
            "swim" => Self::Swim,
            "float" => Self::Float,
            "sit" => Self::Sit,
            "swrun" => Self::SnowWarRun,
            "swdiefront" => Self::SnowWarDieFront,
            "swdieback" => Self::SnowWarDieBack,
            "swpick" => Self::SnowWarPick,
            "swthrow" => Self::SnowWarThrow,
            _ => Self::Other(key.to_string())
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Gesture {
    Smile,
    Aggravated,
    Surprised,
    Sad,
    Joy,
    Crazy,
    Tongue,
    Blink,
    Miserable,
    Puzzled,
    Other(String)
}

impl ToString for Gesture {
    fn to_string(&self) -> String {
        match self {
            Self::Smile => "sml",
            Self::Aggravated => "agr",
            Self::Surprised => "srp",
            Self::Sad => "sad",
            Self::Joy => "joy",
            Self::Crazy => "crz",
            Self::Tongue => "tng",
            Self::Blink => "eyb",
            Self::Miserable => "mis",
            Self::Puzzled => "puz",
            Self::Other(key) => key
        }.to_string()
    }
}

impl Gesture {
    fn identify(key: &str) -> Self {
        match key {
            "sml" => Self::Smile,
            "agr" => Self::Aggravated,
            "srp" => Self::Surprised,
            "sad" => Self::Sad,
            "joy" => Self::Joy,
            "crz" => Self::Crazy,
            "tng" => Self::Tongue,
            "eyb" => Self::Blink,
            "mis" => Self::Miserable,
            "puz" => Self::Puzzled,
            _ => Self::Other(key.to_string())
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum WiredUserMoveType {
    Move,
    Slide
}

impl Default for WiredUserMoveType {
    fn default() -> Self {
        Self::Move
    }
}

impl PacketVariable for WiredUserMoveType {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let (key, size) = i32::from_packet(bytes);

        if key == 0 {
            (Self::Move, size)
        } else {
            (Self::Slide, size)
        }
    }

    fn to_packet(&self) -> Vec<u8> {
        match self {
            WiredUserMoveType::Move => 0i32,
            WiredUserMoveType::Slide => 1i32
        }.to_packet()
    }
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ActionDefinition {
    pub stuff_type_selection_enabled: bool,
    pub furni_limit: i32,
    pub stuff_ids: Vec<LegacyId>,
    pub stuff_type_id: i32,
    pub id: LegacyId,
    pub string_param: String,
    pub int_params: Vec<i32>,
    pub stuff_type_selection_code: i32,
    pub code: i32,
    pub delay_in_pulses: i32,
    pub conflicting_triggers: Vec<i32>
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct AddonDefinition {
    pub stuff_type_selection_enabled: bool,
    pub furni_limit: i32,
    pub stuff_ids: Vec<LegacyId>,
    pub stuff_type_id: i32,
    pub id: LegacyId,
    pub string_param: String,
    pub int_params: Vec<i32>,
    pub stuff_type_selection_code: i32,
    pub code: i32
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ConditionDefinition {
    pub stuff_type_selection_enabled: bool,
    pub furni_limit: i32,
    pub stuff_ids: Vec<LegacyId>,
    pub stuff_type_id: i32,
    pub id: LegacyId,
    pub string_param: String,
    pub int_params: Vec<i32>,
    pub stuff_type_selection_code: i32,
    pub code: i32
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct TriggerDefinition {
    pub stuff_type_selection_enabled: bool,
    pub furni_limit: i32,
    pub stuff_ids: Vec<LegacyId>,
    pub stuff_type_id: i32,
    pub id: LegacyId,
    pub string_param: String,
    pub int_params: Vec<i32>,
    pub stuff_type_selection_code: i32,
    pub code: i32,
    pub conflicting_actions: Vec<i32>
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct HallOfFameEntryData {
    pub user_id: LegacyId,
    pub user_name: String,
    pub figure: String,
    pub rank: i32,
    pub current_score: i32
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CommunityGoalData {
    pub has_goal_expired: bool,
    pub personal_contribution_score: i32,
    pub personal_contribution_rank: i32,
    pub community_total_score: i32,
    pub community_highest_achieved_level: i32,
    pub score_remaining_until_next_level: i32,
    pub percent_completion_towards_next_level: i32,
    pub goal_code: String,
    pub time_remaining_in_seconds: i32,
    pub reward_user_limits: Vec<i32>
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct QuestMessageData {
    pub campaign_code: String,
    pub completed_quests_in_campaign: i32,
    pub quest_count_in_campaign: i32,
    pub activity_point_type: i32,
    pub id: LegacyId,
    pub accepted: bool,
    pub quest_type: String,
    pub image_version: String,
    pub reward_currency_amount: i32,
    pub localization_code: String,
    pub completed_steps: i32,
    pub total_steps: i32,
    pub sort_order: i32,
    pub catalog_page_name: String,
    pub chain_code: String,
    pub easy: bool
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct TalentTrackRewardPerk {
    pub perk_id: String
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct TalentTrackRewardProduct {
    pub product_code: String,
    pub vip_days: i32
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct TalentTrackLevelData {
    pub level: i32,
    pub state: i32,
    pub tasks: Vec<TalentTrackTask>,
    pub reward_perks: Vec<TalentTrackRewardPerk>,
    pub reward_products: Vec<TalentTrackRewardProduct>
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct TalentTrackTask {
    pub achievement_id: LegacyId,
    pub required_level: i32,
    pub badge_code: String,
    pub state: i32,
    pub current_score: i32,
    pub total_score: i32
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CallForHelpPendingCall {
    pub call_id: String,
    pub time_stamp: String,
    pub message: String
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PendingGuideTicket {
    pub ticket_type: i32,
    pub seconds_ago: i32,
    pub is_guide: bool,
    pub other_party_name: String,
    pub other_party_figure: String,
    pub description: String,
    pub room_name: String
}

impl PacketVariable for PendingGuideTicket {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        let (ticket_type, seconds_ago, is_guide) = packet.read();

        (Self {
            ticket_type, seconds_ago, is_guide,
            other_party_name: if ticket_type < 3 || (ticket_type == 3 && !is_guide) { packet.read() } else { "".to_string() },
            other_party_figure: if ticket_type < 3 || (ticket_type == 3 && !is_guide) { packet.read() } else { "".to_string() },
            description: if ticket_type == 1 { packet.read() } else { "".to_string() },
            room_name: if ticket_type == 3 && !is_guide { packet.read() } else { "".to_string() },
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        let mut packet = HPacket::from_header_id(0);

        packet.append((
            self.ticket_type, self.seconds_ago, self.is_guide
        ));
        if self.ticket_type < 3 || (self.ticket_type == 3 && !self.is_guide) {
            packet.append((
                self.other_party_name.clone(), self.other_party_figure.clone()
            ));
        }
        if self.ticket_type == 1 {
            packet.append(self.description.clone());
        }
        if self.ticket_type == 3 && !self.is_guide {
            packet.append(self.room_name.clone());
        }

        packet.get_bytes()[6..].to_vec()
    }
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct LeaderBoardEntry {
    pub user_id: LegacyId,
    pub score: i32,
    pub rank: i32,
    pub name: String,
    pub figure: String,
    pub gender: String
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GameLobbyData {
    pub game_id: LegacyId,
    pub level_name: String,
    pub game_type: i32,
    pub field_type: i32,
    pub number_of_teams: i32,
    pub maximum_players: i32,
    pub owning_player_name: String,
    pub level_entry_id: LegacyId,
    pub players: Vec<GameLobbyPlayerData>
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GameLobbyPlayerData {
    pub user_id: LegacyId,
    pub name: String,
    pub figure: String,
    pub gender: String,
    pub team_id: i32,
    pub skill_level: i32,
    pub total_score: i32,
    pub score_to_next_level: i32
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct AcceptFriendFailureData {
    pub sender_id: LegacyId,
    pub error_code: i32
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct FindFriendsProcessResult {
    pub success: bool
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct FollowFriendFailed {
    pub error_code: i32
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct FriendData {
    pub id: LegacyId,
    pub name: String,
    pub gender: i32,
    pub online: bool,
    pub following_allowed: bool,
    pub figure: String,
    pub category_id: i32,
    pub motto: String,
    pub real_name: String,
    pub facebook_id: String,
    pub persisted_message_user: bool,
    pub vip_member: bool,
    pub pocket_habbo_user: bool,
    pub relationship_status: i16
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct FriendCategoryData {
    pub id: i32,
    pub name: String
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct FriendRequestData {
    pub requester_user_id: LegacyId,
    pub requester_name: String,
    pub figure_string: String
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct HabboSearchResultData {
    pub avatar_id: LegacyId,
    pub avatar_name: String,
    pub avatar_motto: String,
    pub is_avatar_online: bool,
    pub can_follow: bool,
    pub last_online_date: String,
    pub avatar_gender: i32,
    pub avatar_figure: String,
    pub real_name: String
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct BreedingPetInfo {
    pub web_id: LegacyId,
    pub name: String,
    pub level: i32,
    pub figure: String,
    pub owner: String
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RarityCategoryData {
    pub chance: i32,
    pub breeds: Vec<i32>
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct OutfitData {
    pub slot_id: i32,
    pub figure_string: String,
    pub gender: String
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct BotData {
    pub id: LegacyId,
    pub name: String,
    pub motto: String,
    pub gender: String,
    pub figure: String
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct AchievementData {
    pub achievement_id: LegacyId,
    pub level: i32,
    pub badge_id: String,
    pub score_at_start_of_level: i32,
    pub score_limit: i32,
    pub level_reward_points: i32,
    pub level_reward_point_type: i32,
    pub current_points: i32,
    pub final_level: bool,
    pub category: String,
    pub sub_category: String,
    pub level_count: i32,
    pub display_method: i32,
    pub state: i32
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct HotLookInfo {
    pub gender: String,
    pub figure_string: String
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PlayListEntry {
    pub id: LegacyId,
    pub length: i32,
    pub song_name: String,
    pub creator: String
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct SongInfoEntry {
    pub id: LegacyId,
    pub _unused: String,
    pub song_name: String,
    pub data: String,
    pub length: i32,
    pub creator: String
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct IncomeReward {
    pub reward_category: i8,
    pub reward_type: i8,
    pub amount: i32,
    pub product_code: String
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct LiftedRoomData {
    pub flat_id: LegacyId,
    pub area_id: LegacyId,
    pub image: String,
    pub caption: String
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct TopLevelContext {
    pub search_code: String,
    pub quick_links: Vec<SavedSearch>
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct SavedSearch {
    pub id: LegacyId,
    pub search_code: String,
    pub filter: String,
    pub localization: String
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct SearchResultSet {
    pub search_code_original: String,
    pub filtering_data: String,
    pub blocks: Vec<SearchResultList>
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct SearchResultList {
    pub search_code: String,
    pub text: String,
    pub action_allowed: i32,
    pub force_closed: bool,
    pub view_mode: i32,
    pub guest_rooms: Vec<GuestRoomData>
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2PlayerData {
    pub reference_id: LegacyId,
    pub user_name: String,
    pub figure_string: String,
    pub gender: String,
    pub team_id: i32
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GameLevelData {
    pub width: i32,
    pub height: i32,
    pub height_map: String,
    pub fuse_objects: Vec<FuseObjectData>
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct FuseObjectData {
    pub name: String,
    pub id: LegacyId,
    pub x: i32,
    pub y: i32,
    pub x_dimension: i32,
    pub y_dimension: i32,
    pub height: i32,
    pub direction: i32,
    pub altitude: i32,
    pub can_stand_on: i32,
    pub stuff_data: StuffData
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2GameResult {
    pub is_death_match: bool,
    pub result_type: i32,
    pub winner_id: LegacyId
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2TeamScoreData {
    pub team_reference: i32,
    pub score: i32,
    pub players: Vec<Game2TeamPlayerData>
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2TeamPlayerData {
    pub user_name: String,
    pub user_id: LegacyId,
    pub figure: String,
    pub gender: String,
    pub score: i32,
    pub player_stats: Game2PlayerStatsData
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2PlayerStatsData {
    pub score: i32,
    pub kills: i32,
    pub deaths: i32,
    pub snowball_hits: i32,
    pub snowball_hits_taken: i32,
    pub snowballs_thrown: i32,
    pub snowballs_created: i32,
    pub snowballs_from_machine: i32,
    pub friendly_hits: i32,
    pub friendly_kills: i32
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2SnowWarGameStats {
    pub player_with_most_kills: i32,
    pub player_with_most_hits: i32
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GameObjectsData {
    pub game_objects: Vec<SnowWarGameObjectData>
}

#[derive(Clone, Debug, PartialEq)]
pub enum SnowWarGameObjectData {
    SnowballGameObjectData {
        id: i32,
        location_x_3d: i32,
        location_y_3d: i32,
        location_z_3d: i32,
        movement_direction_360: i32,
        trajectory: i32,
        time_to_live: i32,
        throwing_human: i32,
        parabola_offset: i32,
        planar_velocity: i32
    },
    TreeGameObjectData {
        id: i32,
        location_x_3d: i32,
        location_y_3d: i32,
        direction: i32,
        height: i32,
        fuse_object_id: i32,
        max_hits: i32,
        hits: i32
    },
    SnowballPileGameObjectData {
        id: i32,
        location_x_3d: i32,
        location_y_3d: i32,
        max_snow_balls: i32,
        snowball_count: i32,
        fuse_object_id: i32
    },
    SnowballMachineGameObjectData {
        id: i32,
        location_x_3d: i32,
        location_y_3d: i32,
        direction: i32,
        max_snow_balls: i32,
        snowball_count: i32,
        fuse_object_id: i32
    },
    HumanGameObjectData {
        id: i32,
        current_location_x: i32,
        current_location_y: i32,
        current_tile_x: i32,
        current_tile_y: i32,
        body_direction: i32,
        hit_points: i32,
        snow_ball_count: i32,
        is_bot: i32,
        activity_timer: i32,
        activity_state: i32,
        next_tile_x: i32,
        next_tile_y: i32,
        move_target_x: i32,
        move_target_y: i32,
        score: i32,
        team: i32,
        user_id: i32,
        name: String,
        mission: String,
        figure: String,
        sex: String
    }
}

impl PacketVariable for SnowWarGameObjectData {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        (match packet.read::<i32>() {
            1 => Self::SnowballGameObjectData {
                id: packet.read(),
                location_x_3d: packet.read(),
                location_y_3d: packet.read(),
                location_z_3d: packet.read(),
                movement_direction_360: packet.read(),
                trajectory: packet.read(),
                time_to_live: packet.read(),
                throwing_human: packet.read(),
                parabola_offset: packet.read(),
                planar_velocity: packet.read()
            },
            2 => Self::TreeGameObjectData {
                id: packet.read(),
                location_x_3d: packet.read(),
                location_y_3d: packet.read(),
                direction: packet.read(),
                height: packet.read(),
                fuse_object_id: packet.read(),
                max_hits: packet.read(),
                hits: packet.read()
            },
            3 => Self::SnowballPileGameObjectData {
                id: packet.read(),
                location_x_3d: packet.read(),
                location_y_3d: packet.read(),
                max_snow_balls: packet.read(),
                snowball_count: packet.read(),
                fuse_object_id: packet.read()
            },
            4 => Self::SnowballMachineGameObjectData {
                id: packet.read(),
                location_x_3d: packet.read(),
                location_y_3d: packet.read(),
                direction: packet.read(),
                max_snow_balls: packet.read(),
                snowball_count: packet.read(),
                fuse_object_id: packet.read()
            },
            5 => Self::HumanGameObjectData {
                id: packet.read(),
                current_location_x: packet.read(),
                current_location_y: packet.read(),
                current_tile_x: packet.read(),
                current_tile_y: packet.read(),
                body_direction: packet.read(),
                hit_points: packet.read(),
                snow_ball_count: packet.read(),
                is_bot: packet.read(),
                activity_timer: packet.read(),
                activity_state: packet.read(),
                next_tile_x: packet.read(),
                next_tile_y: packet.read(),
                move_target_x: packet.read(),
                move_target_y: packet.read(),
                score: packet.read(),
                team: packet.read(),
                user_id: packet.read(),
                name: packet.read(),
                mission: packet.read(),
                figure: packet.read(),
                sex: packet.read()
            },
            _ => panic!("SnowWarGameObjectData: Unknown type")
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        match self {
            SnowWarGameObjectData::SnowballGameObjectData {
                id, location_x_3d, location_y_3d, location_z_3d,
                movement_direction_360, trajectory, time_to_live,
                throwing_human, parabola_offset, planar_velocity
            } => (
                *id, *location_x_3d, *location_y_3d, *location_z_3d,
                *movement_direction_360, *trajectory, *time_to_live,
                *throwing_human, *parabola_offset, *planar_velocity
            ).to_packet(),
            SnowWarGameObjectData::TreeGameObjectData {
                id, location_x_3d, location_y_3d, direction,
                height, fuse_object_id, max_hits, hits
            } => (
                *id, *location_x_3d, *location_y_3d, *direction,
                *height, *fuse_object_id, *max_hits, *hits
            ).to_packet(),
            SnowWarGameObjectData::SnowballPileGameObjectData {
                id, location_x_3d, location_y_3d,
                max_snow_balls, snowball_count, fuse_object_id
            } => (
                *id, *location_x_3d, *location_y_3d,
                *max_snow_balls, *snowball_count, *fuse_object_id
            ).to_packet(),
            SnowWarGameObjectData::SnowballMachineGameObjectData {
                id, location_x_3d, location_y_3d,  direction,
                max_snow_balls, snowball_count, fuse_object_id
            } => (
                *id, *location_x_3d, *location_y_3d, *direction,
                *max_snow_balls, *snowball_count, *fuse_object_id
            ).to_packet(),
            SnowWarGameObjectData::HumanGameObjectData {
                id, current_location_x, current_location_y,
                current_tile_x, current_tile_y, body_direction,
                hit_points, snow_ball_count, is_bot, activity_timer,
                activity_state, next_tile_x, next_tile_y,
                move_target_x, move_target_y, score, team,
                user_id, name, mission, figure, sex
            } => (
                *id, *current_location_x, *current_location_y,
                *current_tile_x, *current_tile_y, *body_direction,
                *hit_points, *snow_ball_count, *is_bot, *activity_timer,
                *activity_state, *next_tile_x, *next_tile_y,
                *move_target_x, *move_target_y, *score, *team,
                *user_id, name.clone(), mission.clone(), figure.clone(), sex.clone()
            ).to_packet()
        }
    }
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct FurniData {
    pub item_id: LegacyId,
    pub item_type: String,
    pub room_item_id: LegacyId,
    pub item_type_id: i32,
    pub category: i32,
    pub stuff_data: StuffData,
    pub is_recyclable: bool,
    pub is_tradeable: bool,
    pub is_groupable: bool,
    pub is_sellable: bool,
    pub seconds_to_expiration: i32,
    pub has_rent_period_started: bool,
    pub flat_id: LegacyId,
    pub slot_id: String,
    pub extra: i32
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct AvatarEffectData {
    pub effect_type: i32,
    pub sub_type: i32,
    pub duration: i32,
    pub inactive_effects_in_inventory: i32,
    pub seconds_left_if_active: i32,
    pub is_permanent: bool
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct NftWardrobeItem {
    pub token_id: String,
    pub figure_string: String,
    pub gender: String
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct AchievementLevelUpData {
    pub achievement_type: i32,
    pub level: i32,
    pub badge_id: i32,
    pub badge_code: String,
    pub points: i32,
    pub level_reward_points: i32,
    pub level_reward_point_type: i32,
    pub bonus_points: i32,
    pub achievement_id: i32,
    pub removed_badge_code: String,
    pub category: String,
    pub show_dialog_to_user: bool
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ItemDataStructure {
    pub item_id: LegacyId,
    pub item_type: String,
    pub room_item_id: LegacyId,
    pub item_type_id: i32,
    pub category: i32,
    pub stuff_data: StuffData,
    pub creation_day: i32,
    pub creation_month: i32,
    pub creation_year: i32,
    pub extra: i32
}

impl PacketVariable for ItemDataStructure {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        let item_id = packet.read();
        let item_type: String = packet.read();
        let (room_item_id, item_type_id, category, stuff_data,
            creation_day, creation_month, creation_year) = packet.read();
        let extra = if item_type.clone().to_uppercase() == "S" { packet.read() } else { -1 };

        (Self {
            item_id, item_type, room_item_id, item_type_id, category,
            stuff_data, creation_day, creation_month, creation_year, extra
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        if self.item_type.clone().to_uppercase() == "S" {
            (
                self.item_id, self.item_type.clone(), self.room_item_id, self.item_type_id,
                self.category, self.stuff_data.clone(), self.creation_day,
                self.creation_month, self.creation_year, self.extra
            ).to_packet()
        } else {
            (
                self.item_id, self.item_type.clone(), self.room_item_id, self.item_type_id,
                self.category, self.stuff_data.clone(), self.creation_day,
                self.creation_month, self.creation_year
            ).to_packet()
        }
    }
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Perk {
    pub code: String,
    pub error_message: String,
    pub is_allowed: bool
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct BotSkillData {
    pub id: LegacyId,
    pub data: String
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PromoArticleData {
    pub id: LegacyId,
    pub title: String,
    pub body_text: String,
    pub button_text: String,
    pub link_type: i32,
    pub link_content: String,
    pub image_url: String
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CfhSanctionTypeData {
    pub name: String,
    pub sanction_length_in_hours: i32,
    pub _unused: i32,
    pub avatar_only: bool,
    pub trade_lock_info: Option<String>,
    pub machine_ban_info: Option<String>
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CallForHelpCategoryData {
    pub name: String,
    pub topics: Vec<CallForHelpTopicData>
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CallForHelpTopicData {
    pub name: String,
    pub id: LegacyId,
    pub consequence: String
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct BannedUserData {
    pub user_id: LegacyId,
    pub user_name: String
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct FlatControllerData {
    pub user_id: LegacyId,
    pub user_name: String
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RoomChatSettings {
    pub mode: i32,
    pub bubble_width: i32,
    pub scroll_speed: i32,
    pub full_hear_range: i32,
    pub flood_sensitivity: i32
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct FullGameStatusData {
    pub _unused: i32,
    pub remaining_time_seconds: i32,
    pub duration_in_seconds: i32,
    pub game_objects: GameObjectsData,
    pub _unused2: i32,
    pub number_of_teams: i32,
    pub game_status: GameStatusData
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GameStatusData {
    pub turn: i32,
    pub checksum: i32,
    pub events: Vec<Vec<SnowWarGameEventData>>
}

#[derive(Clone, Debug, PartialEq)]
pub enum SnowWarGameEventData {
    HumanLeftGameEventData {
        human_game_object_id: i32
    },
    NewMoveTargetEventData {
        human_game_object_id: i32,
        x: i32,
        y: i32
    },
    HumanThrowsSnowballAtHumanEventData {
        human_game_object_id: i32,
        target_human_game_object_id: i32,
        trajectory: i32
    },
    HumanThrowsSnowballAtPositionEventData {
        human_game_object_id: i32,
        target_x: i32,
        target_y: i32,
        trajectory: i32
    },
    HumanStartsToMakeASnowballEventData {
        human_game_object_id: i32
    },
    CreateSnowballEventData {
        snow_ball_game_object_id: i32,
        human_game_object_id: i32,
        target_x: i32,
        target_y: i32,
        trajectory: i32
    },
    MachineCreatesSnowballEventData {
        snow_ball_machine_reference: i32
    },
    HumanGetsSnowballsFromMachineEventData {
        human_game_object_id: i32,
        snow_ball_machine_reference: i32
    }
}

impl PacketVariable for SnowWarGameEventData {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        (match packet.read::<i32>() {
            1 => SnowWarGameEventData::HumanLeftGameEventData {
                human_game_object_id: packet.read()
            },
            2 => SnowWarGameEventData::NewMoveTargetEventData {
                human_game_object_id: packet.read(),
                x: packet.read(),
                y: packet.read()
            },
            3 => SnowWarGameEventData::HumanThrowsSnowballAtHumanEventData {
                human_game_object_id: packet.read(),
                target_human_game_object_id: packet.read(),
                trajectory: packet.read()
            },
            4 => SnowWarGameEventData::HumanThrowsSnowballAtPositionEventData {
                human_game_object_id: packet.read(),
                target_x: packet.read(),
                target_y: packet.read(),
                trajectory: packet.read()
            },
            7 => SnowWarGameEventData::HumanStartsToMakeASnowballEventData {
                human_game_object_id: packet.read()
            },
            8 => SnowWarGameEventData::CreateSnowballEventData {
                snow_ball_game_object_id: packet.read(),
                human_game_object_id: packet.read(),
                target_x: packet.read(),
                target_y: packet.read(),
                trajectory: packet.read()
            },
            11 => SnowWarGameEventData::MachineCreatesSnowballEventData {
                snow_ball_machine_reference: packet.read()
            },
            12 => SnowWarGameEventData::HumanGetsSnowballsFromMachineEventData {
                human_game_object_id: packet.read(),
                snow_ball_machine_reference: packet.read()
            },
            _ => panic!("SnowWarGameEventData: Unknown type")
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        match self {
            SnowWarGameEventData::HumanLeftGameEventData {
                human_game_object_id
            } => (
                1i32, *human_game_object_id
            ).to_packet(),
            SnowWarGameEventData::NewMoveTargetEventData {
                human_game_object_id, x, y
            } => (
                2i32, *human_game_object_id, *x, *y
            ).to_packet(),
            SnowWarGameEventData::HumanThrowsSnowballAtHumanEventData {
                human_game_object_id, target_human_game_object_id, trajectory
            } => (
                3i32, *human_game_object_id, *target_human_game_object_id, *trajectory
            ).to_packet(),
            SnowWarGameEventData::HumanThrowsSnowballAtPositionEventData {
                human_game_object_id, target_x, target_y, trajectory
            } => (
                4i32, *human_game_object_id, *target_x, *target_y, *trajectory
            ).to_packet(),
            SnowWarGameEventData::HumanStartsToMakeASnowballEventData {
                human_game_object_id
            } => (
                7i32, *human_game_object_id
            ).to_packet(),
            SnowWarGameEventData::CreateSnowballEventData {
                snow_ball_game_object_id, human_game_object_id,
                target_x, target_y, trajectory
            } => (
                8i32, *snow_ball_game_object_id, *human_game_object_id,
                *target_x, *target_y, *trajectory
            ).to_packet(),
            SnowWarGameEventData::MachineCreatesSnowballEventData {
                snow_ball_machine_reference
            } => (
                11i32, *snow_ball_machine_reference
            ).to_packet(),
            SnowWarGameEventData::HumanGetsSnowballsFromMachineEventData {
                human_game_object_id, snow_ball_machine_reference
            } => (
                12i32, *human_game_object_id, *snow_ball_machine_reference
            ).to_packet()
        }
    }
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PetBreedingResultData {
    pub stuff_id: LegacyId,
    pub class_id: i32,
    pub product_code: String,
    pub user_id: LegacyId,
    pub user_name: String,
    pub rarity_level: i32,
    pub has_mutation: bool
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct BadgeAndPointLimit {
    pub badge_id: i32,
    pub limit: i32
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct NewUserExperienceGiftOptions {
    pub day_index: i32,
    pub step_index: i32,
    pub options: Vec<NewUserExperienceGift>
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct NewUserExperienceGift {
    pub thumbnail_url: String,
    pub product_offers: Vec<NewUserExperienceGiftProduct>
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct NewUserExperienceGiftProduct {
    pub product_code: String,
    pub localization_key: String
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct MarketplaceItemStatsData {
    pub day_offset: i32,
    pub average_price: i32,
    pub sold_amount: i32
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MarketPlaceOffer {
    pub offer_id: LegacyId,
    pub status: i32,
    pub furni_type: i32,
    pub furni_id: LegacyId,
    pub stuff_data: StuffData,
    pub extra_data: String,
    pub price: i32,
    pub time_left_minutes: i32,
    pub average_price: i32,
    pub offer_count: i32
}

impl PacketVariable for MarketPlaceOffer {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        let (offer_id, status) = packet.read();
        let furni_type: i32 = packet.read();
        let mut furni_id = LegacyId(-1);
        let mut stuff_data = StuffData::default();
        let mut extra_data = "".to_string();
        if furni_type == 1 {
            furni_id = packet.read();
            stuff_data = packet.read();
        } else if furni_type == 2 {
            furni_id = packet.read();
            extra_data = packet.read();
        } else if furni_type == 3 {
            furni_id = packet.read();
            stuff_data = StuffData::EmptyStuffData {
                unique_serial_data: packet.read()
            }
        }
        let (price, time_left_minutes, average_price) = packet.read();

        (Self {
            offer_id,
            furni_id,
            furni_type,
            extra_data,
            stuff_data,
            price,
            status,
            time_left_minutes,
            average_price,
            offer_count: -1
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        let mut packet = HPacket::from_header_id(0);

        packet.append((
            self.offer_id, self.status, self.furni_type
        ));
        if self.furni_type == 1 {
            packet.append((self.furni_id, self.stuff_data.clone()));
        } else if self.furni_type == 2 {
            packet.append((self.furni_id, self.extra_data.clone()));
        } else if self.furni_type == 3 {
            packet.append((self.furni_id, self.stuff_data.get_unique_serial_data()));
        }
        packet.append((
            self.price, self.time_left_minutes, self.average_price
        ));

        packet.get_bytes()[6..].to_vec()
    }
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ClassifiedUser {
    pub id: LegacyId,
    pub name: String,
    pub user_type: String
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct NewUserExperienceGetGiftsSelection {
    pub day_index: i32,
    pub step_index: i32,
    pub gift_index: i32
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct UpdateForumReadMarkerData {
    pub group_id: LegacyId,
    pub message_count: i32,
    pub has_unread_messages: bool
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CallForHelpFromIMMessage {
    pub user_id: LegacyId,
    pub message: String
}





