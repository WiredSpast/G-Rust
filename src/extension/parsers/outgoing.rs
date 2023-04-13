use std::collections::HashMap;
use miniz_oxide::deflate::compress_to_vec_zlib;
use miniz_oxide::inflate::decompress_to_vec_zlib;
use crate::protocol::hdirection::HDirection;
use crate::protocol::hpacket::HPacket;
use crate::protocol::vars::legacy::{LegacyId, LegacyLength};
use crate::protocol::vars::packetvariable::PacketVariable;
use super::baseparser::BaseParser;
use super::subparsers::*;

// WIN63-202303241432-611275200

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct Game2GetFriendsLeaderboard {
    pub selected_game: i32,
    pub rank: i32,
    pub scroll_dir: i32,
    pub view_size: i32,
    pub window_size: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct Game2GetTotalGroupLeaderboard {
    pub selected_game: i32,
    pub rank: i32,
    pub scroll_dir: i32,
    pub view_size: i32,
    pub window_size: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct Game2GetTotalLeaderboard {
    pub selected_game: i32,
    pub rank: i32,
    pub scroll_dir: i32,
    pub view_size: i32,
    pub window_size: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct Game2GetWeeklyFriendsLeaderboard {
    pub selected_game: i32,
    pub offset: i32,
    pub rank: i32,
    pub scroll_dir: i32,
    pub view_size: i32,
    pub window_size: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct Game2GetWeeklyGroupLeaderboard {
    pub selected_game: i32,
    pub offset: i32,
    pub rank: i32,
    pub scroll_dir: i32,
    pub view_size: i32,
    pub window_size: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct Game2GetWeeklyLeaderboard {
    pub selected_game: i32,
    pub offset: i32,
    pub rank: i32,
    pub scroll_dir: i32,
    pub view_size: i32,
    pub window_size: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct UpdateFigureData {
    pub figure_string: String,
    pub gender: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct OpenCampaignCalendarDoorAsStaff {
    pub campaign_name: String,
    pub day: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct OpenCampaignCalendarDoor {
    pub campaign_name: String,
    pub day: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetOccupiedTiles {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetRoomEntryTile {}

#[derive(BaseParser, Clone, Debug, Default, PartialEq)]
#[to(direction = 1)]
pub struct UpdateFloorProperties {
    pub data: String,
    pub entry_point_x: i32,
    pub entry_point_y: i32,
    pub entry_point_dir: i32,
    pub wall_thickness: i32,
    pub floor_thickness: i32,
    pub fixed_walls_height: i32
}

impl PacketVariable for UpdateFloorProperties {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        (Self {
            data: packet.read(),
            entry_point_x: packet.read::<Option<i32>>().unwrap_or(-1),
            entry_point_y: packet.read::<Option<i32>>().unwrap_or(-1),
            entry_point_dir: packet.read::<Option<i32>>().unwrap_or(-1),
            wall_thickness: packet.read::<Option<i32>>().unwrap_or(-1),
            floor_thickness: packet.read::<Option<i32>>().unwrap_or(-1),
            fixed_walls_height: packet.read::<Option<i32>>().unwrap_or(-1)
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        if self.entry_point_x == -1 && self.entry_point_y == -1 && self.entry_point_dir == -1
            && self.wall_thickness == -1 && self.floor_thickness == -1 {
            self.data.to_packet()
        } else if self.fixed_walls_height == -1 {
            (
                self.data.clone(), self.entry_point_x, self.entry_point_y, self.entry_point_dir,
                self.wall_thickness, self.floor_thickness
            ).to_packet()
        } else {
            (
                self.data.clone(), self.entry_point_x, self.entry_point_y, self.entry_point_dir,
                self.wall_thickness, self.floor_thickness, self.fixed_walls_height
            ).to_packet()
        }
    }
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct AddAdminRightsToMember {
    pub group_id: LegacyId,
    pub user_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ApproveAllMembershipRequests {
    pub group_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ApproveMembershipRequest {
    pub group_id: LegacyId,
    pub user_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ApproveName {
    pub name: String,
    pub approved: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ChangeEmail {
    pub email: String
}

#[derive(BaseParser, Clone, Debug, Default, PartialEq)]
#[to(direction = 1)]
pub struct CreateGuild {
    pub name: String,
    pub description: String,
    pub room_id: LegacyId,
    pub primary_color_id: i32,
    pub secondary_color_id: i32,
    pub badge_settings: Vec<GuildBadgeSettings>
}

impl PacketVariable for CreateGuild {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        let (name, description, room_id, primary_color_id, secondary_color_id) = packet.read();
        let size: LegacyLength = packet.read();
        let mut badge_settings: Vec<GuildBadgeSettings> = Vec::new();
        for _ in 0..*size/3 {
            badge_settings.push(packet.read());
        }

        (Self {
            name, description, room_id, primary_color_id, secondary_color_id, badge_settings
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        let mut packet = HPacket::from_header_id(0);

        packet.append((
            self.name.clone(), self.description.clone(), self.room_id,
            self.primary_color_id, self.secondary_color_id,
            LegacyLength(self.badge_settings.len() as i32 * 3)
        ));
        for setting in self.badge_settings.clone() {
            packet.append(setting.clone());
        }

        packet.get_bytes()[6..].to_vec()
    }
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct DeactivateGuild {
    pub id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct DeselectFavouriteHabboGroup {
    pub id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetEmailStatus {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetExtendedProfileByName {
    pub name: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetExtendedProfile {
    pub user_id: LegacyId,
    pub open: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetGuildCreationInfo {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetGuildEditInfo {
    pub group_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetGuildEditorData {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetGuildMemberships {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetGuildMembers {
    pub group_id: LegacyId,
    pub page_index: i32,
    pub search_text: String,
    pub search_type: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetHabboGroupBadges {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetHabboGroupDetails {
    pub group_id: LegacyId,
    pub open: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetIgnoredUsers {
    pub user_name: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetMemberGuildItemCount {
    pub group_id: LegacyId,
    pub avatar_id: LegacyId,
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetMOTD {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetRelationshipStatusInfo {
    pub user_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetSelectedBadges {
    pub user_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GiveStarGemToUser {
    pub user_id: LegacyId,
    pub amount: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct IgnoreUserId {
    pub user_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct IgnoreUser {
    pub name: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct JoinHabboGroup {
    pub group_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct KickMember {
    pub guild_id: LegacyId,
    pub user_id: LegacyId,
    pub user_blocked: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RejectMembershipRequest {
    pub guild_id: LegacyId,
    pub user_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RemoveAdminRightsFromMember {
    pub guild_id: LegacyId,
    pub user_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ScrGetKickbackInfo {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ScrGetUserInfo {
    pub info_type: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct SelectFavouriteHabboGroup {
    pub group_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct UnblockGroupMember {
    pub group_id: LegacyId,
    pub user_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct UnignoreUser {
    pub name: String
}

#[derive(BaseParser, Clone, Debug, Default, PartialEq)]
#[to(direction = 1)]
pub struct UpdateGuildBadge {
    pub group_id: LegacyId,
    pub badge_settings: Vec<GuildBadgeSettings>
}

impl PacketVariable for UpdateGuildBadge {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        let group_id = packet.read();
        let size: LegacyLength = packet.read();
        let mut badge_settings: Vec<GuildBadgeSettings> = Vec::new();
        for _ in 0..*size/3 {
            badge_settings.push(packet.read());
        }

        (Self {
            group_id, badge_settings
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        let mut packet = HPacket::from_header_id(0);

        packet.append((self.group_id, LegacyLength(self.badge_settings.len() as i32 * 3)));
        for setting in self.badge_settings.clone() {
            packet.append(setting.clone());
        }

        packet.get_bytes()[6..].to_vec()
    }
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct UpdateGuildColors {
    pub group_id: LegacyId,
    pub primary_color_id: i32,
    pub secondary_color_id: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct UpdateGuildIdentity {
    pub group_id: LegacyId,
    pub name: String,
    pub description: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct UpdateGuildSettings {
    pub group_id: LegacyId,
    pub guild_type: i32,
    pub rights_level: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct SetChatPreferences {
    pub force_old_chat: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct SetChatStylePreference {
    pub preferred_chat_style: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct SetIgnoreRoomInvites {
    pub ignored: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct SetNewNavigatorWindowPreferences {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub left_pane_hidden: bool,
    pub _unknown: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct SetRoomCameraPreferences {
    pub follow_disabled: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct SetSoundSettings {
    pub trax_volume: i32,
    pub furni_volume: i32,
    pub generic_volume: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct SetUIFlags {
    pub flags: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ApplySnapshot {
    pub id: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct Open {
    pub stuff_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct UpdateAction {
    pub id: LegacyId,
    pub int_params: Vec<i32>,
    pub string_param: String,
    pub stuff_ids: Vec<LegacyId>,
    pub action_delay: i32,
    pub stuff_selection_type: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct UpdateAddon {
    pub id: LegacyId,
    pub int_params: Vec<i32>,
    pub string_param: String,
    pub stuff_ids: Vec<LegacyId>,
    pub stuff_selection_type: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct UpdateCondition {
    pub id: LegacyId,
    pub int_params: Vec<i32>,
    pub string_param: String,
    pub stuff_ids: Vec<LegacyId>,
    pub stuff_selection_type: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct UpdateTrigger {
    pub id: LegacyId,
    pub int_params: Vec<i32>,
    pub string_param: String,
    pub stuff_ids: Vec<LegacyId>,
    pub stuff_selection_type: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct PollAnswer {
    pub id: i32,
    pub question_id: i32,
    pub answers: Vec<String>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct PollReject {
    pub id: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct PollStart {
    pub id: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct AvatarExpression {
    pub expression: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ChangeMotto {
    pub motto: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ChangePosture {
    pub posture: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct CustomizeAvatarWithFurni {
    pub id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct Dance {
    pub style: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct DropCarryItem {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct LookTo {
    pub x: i32,
    pub y: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct PassCarryItem {
    pub user_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct Sign {
    pub sign: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetTalentTrackLevel {
    pub talent_track_name: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetTalentTrack {
    pub talent_track_name: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GuideAdvertisementRead {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct DeleteRoom {
    pub room_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetBannedUsersFromRoom {
    pub room_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetCustomRoomFilter {
    pub room_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetFlatControllers {
    pub room_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetRoomSettings {
    pub room_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct SaveRoomSettings {
    pub room_id: LegacyId,
    pub name: String,
    pub description: String,
    pub door_mode: i32,
    pub password: String,
    pub maximum_visitors: i32,
    pub category_id: i32,
    pub tags: Vec<String>,
    pub trade_mode: i32,
    pub allow_pets: bool,
    pub allow_food_consume: bool,
    pub allow_walk_through: bool,
    pub hide_walls: bool,
    pub wall_thickness: i32,
    pub floor_thickness: i32,
    pub who_can_mute: i32,
    pub who_can_kick: i32,
    pub who_can_ban: i32,
    pub chat_mode: i32,
    pub chat_bubble_size: i32,
    pub chat_scroll_up_frequency: i32,
    pub chat_full_hear_range: i32,
    pub chat_flood_sensitivity: i32,
    pub allow_navigator_dyn_cats: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct UpdateRoomCategoryAndTradeSettings {
    pub room_id: LegacyId,
    pub category: i32,
    pub trade_mode: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct UpdateRoomFilter {
    pub room_id: LegacyId,
    pub is_added: bool,
    pub bad_word: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct PeerUsersClassification {
    pub data: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RoomUsersClassification {
    pub data: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ClientHello {
    pub version: String,
    pub client_type: String,
    pub operating_system: i32,
    pub _unknown: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct CompleteDiffieHandshake {
    pub public_key: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct Disconnect {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct InfoRetrieve {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct InitDiffieHandshake {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct Pong {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct SSOTicket {
    pub sso_ticket: String,
    pub time: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct UniqueID {
    pub machine_id: String,
    pub finger_print: String,
    pub flash_version: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct VersionCheck {
    pub _unknown: i32,
    pub flash_client: String,
    pub external_variables: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct AcceptFriend {
    pub request_ids: Vec<LegacyId>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct DeclineFriend {
    pub none_declined: bool,
    pub request_ids: Vec<LegacyId>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct FindNewFriends {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct FollowFriend {
    pub user_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct FriendListUpdate {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetFriendRequests {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct HabboSearch {
    pub search_query: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct MessengerInit {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RemoveFriend {
    pub user_ids: Vec<LegacyId>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RequestFriend {
    pub user_name: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct SendMsg {
    pub user_id: LegacyId,
    pub msg: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct SendRoomInvite {
    pub user_ids: Vec<LegacyId>,
    pub msg: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct SetRelationshipStatus {
    pub user_id: LegacyId,
    pub relationship_status: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct VisitUser {
    pub user_name: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct AddFavouriteRoom {
    pub room_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct CancelEvent {
    pub ad_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct CanCreateRoom {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct CompetitionRoomsSearch {
    pub goal_id: i32,
    pub page_index: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ConvertGlobalRoomId {
    pub room_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct CreateFlat {
    pub name: String,
    pub description: String,
    pub model: String,
    pub category: i32,
    pub max_visitors: i32,
    pub trade_mode: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct DeleteFavouriteRoom {
    pub room_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct EditEvent {
    pub ad_id: LegacyId,
    pub name: String,
    pub description: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ForwardToARandomPromotedRoom {
    pub room_category: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ForwardToSomeRoom {
    pub room_identifier: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetGuestRoom {
    pub room_id: LegacyId,
    pub entering_room: i32,
    pub go_to_room: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetOfficialRooms {
    pub ad_index: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetPopularRoomTags {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetUserEventCats {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetUserFlatCats {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GuildBaseSearch {
    pub ad_index: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct MyFavouriteRoomsSearch {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct MyFrequentRoomHistorySearch {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct MyFriendsRoomsSearch {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct MyGuildBasesSearch {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct MyRecommendedRooms {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct MyRoomHistorySearch {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct MyRoomRightsSearch {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct MyRoomsSearch {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct PopularRoomsSearch {
    pub search_query: String,
    pub ad_index: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RateFlat {
    pub rating: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RemoveOwnRoomRightsRoom {
    pub room_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RoomAdEventTabAdClicked {
    pub room_id: LegacyId,
    pub room_ad_name: String,
    pub room_ad_expires_in_min: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RoomAdEventTabViewed {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RoomAdSearch {
    pub ad_index: i32,
    pub search_type: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RoomsWhereMyFriendsAreSearch {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RoomsWithHighestScoreSearch {
    pub ad_index: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RoomTextSearch {
    pub search_query: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct SetRoomSessionTags {
    pub tag1: String,
    pub tag2: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ToggleStaffPick {
    pub room_id: LegacyId,
    pub is_staff_pick: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct UpdateHomeRoom {
    pub room_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct BuyMarketplaceOffer {
    pub offer_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct BuyMarketplaceTokens {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct CancelMarketplaceOffer {
    pub offer_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetMarketplaceCanMakeOffer {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetMarketplaceConfiguration {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetMarketplaceItemStats {
    pub placement_type: i32,
    pub item_type: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetMarketplaceOffers {
    pub min_price: i32,
    pub max_price: i32,
    pub search_string: String,
    pub sort_key: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetMarketplaceOwnOffers {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct MakeOffer {
    pub price: i32,
    pub placement_type: i32,
    pub room_item_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RedeemMarketplaceOfferCredits {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ForwardToACompetitionRoom {
    pub goal_code: String,
    pub user_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ForwardToASubmittableRoom {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ForwardToRandomCompetitionRoom {
    pub goal_code: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetCurrentTimingCode {
    pub timing_code: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetIsUserPartOfCompetition {
    pub goal_code: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetSecondsUntil {
    pub time: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RoomCompetitionInit {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct SubmitRoomToCompetition {
    pub goal_code: String,
    pub event_type: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct VoteForRoom {
    pub goal_code: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct FriendFurniConfirmLock {
    pub stuff_id: LegacyId,
    pub canceled: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct MysteryBoxWaitingCanceled {
    pub owner_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct AcceptQuest {
    pub quest_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ActivateQuest {
    pub quest_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct CancelQuest {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct FriendRequestQuestComplete {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetCommunityGoalHallOfFame {
    pub code: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetCommunityGoalProgress {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetConcurrentUsersGoalProgress {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetConcurrentUsersReward {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetDailyQuest {
    pub is_easy: bool,
    pub index: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetQuests {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetSeasonalQuestsOnly {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct OpenQuestTracker {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RejectQuest {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct StartCampaign {
    pub campaign_name: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetResolutionAchievements {
    pub stuff_id: LegacyId,
    pub _unknown: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct CloseIssueDefaultAction {
    pub highest_priority_issue_id: LegacyId,
    pub issue_ids: Vec<LegacyId>,
    pub topic_id: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct CloseIssues {
    pub reason: i32,
    pub issue_ids: Vec<LegacyId>
}

#[derive(BaseParser, Clone, Debug, Default, PartialEq)]
#[to(direction = 1)]
pub struct DefaultSanction {
    pub user_id: LegacyId,
    pub topic_id: i32,
    pub message: String,
    pub issue_id: LegacyId
}

impl PacketVariable for DefaultSanction {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        (Self {
            user_id: packet.read(),
            topic_id: packet.read(),
            message: packet.read(),
            issue_id: packet.read::<Option<LegacyId>>().unwrap_or(LegacyId(-1))
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        if *self.issue_id == -1 {
            (
                self.user_id, self.topic_id, self.message.clone()
            ).to_packet()
        } else {
            (
                self.user_id, self.topic_id, self.message.clone(), self.issue_id
            ).to_packet()
        }
    }
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetCfhChatlog {
    pub issue_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetModeratorRoomInfo {
    pub room_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetModeratorUserInfo {
    pub user_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetRoomChatlog {
    pub is_guest_room: i32,
    pub room_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetRoomVisits {
    pub user_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetUserChatlog {
    pub user_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PartialEq)]
#[to(direction = 1)]
pub struct ModAlert {
    pub user_id: LegacyId,
    pub message: String,
    pub topic_id: i32,
    pub issue_id: LegacyId
}

impl PacketVariable for ModAlert {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        (Self {
            user_id: packet.read(),
            message: packet.read(),
            topic_id: packet.read(),
            issue_id: packet.read::<Option<LegacyId>>().unwrap_or(LegacyId(-1))
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        if *self.issue_id == -1 {
            (
                self.user_id, self.message.clone(), self.topic_id
            ).to_packet()
        } else {
            (
                self.user_id, self.message.clone(), self.topic_id, self.issue_id
            ).to_packet()
        }
    }
}

#[derive(BaseParser, Clone, Debug, Default, PartialEq)]
#[to(direction = 1)]
pub struct ModBan {
    pub user_id: LegacyId,
    pub message: String,
    pub topic_id: i32,
    pub sanction_type_id: i32,
    pub is_perm_ban: bool,
    pub issue_id: LegacyId
}

impl PacketVariable for ModBan {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        (Self {
            user_id: packet.read(),
            message: packet.read(),
            topic_id: packet.read(),
            sanction_type_id: packet.read(),
            is_perm_ban: packet.read(),
            issue_id: packet.read::<Option<LegacyId>>().unwrap_or(LegacyId(-1))
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        if *self.issue_id == -1 {
            (
                self.user_id, self.message.clone(), self.topic_id,
                self.sanction_type_id, self.is_perm_ban
            ).to_packet()
        } else {
            (
                self.user_id, self.message.clone(), self.topic_id,
                self.sanction_type_id, self.is_perm_ban, self.issue_id
            ).to_packet()
        }
    }
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ModerateRoom {
    pub room_id: LegacyId,
    pub lock_room: i32,
    pub force_room_change: i32,
    pub kick_everyone: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ModeratorAction {
    pub action: i32,
    pub message: String,
    pub _unknown: String
}

#[derive(BaseParser, Clone, Debug, Default, PartialEq)]
#[to(direction = 1)]
pub struct ModKick {
    pub user_id: LegacyId,
    pub message: String,
    pub topic_id: i32,
    pub issue_id: LegacyId
}

impl PacketVariable for ModKick {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        (Self {
            user_id: packet.read(),
            message: packet.read(),
            topic_id: packet.read(),
            issue_id: packet.read::<Option<LegacyId>>().unwrap_or(LegacyId(-1))
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        if *self.issue_id == -1 {
            (
                self.user_id, self.message.clone(), self.topic_id
            ).to_packet()
        } else {
            (
                self.user_id, self.message.clone(), self.topic_id, self.issue_id
            ).to_packet()
        }
    }
}

#[derive(BaseParser, Clone, Debug, Default, PartialEq)]
#[to(direction = 1)]
pub struct ModMessage {
    pub user_id: LegacyId,
    pub message: String,
    pub _unknown1: String,
    pub _unknown2: String,
    pub topic_id: i32,
    pub issue_id: LegacyId
}

impl PacketVariable for ModMessage {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        (Self {
            user_id: packet.read(),
            message: packet.read(),
            _unknown1: packet.read(),
            _unknown2: packet.read(),
            topic_id: packet.read(),
            issue_id: packet.read::<Option<LegacyId>>().unwrap_or(LegacyId(-1))
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        if *self.issue_id == -1 {
            (
                self.user_id, self.message.clone(), self._unknown1.clone(),
                self._unknown2.clone(), self.topic_id
            ).to_packet()
        } else {
            (
                self.user_id, self.message.clone(), self._unknown1.clone(),
                self._unknown2.clone(), self.topic_id, self.issue_id
            ).to_packet()
        }
    }
}

#[derive(BaseParser, Clone, Debug, Default, PartialEq)]
#[to(direction = 1)]
pub struct ModMute {
    pub user_id: LegacyId,
    pub message: String,
    pub topic_id: i32,
    pub issue_id: LegacyId
}

impl PacketVariable for ModMute {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        (Self {
            user_id: packet.read(),
            message: packet.read(),
            topic_id: packet.read(),
            issue_id: packet.read::<Option<LegacyId>>().unwrap_or(LegacyId(-1))
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        if *self.issue_id == -1 {
            (
                self.user_id, self.message.clone(), self.topic_id
            ).to_packet()
        } else {
            (
                self.user_id, self.message.clone(), self.topic_id, self.issue_id
            ).to_packet()
        }
    }
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ModToolPreferences {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ModToolSanction {
    pub highest_priority_issue_id: LegacyId,
    pub user_id: LegacyId,
    pub topic_id: i32,
}

#[derive(BaseParser, Clone, Debug, Default, PartialEq)]
#[to(direction = 1)]
pub struct ModTradingLock {
    pub user_id: LegacyId,
    pub message: String,
    pub length_min: i32,
    pub topic_id: i32,
    pub issue_id: LegacyId,
}

impl PacketVariable for ModTradingLock {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        (Self {
            user_id: packet.read(),
            message: packet.read(),
            length_min: packet.read(),
            topic_id: packet.read(),
            issue_id: packet.read::<Option<LegacyId>>().unwrap_or(LegacyId(-1))
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        if *self.issue_id == -1 {
            (
                self.user_id, self.message.clone(), self.length_min, self.topic_id
            ).to_packet()
        } else {
            (
                self.user_id, self.message.clone(), self.length_min, self.topic_id, self.issue_id
            ).to_packet()
        }
    }
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct PickIssues {
    pub issue_ids: Vec<LegacyId>,
    pub retry_enabled: bool,
    pub retry_count: i32,
    pub error: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ReleaseIssues {
    pub issue_ids: Vec<LegacyId>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct Game2MakeSnowball {
    pub turn: i32,
    pub sub_turn: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct Game2RequestFullStatusUpdate {
    pub status: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct Game2SetUserMoveTarget {
    pub x: i32,
    pub y: i32,
    pub turn: i32,
    pub sub_turn: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct Game2ThrowSnowballAtHuman {
    pub user_id: LegacyId,
    pub trajectory: i32,
    pub turn: i32,
    pub sub_turn: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct Game2ThrowSnowballAtPosition {
    pub x: i32,
    pub y: i32,
    pub trajectory: i32,
    pub turn: i32,
    pub sub_turn: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct AmbassadorAlert {
    pub user_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct AssignRights {
    pub user_id: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct BanUserWithDuration {
    pub user_id: i32,
    pub ban_type: String,
    pub room_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct KickUser {
    pub user_id: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct LetUserIn {
    pub user_name: String,
    pub can_enter: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct MuteAllInRoom {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct MuteUser {
    pub user_id: LegacyId,
    pub mute_type: i32,
    pub room_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RemoveAllRights {
    pub room_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RemoveRights {
    pub user_ids: Vec<LegacyId>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct UnbanUserFromRoom {
    pub user_id: LegacyId,
    pub room_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct UnmuteUser {
    pub user_id: LegacyId,
    pub room_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct NavigatorAddCollapsedCategory {
    pub search_code: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct NavigatorAddSaved {
    pub search_code: String,
    pub filtering: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct NavigatorDeleteSaved {
    pub id: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct NavigatorRemoveCollapsedCategory {
    pub search_code: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct NavigatorSetSearchCodeViewMode {
    pub search_code: String,
    pub view_mode: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct NewNavigatorInit {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct NewNavigatorSearch {
    pub search_code: String,
    pub filtering: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct BreedPets {
    pub action_id: i32,
    pub pet1_id: LegacyId,
    pub pet2_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct CustomizePetWithFurni {
    pub item_id: LegacyId,
    pub pet_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetPetInfo {
    pub pet_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct PetSelected {
    pub pet_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RespectPet {
    pub pet_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct CommunityGoalVote {
    pub option: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetAchievements {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetBadgePointLimits {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetBadges {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetIsBadgeRequestFulfilled {
    pub badge_code: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RequestABadge {
    pub badge_code: String
}

#[derive(BaseParser, Clone, Debug, Default, PartialEq)]
#[to(direction = 1)]
pub struct SetActivatedBadges {
    pub badges: [String; 5]
}

impl PacketVariable for SetActivatedBadges {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);
        let indexed_badges = packet.read::<[(i32, String); 5]>();

        (Self {
            badges: [
                indexed_badges[0].1.clone(), indexed_badges[1].1.clone(),
                indexed_badges[2].1.clone(), indexed_badges[3].1.clone(),
                indexed_badges[4].1.clone()
            ]
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        let mut packet = HPacket::from_header_id(0);

        for i in 0..5 {
            packet.append((i as i32, self.badges[i].clone()));
        }

        packet.get_bytes()[6..].to_vec()
    }
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct Game2ExitGame {
    pub exit: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct Game2GameChat {
    pub message: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct Game2LoadStageReady {
    pub percentage: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct Game2PlayAgain {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetCreditsInfo {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct CommandBot {
    pub bot_id: LegacyId,
    pub skill_type: i32,
    pub command_string: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetBotCommandConfigurationData {
    pub bot_id: LegacyId,
    pub skill_type: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct AvatarEffectActivated {
    pub effect_id: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct AvatarEffectSelected {
    pub effect_id: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ChangeQueue {
    pub queue_type: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct OpenFlatConnection {
    pub room_id: LegacyId,
    pub room_password: String,
    pub _unknown: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct Quit {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetSelectedNftWardrobeOutfit {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetUserNftWardrobe {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct SaveUserNftWardrobe {
    pub token_id: String
}

#[derive(BaseParser, Clone, Debug, Default, PartialEq)]
#[to(direction = 1)]
pub struct NewUserExperienceGetGifts {
    pub selections: Vec<NewUserExperienceGetGiftsSelection> // (day_index, step_index, gift_index)
}

impl PacketVariable for NewUserExperienceGetGifts {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        let mut selections: Vec<NewUserExperienceGetGiftsSelection> = Vec::new();
        let size: LegacyLength = packet.read();
        for _ in 0..*size/3 {
            selections.push(packet.read());
        }

        (Self {
            selections
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        let mut packet = HPacket::from_header_id(0);

        packet.append(LegacyLength(self.selections.len() as i32 * 3));
        for selection in self.selections.clone() {
            packet.append(selection);
        }

        packet.get_bytes()[6..].to_vec()
    }
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct NewUserExperienceScriptProceed {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ClickFurni {
    pub item_id: LegacyId,
    pub _unknown: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct CompostPlant {
    pub pet_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetFurnitureAliases {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetHeightMap {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetItemData {
    pub item_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetPetCommands {
    pub pet_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GiveSupplementToPet {
    pub pet_id: LegacyId,
    pub supplement_type: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct HarvestPet {
    pub pet_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct MountPet {
    pub pet_id: LegacyId,
    pub mounted: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct MoveAvatar {
    pub x: i32,
    pub y: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct MoveObject {
    pub item_id: LegacyId,
    pub x: i32,
    pub y: i32,
    pub direction: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct MovePet {
    pub item_id: LegacyId,
    pub x: i32,
    pub y: i32,
    pub direction: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct MoveWallItem {
    pub item_id: LegacyId,
    pub location: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct PickupObject {
    pub placement_type: i32,
    pub item_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct PlaceBot {
    pub bot_id: LegacyId,
    pub x: i32,
    pub y: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct PlaceObject {
    pub data: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct PlacePet {
    pub pet_id: LegacyId,
    pub x: i32,
    pub y: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RemoveBotFromFlat {
    pub bot_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RemoveItem {
    pub item_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RemovePetFromFlat {
    pub pet_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RemoveSaddleFromPet {
    pub pet_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct SetClothingChangeData {
    pub item_id: LegacyId,
    pub figure_string: String,
    pub gender: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct SetItemData {
    pub item_id: LegacyId,
    pub color_hex: String,
    pub text: String
}

#[derive(BaseParser, Clone, Debug, Default, PartialEq)]
#[to(direction = 1)]
pub struct SetObjectData {
    pub item_id: LegacyId,
    pub data: HashMap<String, String>
}

impl PacketVariable for SetObjectData {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        let item_id = packet.read();
        let size: LegacyLength = packet.read();
        let mut data: HashMap<String, String> = HashMap::new();
        for _ in 0..*size/2 {
            data.insert(packet.read(), packet.read());
        }

        (Self {
            item_id, data
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        let mut packet = HPacket::from_header_id(0);

        packet.append((self.item_id, LegacyLength(self.data.len() as i32 * 2)));
        for (key, val) in self.data.clone() {
            packet.append((key, val));
        }

        packet.get_bytes()[6..].to_vec()
    }
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct TogglePetBreedingPermission {
    pub pet_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct TogglePetRidingPermission {
    pub pet_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct UseFurniture {
    pub item_id: LegacyId,
    pub param: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct UseWallItem {
    pub item_id: LegacyId,
    pub param: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ResetPhoneNumberState {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct SetPhoneNumberVerificationStatus {
    pub status: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct TryPhoneNumber {
    pub country_code: String,
    pub phone_number: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct VerifyCode {
    pub verification_code: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RoomNetworkOpenConnection {
    pub network_id: LegacyId,
    pub home_room_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct AddJukeboxDisk {
    pub disk_id: LegacyId,
    pub slot_number: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetJukeboxPlayList {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetNowPlaying {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetOfficialSongId {
    pub song_id: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetSongInfo {
    pub song_ids: Vec<i32>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetSoundMachinePlayList {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetSoundSettings {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetUserSongDisks {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RemoveJukeboxDisk {
    pub slot_number: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetForumsList {
    pub list_code: i32,
    pub start_index: i32,
    pub count: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetForumStats {
    pub group_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetMessages {
    pub group_id: LegacyId,
    pub thread_id: LegacyId,
    pub start_index: i32,
    pub count: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetThread {
    pub group_id: LegacyId,
    pub thread_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetThreads {
    pub group_id: LegacyId,
    pub start_index: i32,
    pub count: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetUnreadForumsCount {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ModerateMessage {
    pub group_id: LegacyId,
    pub thread_id: LegacyId,
    pub message_id: LegacyId,
    pub action_id: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ModerateThread {
    pub group_id: LegacyId,
    pub thread_id: LegacyId,
    pub action_id: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct PostMessage {
    pub group_id: LegacyId,
    pub thread_id: LegacyId,
    pub subject: String,
    pub message: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct UpdateForumReadMarker {
    pub data: Vec<UpdateForumReadMarkerData>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct UpdateForumSettings {
    pub group_id: LegacyId,
    pub read_permissions: i32,
    pub post_message_permissions: i32,
    pub post_thread_permissions: i32,
    pub moderate_permissions: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct UpdateThread {
    pub group_id: LegacyId,
    pub thread_id: LegacyId,
    pub is_locked: bool,
    pub is_sticky: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct EventLog {
    pub event_category: String,
    pub event_type: String,
    pub action: String,
    pub extra_string: String,
    pub extra_int: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct LagWarningReport {
    pub lag_level: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct LatencyPingReport {
    pub sum: i32,
    pub average: i32,
    pub count: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct LatencyPingRequest {
    pub request_id: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct PerformanceLog {
    pub time_stamp: i32,
    pub user_agent: String,
    pub flash_version: String,
    pub operating_system: String,
    pub _unknown1: String,
    pub is_debugger: bool,
    pub total_memory: i32,
    pub _unknown2: i32,
    pub garbage_collection_count: i32,
    pub average_update_interval: i32,
    pub update_count: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct CallForHelpFromForumMessage {
    pub group_id: LegacyId,
    pub thread_id: LegacyId,
    pub message_id: LegacyId,
    pub topic_id: i32,
    pub message: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct CallForHelpFromForumThread {
    pub group_id: LegacyId,
    pub thread_id: LegacyId,
    pub topic_id: i32,
    pub message: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct CallForHelpFromIM {
    pub message: String,
    pub topic_id: i32,
    pub user_id: LegacyId,
    pub messages: Vec<CallForHelpFromIMMessage>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct CallForHelpFromPhoto {
    pub image: String,
    pub name: i32,
    pub room_id: LegacyId,
    pub creator_id: LegacyId,
    pub item_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct CallForHelpFromSelfie {
    pub image: String,
    pub message: i32,
    pub room_id: LegacyId,
    pub creator_id: LegacyId,
    pub item_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct CallForHelp {
    pub message: String,
    pub topic_id: i32,
    pub user_id: LegacyId,
    pub room_id: LegacyId,
    pub messages: Vec<CallForHelpFromIMMessage>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ChatReviewGuideDecidesOnOffer {
    pub accepted: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ChatReviewGuideDetached {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ChatReviewGuideVote {
    pub vote: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ChatReviewSessionCreate {
    pub user_id: LegacyId,
    pub room_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct DeletePendingCallsForHelp {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetCfhStatus {
    pub show: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetGuideReportingStatus {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetPendingCallsForHelp {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetQuizQuestions {
    pub name: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GuideSessionCreate {
    pub request_type: i32,
    pub message: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GuideSessionFeedback {
    pub positive: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GuideSessionGetRequesterRoom {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GuideSessionGuideDecides {
    pub accepted: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GuideSessionInviteRequester {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GuideSessionIsTyping {
    pub is_typing: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GuideSessionMessage {
    pub message: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GuideSessionOnDutyUpdate {
    pub on_duty: bool,
    pub handle_guide_tickets: bool,
    pub handle_helper_tickets: bool,
    pub handle_guardian_tickets: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GuideSessionReport {
    pub message: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GuideSessionRequesterCancels {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GuideSessionResolved {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct PostQuizAnswers {
    pub quiz_id: String,
    pub answers: Vec<i32>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetInterstitial {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct InterstitialShown {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RequestFurniInventory {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RequestFurniInventoryWhenNotInRoom {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RequestRoomPropertySet {
    pub room_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct AcceptTrading {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct AddItemsToTrade {
    pub item_ids: Vec<LegacyId>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct AddItemToTrade {
    pub item_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct CloseTrading {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ConfirmAcceptTrading {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ConfirmDeclineTrading {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct OpenTrading {
    pub user_index: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RemoveItemFromTrade {
    pub item_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct UnacceptTrading {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetHotLooks {
    pub count: i8
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct BuildersClubPlaceRoomItem {
    pub page_id: LegacyId,
    pub offer_id: LegacyId,
    pub extra_param: String,
    pub x: i32,
    pub y: i32,
    pub direction: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct BuildersClubPlaceWallItem {
    pub page_id: LegacyId,
    pub offer_id: LegacyId,
    pub extra_param: String,
    pub location: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct BuildersClubQueryFurniCount {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetBonusRareInfo {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetBundleDiscountRuleset {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetCatalogIndex {
    pub catalog_type: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetCatalogPage {
    pub page_id: LegacyId,
    pub offer_id: LegacyId,
    pub catalog_type: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetCatalogPageWithEarliestExpiry {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetClubGift {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetClubOffers {
    pub _unknown: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetGiftWrappingConfiguration {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetHabboClubExtendOffer {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetIsOfferGiftable {
    pub offer_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetLimitedOfferAppearingNext {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetNextTargetedOffer {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetProductOffer {
    pub offer_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetRoomAdPurchaseInfo {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetSeasonalCalendarDaily {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetSellablePetPalettes {
    pub localization_id: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetSnowWarGameTokensOffer {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct MarkCatalogNewAdditionsPageOpened {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct PurchaseBasicMembershipExtension {
    pub offer_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct PurchaseFromCatalogAsGift {
    pub page_id: LegacyId,
    pub offer_id: LegacyId,
    pub localization_id: String,
    pub receiver_name: String,
    pub message: String,
    pub color: i32,
    pub box_type: i32,
    pub ribbon_type: i32,
    pub show_purchaser_name: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct PurchaseFromCatalog {
    pub page_id: LegacyId,
    pub offer_id: LegacyId,
    pub localization_id: String,
    pub amount: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct PurchaseRoomAd {
    pub page_id: LegacyId,
    pub offer_id: LegacyId,
    pub room_id: LegacyId,
    pub name: String,
    pub extended: bool,
    pub description: String,
    pub category_id: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct PurchaseSnowWarGameTokensOffer {
    pub offer_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct PurchaseTargetedOffer {
    pub page_id: LegacyId,
    pub offer_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct PurchaseVipMembershipExtension {
    pub offer_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RedeemVoucher {
    pub voucher_code: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RoomAdPurchaseInitiated {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct SelectClubGift {
    pub localization_id: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct SetTargetedOfferState {
    pub offer_id: LegacyId,
    pub state: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ShopTargetedOfferViewed {
    pub offer_id: LegacyId,
    pub state: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct Craft {
    pub crafting_table_id: LegacyId,
    pub craft_type: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct CraftSecret {
    pub crafting_table_id: LegacyId,
    pub selected_ingredient_ids: Vec<LegacyId>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetCraftableProducts {
    pub crafting_table_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetCraftingRecipe {
    pub product_code: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetCraftingRecipesAvailable {
    pub crafting_table_id: LegacyId,
    pub selected_ingredient_ids: Vec<LegacyId>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ChangeUserName {
    pub user_name: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct CheckUserName {
    pub user_name: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetWardrobe {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct SaveWardrobeOutfit {
    pub slot_id: i32,
    pub figure_string: String,
    pub gender: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ResetUnseenItemIds {
    pub category_id: i32,
    pub p2: Vec<LegacyId>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ResetUnseenItems {
    pub category_id: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct CancelTyping {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct Chat {
    pub text: String,
    pub chat_style: i32,
    pub index: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct Shout {
    pub text: String,
    pub chat_style: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct StartTyping {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct Whisper {
    pub text: String,
    pub chat_style: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetBotInventory {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct CreditVaultStatus {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct IncomeRewardClaim {
    pub reward_type: i8
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct IncomeRewardStatus {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct WithdrawCreditVault {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct Game2CheckGameDirectoryStatus {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct Game2GetAccountGameStatus {
    pub _unknown: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct Game2LeaveGame {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct Game2QuickJoinGame {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct Game2StartSnowWar {
    pub game_identifier: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct AddSpamWallPostIt {
    pub item_id: LegacyId,
    pub location: String,
    pub color_hex: String,
    pub text: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ControlYoutubeDisplayPlayback {
    pub item_id: LegacyId,
    pub action: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct CreditFurniRedeem {
    pub item_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct DiceOff {
    pub item_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct EnterOneWayDoor {
    pub item_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ExtendRentOrBuyoutFurni {
    pub is_wall_item: bool,
    pub item_id: LegacyId,
    pub is_buyout: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ExtendRentOrBuyoutStripItem {
    pub item_id: LegacyId,
    pub is_buyout: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetGuildFurniContextMenuInfo {
    pub item_id: LegacyId,
    pub group_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetRentOrBuyoutOffer {
    pub is_wall_item: bool,
    pub full_name: String,
    pub is_buyout: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetYoutubeDisplayStatus {
    pub item_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct OpenMysteryTrophy {
    pub item_id: LegacyId,
    pub trophy_inscription: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct OpenPetPackage {
    pub item_id: LegacyId,
    pub name: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct PlacePostIt {
    pub item_id: LegacyId,
    pub location: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct PresentOpen {
    pub item_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RentableSpaceCancelRent {
    pub item_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RentableSpaceRent {
    pub item_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RentableSpaceStatus {
    pub item_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RoomDimmerChangeState {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RoomDimmerGetPresets {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RoomDimmerSavePreset {
    pub preset_number: i32,
    pub effect_type_id: i32,
    pub color: String,
    pub brightness: i32,
    pub apply: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct SetCustomStackingHeight {
    pub stack_tile_id: LegacyId,
    pub height: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct SetMannequinFigure {
    pub mannequin_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct SetMannequinName {
    pub mannequin_id: LegacyId,
    pub name: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct SetRandomState {
    pub item_id: LegacyId,
    pub param: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct SetRoomBackgroundColorData {
    pub item_id: LegacyId,
    pub hue: i32,
    pub saturation: i32,
    pub lightness: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct SetYoutubeDisplayPlaylist {
    pub item_id: LegacyId,
    pub name: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct SpinWheelOfFortune {
    pub item_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ThrowDice {
    pub item_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct CancelPetBreeding {
    pub stuff_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct ConfirmPetBreeding {
    pub stuff_id: LegacyId,
    pub name: String,
    pub pet1_id: LegacyId,
    pub pet2_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetPetInventory {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct PhotoCompetition {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct PublishPhoto {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct PurchasePhoto {}

#[derive(BaseParser, Clone, Debug, Default, PartialEq)]
#[to(direction = 1)]
pub struct RenderRoom {
    pub json_string: String
}

impl PacketVariable for RenderRoom {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);
        let compressed: Vec<u8> = packet.read();
        let decompressed = decompress_to_vec_zlib(compressed.as_slice()).expect("Couldn't decompress RenderRoom packet");
        (Self {
            json_string: String::from_utf8(decompressed).expect("Couldn't form string from decompressed RenderRoom bytes")
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        let decompressed = self.json_string.as_bytes();
        compress_to_vec_zlib(decompressed, 10).to_packet()
    }
}

#[derive(BaseParser, Clone, Debug, Default, PartialEq)]
#[to(direction = 1)]
pub struct RenderRoomThumbnail {
    pub json_string: String
}

impl PacketVariable for RenderRoomThumbnail {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);
        let compressed: Vec<u8> = packet.read();
        let decompressed = decompress_to_vec_zlib(compressed.as_slice()).expect("Couldn't decompress RenderRoom packet");
        (Self {
            json_string: String::from_utf8(decompressed).expect("Couldn't form string from decompressed RenderRoom bytes")
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        let decompressed = self.json_string.as_bytes();
        compress_to_vec_zlib(decompressed, 10).to_packet()
    }
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct RequestCameraConfiguration {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
#[to(direction = 1)]
pub struct GetPromoArticles {}





