use std::collections::HashMap;
use crate::extension::parsers::stuffdata::StuffData;
use crate::protocol::hpacket::HPacket;
use crate::protocol::vars::legacy::{LegacyDouble, LegacyId, LegacyLength, LegacyStringId};
use crate::protocol::vars::packetvariable::PacketVariable;
use super::baseparser::BaseParser;
use super::subparsers::*;

// WIN63-202303241432-611275200

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CraftableProducts {
    pub recipe_product_items: Vec<FurnitureProductItem>,
    pub usable_inventory_furni_classes: Vec<String>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CraftingRecipe {
    pub ingredients: Vec<OutgoingIngredient>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CraftingRecipesAvailable {
    pub count: i32,
    pub recipe_complete: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CraftingResult {
    pub success: bool,
    pub product_data: FurnitureProductItem
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct YouAreController {
    pub flat_id: LegacyId,
    pub room_controller_level: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct YouAreNotController {
    pub flat_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct YouAreOwner {
    pub flat_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CancelMysteryBoxWait {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GotMysteryBoxPrize {
    pub content_type: String,
    pub class_id: i32 // Might be a LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct MysteryBoxKeys {
    pub box_color: String,
    pub key_color: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ShowMysteryBoxWait {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct AccountSafetyLockStatusChange {
    pub status: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ApproveName {
    pub result: i32,
    pub name_validation_info: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ChangeEmailResult {
    pub result: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct EmailStatusResult {
    pub email: String,
    pub is_verified: bool,
    pub allow_change: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ExtendedProfileChanged {
    pub user_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ExtendedProfile {
    pub data: ExtendedProfileData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GroupDetailsChanged {
    pub group_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GroupMembershipRequested {
    pub group_id: LegacyId,
    pub requester: MemberData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GuildCreated {
    pub base_room_id: LegacyId,
    pub group_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GuildCreationInfo {
    pub data: GuildCreationData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GuildEditFailed {
    pub reason: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GuildEditInfo {
    pub data: GuildEditData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GuildEditorData {
    pub base_parts: Vec<BadgePartData>,
    pub layer_parts: Vec<BadgePartData>,
    pub badge_colors: Vec<GuildColorData>,
    pub primary_colors: Vec<GuildColorData>,
    pub secondary_colors: Vec<GuildColorData>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GuildMemberFurniCountInHQ {
    pub user_id: LegacyId,
    pub furni_count: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GuildMemberMgmtFailed {
    pub guild_id: LegacyId,
    pub reason: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GuildMembershipRejected {
    pub guild_id: LegacyId,
    pub user_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GuildMemberships {
    pub guilds: Vec<HabboGroupEntryData>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GuildMembershipUpdated {
    pub guild_id: LegacyId,
    pub data: MemberData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GuildMembers {
    pub data: Vec<GuildMemberData>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct HabboGroupBadges {
    pub badges: HashMap<i32, String>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct HabboGroupDeactivated {
    pub group_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct HabboGroupDetails {
    pub data: HabboGroupDetailsData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct HabboGroupJoinFailed {
    pub reason: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct HabboUserBadges {
    pub user_id: LegacyId,
    pub badges: HashMap<i32, String>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct HandItemReceived {
    pub giver_user_id: LegacyId,
    pub hand_item_type: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct IgnoredUsers {
    pub ignored_users: Vec<String>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct IgnoreResult {
    pub result: i32,
    pub name: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct InClientLink {
    pub link: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PetRespectNotification {
    pub respect: i32,
    pub pet_owner_id: LegacyId,
    pub pet_data: PetData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PetSupplementedNotification {
    pub pet_id: LegacyId,
    pub user_id: LegacyId,
    pub supplement_type: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RelationshipStatusInfo {
    pub user_id: LegacyId,
    pub relationship_statuses: Vec<RelationshipStatusInfoData>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RespectNotification {
    pub user_id: LegacyId,
    pub respect_total: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ScrSendKickbackInfo {
    pub data: ScrKickbackData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ScrSendUserInfo {
    pub product_name: String,
    pub days_to_period_end: i32,
    pub member_periods: i32,
    pub periods_subscribed_ahead: i32,
    pub response_type: i32,
    pub has_ever_been_member: bool,
    pub is_vip: bool,
    pub past_club_days: bool,
    pub past_vip_days: bool,
    pub minutes_until_expiration: i32,
    pub minutes_since_last_modified: Option<i32>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct UserNameChanged {
    pub web_id: i32, // Might be a LegacyId
    pub id: LegacyId,
    pub new_name: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RoomEntryTile {
    pub x: i32,
    pub y: i32,
    pub dir: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RoomOccupiedTiles {
    pub occupied_tiles: Vec<(i32, i32)>
}

#[derive(BaseParser, Clone, Debug, Default, PartialEq)]
pub struct CantConnect {
    pub reason: i32,
    pub parameter: String
}

impl PacketVariable for CantConnect {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);
        let reason = packet.read();
        let parameter = if reason == 3 { packet.read() } else { String::from("") };
        (CantConnect { reason, parameter }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        let mut packet = HPacket::from_header_id(0);
        packet.append(self.reason);
        if self.reason == 3 {
            packet.append(self.parameter.clone());
        }
        packet.get_bytes()[6..].to_vec()
    }
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CloseConnection {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct FlatAccessible {
    pub flat_id: LegacyId,
    pub user_name: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GamePlayerValue {
    pub user_id: LegacyId,
    pub value: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct OpenConnection {
    pub flat_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RoomForward {
    pub room_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RoomQueueStatus {
    pub flat_id: LegacyId,
    pub queue: RoomQueueSet
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RoomReady {
    pub room_type: String,
    pub room_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct YouArePlayingGame {
    pub is_playing: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct YouAreSpectator {
    pub flat_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CampaignCalendarData {
    pub campaign_name: String,
    pub campaign_image: String,
    pub current_day: i32,
    pub campaign_days: i32,
    pub opened_days: Vec<i32>,
    pub missed_days: Vec<i32>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CampaignCalendarDoorOpened {
    pub door_opened: bool,
    pub product_name: String,
    pub custom_image: String,
    pub furniture_class_name: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Interstitial {
    pub can_show_interstitial: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RoomAdError {
    pub error_code: i32,
    pub filtered_text: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ForumData {
    pub forum_data: ExtendedForumData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ForumsList {
    pub list_code: i32,
    pub total_amount: i32,
    pub start_index: i32,
    pub amount: i32,
    pub forums: Vec<BaseForumData>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ForumThreads {
    pub group_id: LegacyId,
    pub start_index: i32,
    pub amount: i32,
    pub threads: Vec<ThreadData>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PostMessage {
    pub group_id: LegacyId,
    pub thread_id: LegacyId,
    pub message: MessageData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PostThread {
    pub group_id: LegacyId,
    pub thread: ThreadData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ThreadMessages {
    pub group_id: LegacyId,
    pub thread_id: LegacyId,
    pub start_index: i32,
    pub amount: i32,
    pub messages: Vec<MessageData>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct UnreadForumsCount {
    pub count: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct UpdateMessage {
    pub group_id: LegacyId,
    pub thread_id: LegacyId,
    pub message: MessageData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct UpdateThread {
    pub group_id: LegacyId,
    pub thread: ThreadData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PhoneCollectionState {
    pub phone_status_code: i32,
    pub collection_status_code: i32,
    pub milliseconds_to_allow_process_reset: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct TryPhoneNumberResult {
    pub result_code: i32,
    pub millis_to_allow_process_reset: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct TryVerificationCodeResult {
    pub result_code: i32,
    pub milliseconds_to_allow_process_reset: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CompetitionEntrySubmitResult {
    pub goal_id: LegacyId,
    pub goal_code: String,
    pub result: i32,
    pub required_furnis: Vec<String>,
    pub present_furnis: Vec<String>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CompetitionVotingInfo {
    pub goal_id: LegacyId,
    pub goal_code: String,
    pub result_code: i32,
    pub votes_remaining: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CurrentTimingCode {
    pub scheduling_str: String,
    pub code: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct IsUserPartOfCompetition {
    pub is_part_of: bool,
    pub target_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct NoOwnedRoomsAlert {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct SecondsUntil {
    pub time_str: String,
    pub seconds_until: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct AchievementResolutionCompleted {
    pub stuff_code: String,
    pub badge_code: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct AchievementResolutionProgress {
    pub stuff_id: LegacyId,
    pub achievement_id: LegacyId,
    pub required_level_badge_code: String,
    pub user_progress: i32,
    pub total_progress: i32,
    pub end_time: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct AchievementResolutions {
    pub stuff_id: LegacyId,
    pub achievements: Vec<AchievementResolutionData>,
    pub end_time: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Chat {
    pub user_index: i32,
    pub text: String,
    pub gesture: i32,
    pub style_id: i32,
    pub links: Vec<(String, String, bool)>,
    pub tracking_id: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct FloodControl {
    pub seconds: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RemainingMutePeriod {
    pub seconds_remaining: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RoomChatSettings {
    pub mode: i32,
    pub bubble_width: i32,
    pub scroll_speed: i32,
    pub full_hear_range: i32,
    pub flood_sensitivity: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RoomFilterSettings {
    pub bad_words: Vec<String>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Shout {
    pub user_index: i32,
    pub text: String,
    pub gesture: i32,
    pub style_id: i32,
    pub links: Vec<(String, String, bool)>,
    pub tracking_id: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct UserTyping {
    pub user_index: i32,
    pub state: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Whisper {
    pub user_index: i32,
    pub text: String,
    pub gesture: i32,
    pub style_id: i32,
    pub links: Vec<(String, String, bool)>,
    pub tracking_id: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CanCreateRoom {
    pub result_code: i32,
    pub room_limit: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CategoriesWithVisitorCount {
    pub data: CategoriesWithVisitorCountData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CompetitionRoomsData {
    pub goal_id: i32,
    pub page_index: i32,
    pub page_count: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ConvertedRoomId {
    pub global_id: String,
    pub converted_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Doorbell {
    pub user_name: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct FavouriteChanged {
    pub flat_id: LegacyId,
    pub added: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Favourites {
    pub limit: i32,
    pub favourite_room_ids: Vec<LegacyId>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct FlatAccessDenied {
    pub flat_id: LegacyId,
    pub user_name: Option<String>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct FlatCreated {
    pub flat_id: LegacyId,
    pub flat_name: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GetGuestRoomResult {
    pub enter_room: bool,
    pub data: GuestRoomData,
    pub room_forward: bool,
    pub staff_pick: bool,
    pub is_group_member: bool,
    pub all_in_room_muted: bool,
    pub room_moderation_settings: RoomModerationSettings,
    pub can_mute: bool,
    pub chat_settings: RoomChatSettings
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GuestRoomSearchResult {
    pub data: GuestRoomSearchResultData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct NavigatorSettings {
    pub home_room_id: LegacyId,
    pub room_id_to_enter: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PartialEq)]
pub struct OfficialRooms {
    pub data: OfficialRoomsData,
    pub ad_room: Option<OfficialRoomEntryData>,
    pub promoted_rooms: PromotedRoomsData
}

impl PacketVariable for OfficialRooms {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        let data = packet.read();
        let ad_room = if packet.read::<i32>() > 0 { packet.read() } else { None };
        let promoted_rooms = packet.read();

        (OfficialRooms { data, ad_room, promoted_rooms }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        let mut packet = HPacket::from_header_id(0);

        packet.append((
            self.data.clone(),
            if self.ad_room.is_some() { 1 } else { 0 },
            self.ad_room.clone(),
            self.promoted_rooms.clone()
        ));

        packet.get_bytes()[6..].to_vec()
    }
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PopularRoomTagsResult {
    pub data: PopularRoomTagsData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RoomEventCancel {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RoomEvent {
    pub data: RoomEventData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RoomInfoUpdated {
    pub flat_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RoomRating {
    pub rating: i32,
    pub can_rate: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct UserEventCats {
    pub event_categories: Vec<EventCategory>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct UserFlatCats {
    pub nodes: Vec<FlatCategory>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CustomStackingHeightUpdate {
    pub furni_id: LegacyId,
    pub height: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CustomUserNotification {
    pub code: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct DiceValue {
    pub id: LegacyId,
    pub value: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct FurniRentOrBuyoutOffer {
    pub is_wall_item: bool,
    pub furni_type_name: String,
    pub buyout: bool,
    pub price_in_credits: i32,
    pub price_in_activity_points: i32,
    pub activity_point_type: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GuildFurniContextMenuInfo {
    pub object_id: LegacyId,
    pub guild_id: LegacyId,
    pub guild_name: String,
    pub guild_home_room_id: LegacyId,
    pub user_is_member: bool,
    pub guild_has_readable_forum: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct OneWayDoorStatus {
    pub id: LegacyId,
    pub status: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct OpenPetPackageRequested {
    pub object_id: LegacyId,
    pub figure_data: Option<PetFigureData>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct OpenPetPackageResult {
    pub object_id: LegacyId,
    pub name_validation_status: i32,
    pub name_validation_info: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PresentOpened {
    pub item_type: String,
    pub class_id: LegacyId,
    pub product_code: String,
    pub placed_item_id: LegacyId,
    pub placed_item_type: String,
    pub placed_in_room: bool,
    pub pet_figure_string: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RentableSpaceRentFailed {
    pub reason: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RentableSpaceRentOk {
    pub expiry_time: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RentableSpaceStatus {
    pub rented: bool,
    pub can_rent_error_code: i32,
    pub renter_id: LegacyId,
    pub renter_name: String,
    pub time_remaining: i32,
    pub price: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RequestSpamWallPostIt {
    pub item_id: LegacyId,
    pub location: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RoomDimmerPresets {
    pub presets: Vec<RoomDimmerPresetsMessageData>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RoomMessageNotification {
    pub room_id: LegacyId,
    pub room_name: String,
    pub message_count: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct YoutubeControlVideo {
    pub furni_id: LegacyId,
    pub command_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct YoutubeDisplayPlaylists {
    pub furni_id: LegacyId,
    pub playlist: Vec<YoutubeDisplayPlaylist>,
    pub selected_playlist_id: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct YoutubeDisplayVideo {
    pub furni_id: LegacyId,
    pub video_id: String,
    pub start_at_seconds: i32,
    pub end_at_seconds: i32,
    pub state: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CfhChatlog {
    pub data: CfhChatlogData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct IssueDeleted {
    pub issue_id: LegacyStringId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct IssueInfo {
    pub issue_data: IssueMessageData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct IssuePickFailed {
    pub issues: Vec<(LegacyId, LegacyId,  String)>,
    pub retry_enabled: bool,
    pub retry_count: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ModeratorActionResult {
    pub user_id: LegacyId,
    pub success: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ModeratorCaution {
    pub message: String,
    pub url: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ModeratorInit {
    pub data: ModeratorInitData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Moderator {
    pub message: String,
    pub url: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ModeratorRoomInfo {
    pub data: RoomModerationData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ModeratorToolPreferences {
    pub window_x: i32,
    pub window_y: i32,
    pub window_width: i32,
    pub window_height: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ModeratorUserInfo {
    pub data: ModeratorUserInfoData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RoomChatlog {
    pub data: ChatRecordData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RoomVisits {
    pub data: RoomVisitsData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct UserBanned {
    pub message: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct UserChatlog {
    pub data: UserChatlogData
}

#[derive(BaseParser, Clone, Debug, Default, PartialEq)]
pub struct PollContents {
    pub id: LegacyId,
    pub start_message: String,
    pub end_message: String,
    pub questions: Vec<PollQuestion>,
    pub nps_poll: bool
}

impl PacketVariable for PollContents {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        let (id, start_message, end_message) = packet.read();
        let mut questions = Vec::new();
        let question_count: LegacyLength = packet.read();
        for _ in 0..*question_count {
            let mut question: PollQuestion = packet.read();
            question.children = packet.read();
            questions.push(question);
        }
        let nps_poll = packet.read();

        (PollContents {
            id, start_message, end_message, questions, nps_poll
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        let mut packet = HPacket::from_header_id(0);

        packet.append((self.id, self.start_message.clone(), self.end_message.clone()));
        packet.append(LegacyLength(self.questions.len() as i32));
        for question in self.questions.iter() {
            packet.append(question.clone());
            packet.append(question.children.clone());
        }
        packet.append(self.nps_poll);

        packet.get_bytes()[6..].to_vec()
    }
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PollError {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PollOffer {
    pub id: LegacyId,
    pub offer_type: String,
    pub headline: String,
    pub summary: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct QuestionAnswered {
    pub user_id: LegacyId,
    pub value: String,
    pub answer_counts: HashMap<String, i32>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Question {
    pub poll_type: String,
    pub poll_id: LegacyId,
    pub question_id: LegacyId,
    pub duration: i32,
    pub question: QuestionData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct QuestionFinished {
    pub question_id: LegacyId,
    pub answer_counts: HashMap<String, i32>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ErrorReport {
    pub message_id: LegacyId,
    pub error_code: i32,
    pub timestamp: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct BonusRareInfo {
    pub product_type: String,
    pub product_class_id: i32, // Might be a LegacyId,
    pub total_coins_for_bonus: i32,
    pub coins_still_required_to_buy: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct BuildersClubFurniCount {
    pub furni_count: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct BuildersClubSubscriptionStatus {
    pub seconds_left: i32,
    pub furni_limit: i32,
    pub max_furni_limit: i32,
    pub seconds_left_with_grace: Option<i32>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct BundleDiscountRuleset {
    pub max_purchase_size: i32,
    pub bundle_size: i32,
    pub bundle_discount_size: i32,
    pub bonus_threshold: i32,
    pub additional_bonus_discount_threshold_quantities: Vec<i32>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CatalogIndex {
    pub root: CatalogNodeData,
    pub new_additions_available: bool,
    pub catalog_type: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CatalogPage {
    pub page_id: LegacyId,
    pub catalog_type: String,
    pub layout_code: String,
    pub localization: CatalogLocalizationData,
    pub offers: Vec<CatalogPageMessageOfferData>,
    pub offer_id: LegacyId,
    pub accept_season_currency_as_credits: bool,
    pub front_page_items: Option<Vec<FrontPageItem>>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CatalogPageWithEarliestExpiry {
    pub page_name: String,
    pub seconds_to_expiry: i32,
    pub image: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CatalogPublished {
    pub instantly_refresh_catalogue: bool,
    pub new_furni_data_hash: Option<String>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ClubGiftInfo {
    pub days_until_next_gift: i32,
    pub gifts_available: i32,
    pub offers: Vec<CatalogPageMessageOfferData>,
    pub gift_data: Vec<ClubGiftData>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ClubGiftSelected {
    pub product_code: String,
    pub products: Vec<CatalogPageMessageProductData>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GiftReceiverNotFound {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GiftWrappingConfiguration {
    pub is_wrapping_enabled: bool,
    pub wrapping_price: i32,
    pub stuff_types: Vec<i32>,
    pub box_types: Vec<i32>,
    pub ribbon_types: Vec<i32>,
    pub default_stuff_types: Vec<i32>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct HabboClubExtendOffer {
    pub offer: ClubOfferExtendData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct LimitedEditionSoldOut {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct LimitedOfferAppearingNext {
    pub appears_in_seconds: i32,
    pub page_id: LegacyId,
    pub offer_id: LegacyId,
    pub product_type: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct NotEnoughBalance {
    pub not_enough_credits: bool,
    pub not_enough_activity_points: bool,
    pub activity_point_type: Option<i32>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ProductOffer {
    pub offer_data: CatalogPageMessageOfferData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PurchaseError {
    pub error_code: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PurchaseNotAllowed {
    pub error_code: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PurchaseOK {
    pub offer: PurchaseOKMessageOfferData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RoomAdPurchaseInfo {
    pub is_vip: bool,
    pub rooms: Vec<RoomEntryData>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct SeasonalCalendarDailyOffer {
    pub page_id: LegacyId,
    pub offer_data: Vec<CatalogPageMessageOfferData>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct SellablePetPalettes {
    pub product_code: String,
    pub sellable_palettes: Vec<SellablePetPaletteData>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct SnowWarGameTokens {
    pub offers: Vec<SnowWarGameTokenOffer>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct TargetedOffer {
    pub tracking_state: i32,
    pub id: LegacyId,
    pub identifier: String,
    pub product_code: String,
    pub price_in_credits: i32,
    pub price_in_activity_points: i32,
    pub activity_point_type: i32,
    pub purchase_limit: i32,
    pub expiration_time: i32,
    pub title: String,
    pub description: String,
    pub image_url: String,
    pub icon_image_url: String,
    pub offer_type: i32,
    pub sub_product_codes: Vec<String>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct TargetedOfferNotFound {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct VoucherRedeemError {
    pub error_code: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct VoucherRedeemOk {
    pub product_description: String,
    pub product_name: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct FavoriteMembershipUpdate {
    pub room_index: i32,
    pub habbo_group_id: LegacyId,
    pub status: i32,
    pub habbo_group_name: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct FloorHeightMap {
    pub is_small_scale: bool,
    pub fixed_walls_height: i32,
    pub text: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct FurnitureAliases {
    pub aliases: HashMap<String, String>
}

#[derive(BaseParser, Clone, Debug, Default, PartialEq)]
pub struct HeightMap {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<HeightMapTile>
}

impl HeightMap {
    pub fn get_tile(self, x: usize, y: usize) -> Option<HeightMapTile> {
        if x >= self.width as usize || y >= self.height as usize || y * self.width as usize + x >= self.tiles.len() {
            None
        } else {
            Some(self.tiles[y * self.width as usize + x].clone())
        }
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: HeightMapTile) {
        if !(x >= self.width as usize || y >= self.height as usize || y * self.width as usize + x >= self.tiles.len()) {
            self.tiles[y * self.width as usize + x] = tile.clone()
        }
    }
}

impl PacketVariable for HeightMap {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        let (width, size): (i32, LegacyLength) = packet.read();
        let height = *size as i32/width;
        let mut tiles = Vec::new();
        for _ in 0..*size {
            tiles.push(packet.read());
        }

        (Self {
            width, height, tiles
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        let mut packet = HPacket::from_header_id(0);

        packet.append((
            self.width, self.width * self.height
        ));
        if self.width * self.height != self.tiles.len() as i32 {
            panic!("HeightMap: There should be width * height tiles");
        }
        for tile in self.tiles.clone() {
            packet.append(tile.clone());
        }

        packet.get_bytes()[6..].to_vec()
    }
}

#[derive(BaseParser, Clone, Debug, Default, PartialEq)]
pub struct HeightMapUpdate {
    pub tile_updates: Vec<HeightMapTileUpdate>
}

impl PacketVariable for HeightMapUpdate {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        let mut tile_updates = Vec::new();
        let count: i16 = packet.read();
        for _ in 0..count {
            tile_updates.push(packet.read());
        }

        (Self {
            tile_updates
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        let mut packet = HPacket::from_header_id(0);

        packet.append(self.tile_updates.len() as i16);
        for update in self.tile_updates.clone() {
            packet.append(update.clone());
        }

        packet.get_bytes()[6..].to_vec()
    }
}

#[derive(BaseParser, Clone, Debug, Default, PartialEq)]
pub struct ItemAdd {
    pub item: WallItem
}

impl PacketVariable for ItemAdd {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        let mut item: WallItem = packet.read();
        item.owner_name = packet.read();

        (Self {
            item
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        self.item.to_packet()
    }
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ItemDataUpdate {
    pub id: LegacyStringId,
    pub item_data: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ItemRemove {
    pub item_id: LegacyStringId,
    pub picker_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Items {
    pub owner_names: HashMap<LegacyId, String>,
    pub items: Vec<WallItem>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ItemUpdate {
    pub data: WallItem
}

#[derive(BaseParser, Clone, Debug, PartialEq)]
pub struct ObjectAdd {
    pub object: FloorItem
}

impl PacketVariable for ObjectAdd {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        let mut object: FloorItem = packet.read();
        object.owner_name = packet.read();

        (Self {
            object
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        self.object.to_packet()
    }
}

#[derive(BaseParser, Clone, Debug, PacketVariable, PartialEq)]
pub struct ObjectDataUpdate {
    pub id: LegacyStringId,
    pub data: StuffData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ObjectRemove {
    pub id: LegacyStringId,
    pub is_expired: bool,
    pub picker_id: LegacyId,
    pub delay: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ObjectsDataUpdate {
    pub data_updates: HashMap<LegacyId, StuffData>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Objects {
    pub owner_names: HashMap<LegacyId, String>,
    pub objects: Vec<FloorItem>
}

#[derive(BaseParser, Clone, Debug, PacketVariable, PartialEq)]
pub struct ObjectUpdate {
    pub data: FloorItem
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RoomEntryInfo {
    pub guest_room_id: LegacyId,
    pub is_owner: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RoomProperty {
    pub prop_type: String,
    pub prop_value: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RoomVisualizationSettings {
    pub walls_hidden: bool,
    pub wall_thickness: i32,
    pub floor_thickness: i32
}

#[derive(BaseParser, Clone, Debug, Default, PartialEq)]
pub struct SlideObjectBundle {
    pub old_x: i32,
    pub old_y: i32,
    pub new_x: i32,
    pub new_y: i32,
    pub objects_z: HashMap<LegacyId, SlideObjectHeight>,
    pub avatar_movement_type: i32,
    pub avatar_id: LegacyId,
    pub avatar_old_z: LegacyDouble,
    pub avatar_new_z: LegacyDouble
}

impl PacketVariable for SlideObjectBundle {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        let (old_x, old_y, new_x, new_y, objects_z, avatar_movement_type) = packet.read();

        (Self {
            old_x, old_y, new_x, new_y, objects_z, avatar_movement_type,
            avatar_id: if vec![1, 2].contains(&avatar_movement_type) { packet.read() } else { LegacyId::default() },
            avatar_old_z: if vec![1, 2].contains(&avatar_movement_type) { packet.read() } else { LegacyDouble::default() },
            avatar_new_z: if vec![1, 2].contains(&avatar_movement_type) { packet.read() } else { LegacyDouble::default() }
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        if vec![1, 2].contains(&self.avatar_movement_type) {
            (
                self.old_x, self.old_y, self.new_x, self.new_y,
                self.objects_z.clone(), self.avatar_movement_type,
                self.avatar_id, self.avatar_old_z, self.avatar_new_z
            ).to_packet()
        } else {
            (
                self.old_x, self.old_y, self.new_x, self.new_y,
                self.objects_z.clone(), self.avatar_movement_type
            ).to_packet()
        }
    }
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct SpecialRoomEffect {
    pub effect_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct UserChange {
    pub id: LegacyId,
    pub figure: String,
    pub sex: String,
    pub custom_info: String,
    pub achievement_score: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct UserRemove {
    pub id: LegacyStringId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Users {
    pub users: Vec<User>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct UserUpdate {
    pub users: Vec<UserUpdateMessageData>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct WiredFurniMove {
    pub old_x: i32,
    pub old_y: i32,
    pub new_x: i32,
    pub new_y: i32,
    pub old_z: LegacyDouble,
    pub new_z: LegacyDouble,
    pub furni_id: LegacyId,
    pub animation_time: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct WiredUserMove {
    pub old_x: i32,
    pub old_y: i32,
    pub new_x: i32,
    pub new_y: i32,
    pub old_z: LegacyDouble,
    pub new_z: LegacyDouble,
    pub user_id: LegacyId,
    pub move_type: WiredUserMoveType,
    pub animation_time: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Open {
    pub stuff_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct WiredFurniAction {
    pub def: ActionDefinition
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct WiredFurniAddon {
    pub def: AddonDefinition
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct WiredFurniCondition {
    pub def: ConditionDefinition
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct WiredFurniTrigger {
    pub def: TriggerDefinition
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct WiredRewardResult {
    pub reason: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct WiredSaveSuccess {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct WiredValidationError {
    pub info: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CommunityGoalHallOfFame {
    pub goal_code: String,
    pub hof: Vec<HallOfFameEntryData>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CommunityGoalProgress {
    pub data: CommunityGoalData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ConcurrentUsersGoalProgress {
    pub state: i32,
    pub user_count: i32,
    pub user_count_goal: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct EpicPopup {
    pub image_uri: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct QuestCancelled {
    pub expired: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct QuestCompleted {
    pub quest_data: QuestMessageData,
    pub show_dialog: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct QuestDaily {
    pub quest: QuestMessageData,
    pub easy_quest_count: i32,
    pub hard_quest_count: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Quest {
    pub quest: QuestMessageData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Quests {
    pub quests: Vec<QuestMessageData>,
    pub open_window: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct SeasonalQuests {
    pub quests: Vec<QuestMessageData>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct TalentLevelUp {
    pub talent_track_name: String,
    pub level: i32,
    pub reward_perks: Vec<TalentTrackRewardPerk>,
    pub reward_products: Vec<TalentTrackRewardProduct>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct TalentTrackLevel {
    pub talent_track_name: String,
    pub level: i32,
    pub max_level: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct TalentTrack {
    pub name: String,
    pub levels: Vec<TalentTrackLevelData>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct AvailabilityStatus {
    pub is_open: bool,
    pub on_shut_down: bool,
    pub is_authentic_habbo: Option<bool>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct InfoHotelClosed {
    pub open_hour: i32,
    pub open_minute: i32,
    pub user_thrown_out_at_close: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct InfoHotelClosing {
    pub minutes_until_closing: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct LoginFailedHotelClosed {
    pub open_hour: i32,
    pub open_minute: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct MaintenanceStatus {
    pub is_in_maintenance: bool,
    pub minutes_until_maintenance: i32,
    pub duration: Option<bool>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CallForHelpDisabledNotify {
    pub info_url: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CallForHelpPendingCallsDeleted {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CallForHelpPendingCalls {
    pub calls: Vec<CallForHelpPendingCall>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CallForHelpReply {
    pub message: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CallForHelpResult {
    pub result_type: i32,
    pub message_text: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ChatReviewSessionDetached {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ChatReviewSessionOfferedToGuide {
    pub acceptance_timeout: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ChatReviewSessionResults {
    pub winning_vote_code: i32,
    pub own_vote_code: i32,
    pub final_status: Vec<i32>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ChatReviewSessionStarted {
    pub voting_timeout: i32,
    pub chat_record: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ChatReviewSessionVotingStatus {
    pub status: Vec<i32>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GuideOnDutyStatus {
    pub on_duty: bool,
    pub guides_on_duty: i32,
    pub helpers_on_duty: i32,
    pub guardians_on_duty: i32
}

#[derive(BaseParser, Clone, Debug, Default, PartialEq)]
pub struct GuideReportingStatus {
    pub status_code: i32,
    pub pending_ticket: Option<PendingGuideTicket>
}

impl PacketVariable for GuideReportingStatus {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        let status_code = packet.read();

        (Self {
            status_code,
            pending_ticket: if status_code == 1 { packet.read() } else { None }
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        if self.status_code == 1 && self.pending_ticket.is_some() {
            (self.status_code, self.pending_ticket.clone()).to_packet()
        } else if self.status_code == 1 {
            2i32.to_packet()
        } else {
            self.status_code.to_packet()
        }
    }
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GuideSessionAttached {
    pub as_guide: bool,
    pub help_request_type: i32,
    pub help_request_description: String,
    pub role_specific_wait_time: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GuideSessionDetached {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GuideSessionEnded {
    pub end_reason: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GuideSessionError {
    pub error_code: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GuideSessionInvitedToGuideRoom {
    pub room_id: LegacyId,
    pub room_name: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GuideSessionMessage {
    pub chat_message: String,
    pub sender_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GuideSessionPartnerIsTyping {
    pub is_typing: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GuideSessionRequesterRoom {
    pub requester_room_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GuideSessionStarted {
    pub requester_user_id: LegacyId,
    pub requester_name: String,
    pub requester_figure: String,
    pub guide_user_id: LegacyId,
    pub guide_name: String,
    pub guide_figure: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GuideTicketCreationResult {
    pub localization_code: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GuideTicketResolution {
    pub localization_code: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct IssueCloseNotification {
    pub close_reason: i32,
    pub message_text: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct QuizData {
    pub quiz_code: String,
    pub question_ids: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct QuizResults {
    pub quiz_code: String,
    pub question_ids_for_wrong_answers: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2FriendsLeaderboard {
    pub leaderboard: Vec<LeaderBoardEntry>,
    pub total_list_size: i32,
    pub game_type_id: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2TotalGroupLeaderboard {
    pub leaderboard: Vec<LeaderBoardEntry>,
    pub total_list_size: i32,
    pub game_type_id: i32,
    pub favourite_group_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2TotalLeaderboard {
    pub leaderboard: Vec<LeaderBoardEntry>,
    pub total_list_size: i32,
    pub game_type_id: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2WeeklyFriendsLeaderboard {
    pub year: i32,
    pub week: i32,
    pub max_offset: i32,
    pub current_offset: i32,
    pub minutes_until_reset: i32,
    pub leaderboard: Vec<LeaderBoardEntry>,
    pub total_list_size: i32,
    pub game_type_id: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2WeeklyGroupLeaderboard {
    pub year: i32,
    pub week: i32,
    pub max_offset: i32,
    pub current_offset: i32,
    pub minutes_until_reset: i32,
    pub favourite_group_id: LegacyId,
    pub leaderboard: Vec<LeaderBoardEntry>,
    pub total_list_size: i32,
    pub game_type_id: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2WeeklyLeaderboard {
    pub year: i32,
    pub week: i32,
    pub max_offset: i32,
    pub current_offset: i32,
    pub minutes_until_reset: i32,
    pub leaderboard: Vec<LeaderBoardEntry>,
    pub total_list_size: i32,
    pub game_type_id: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2AccountGameStatus {
    pub game_type_id: i32,
    pub free_games_left: i32,
    pub games_played_total: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2GameCancelled {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2GameCreated {
    pub game_lobby_data: GameLobbyData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2GameDirectoryStatus {
    pub status: i32,
    pub block_length: i32,
    pub games_played: i32,
    pub free_games_left: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2GameLongData {
    pub game_lobby_data: GameLobbyData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2GameStarted {
    pub game_lobby_data: GameLobbyData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2InArenaQueue {
    pub position: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2JoiningGameFailed {
    pub reason: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2StartCounter {
    pub count_down_length: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2StartingGameFailed {
    pub reason: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2StopCounter {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2UserBlocked {
    pub player_block_length: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2UserJoinedGame {
    pub user: GameLobbyPlayerData,
    pub was_team_switched: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2UserLeftGame {
    pub user_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct AcceptFriendResult {
    pub failures: Vec<AcceptFriendFailureData>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct FindFriendsProcessResult {
    pub success: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct FollowFriendFailed {
    pub error_code: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct FriendListFragment {
    pub total_fragments: i32,
    pub fragment_no: i32,
    pub friend_fragment: Vec<FriendData>
}

#[derive(BaseParser, Clone, Debug, Default, PartialEq)]
pub struct FriendListUpdate {
    pub cats: Vec<FriendCategoryData>,
    pub removed_friend_ids: Vec<LegacyId>,
    pub updated_friends: Vec<FriendData>,
    pub added_friends: Vec<FriendData>
}

impl PacketVariable for FriendListUpdate {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        let cats = packet.read();
        let count: LegacyLength = packet.read();
        let mut removed_friend_ids = Vec::new();
        let mut updated_friends = Vec::new();
        let mut added_friends = Vec::new();
        for _ in 0..*count {
            match packet.read::<i32>() {
                -1 => removed_friend_ids.push(packet.read()),
                0 => updated_friends.push(packet.read()),
                1 => added_friends.push(packet.read()),
                _ => {}
            }
        }

        (Self {
            cats, removed_friend_ids, updated_friends, added_friends
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        let mut packet = HPacket::from_header_id(0);

        packet.append(self.cats.clone());
        let size = self.removed_friend_ids.len() + self.updated_friends.len() + self.added_friends.len();
        packet.append(LegacyLength(size as i32));
        for id in self.removed_friend_ids.clone() {
            packet.append((-1i32, id));
        }
        for friend in self.updated_friends.clone() {
            packet.append((0i32, friend.clone()));
        }
        for friend in self.added_friends.clone() {
            packet.append((0i32, friend.clone()));
        }

        packet.get_bytes()[6..].to_vec()
    }
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct FriendNotification {
    pub avatar_id: String,
    pub type_code: i32,
    pub message: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct FriendRequests {
    pub total_req_count: i32,
    pub reqs: Vec<FriendRequestData>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct HabboSearchResult {
    pub friends: Vec<HabboSearchResultData>,
    pub others: Vec<HabboSearchResultData>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct InstantMessageError {
    pub error_code: i32,
    pub user_id: LegacyId,
    pub message: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct MessengerError {
    pub client_message_id: i32,
    pub error_code: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct MessengerInit {
    pub user_friend_limit: i32,
    pub normal_friend_limit: i32,
    pub extended_friend_limit: i32,
    pub categories: Vec<FriendCategoryData>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct MiniMailNewMessage {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct MiniMailUnreadCount {
    pub unread_message_count: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct NewConsole {
    pub sender_id: LegacyId,
    pub message_text: String,
    pub seconds_since_sent: i32,
    pub extra_data: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct NewFriendRequest {
    pub req: FriendRequestData
}

#[derive(BaseParser, Clone, Debug, Default, PartialEq)]
pub struct RoomInviteError {
    pub error_code: i32,
    pub failed_recipients: Vec<LegacyId>
}

impl PacketVariable for RoomInviteError {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        let error_code = packet.read();

        (Self {
            error_code,
            failed_recipients: if error_code == 1 { packet.read() } else { Vec::new() }
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        if self.error_code == 1 {
            (self.error_code, self.failed_recipients.clone()).to_packet()
        } else {
            self.error_code.to_packet()
        }
    }
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RoomInvite {
    pub sender_id: LegacyId,
    pub message_text: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ConfirmBreedingRequest {
    pub nest_id: LegacyId,
    pub pet_1: BreedingPetInfo,
    pub pet_2: BreedingPetInfo,
    pub rarity_categories: Vec<RarityCategoryData>,
    pub result_pet_type: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ConfirmBreedingResult {
    pub breeding_nest_stuff_id: LegacyId,
    pub result: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GoToBreedingNestFailure {
    pub reason: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct NestBreedingSuccess {
    pub pet_id: LegacyId,
    pub rarity_category: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PetAddedToInventory {
    pub pet: PetData,
    pub open_inventory: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PetBreeding {
    pub state: i32,
    pub own_pet_id: LegacyId,
    pub other_pet_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PetInventory {
    pub total_fragments: i32,
    pub fragment_no: i32,
    pub pet_list_fragment: Vec<PetData>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PetReceived {
    pub bought_as_gift: bool,
    pub pet: PetData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PetRemovedFromInventory {
    pub pet_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ChangeUserNameResult {
    pub result_code: i32,
    pub name: String,
    pub name_suggestions: Vec<String>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CheckUserNameResult {
    pub result_code: i32,
    pub name: String,
    pub name_suggestions: Vec<String>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct FigureUpdate {
    pub figure: String,
    pub gender: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Wardrobe {
    pub state: i32,
    pub outfits: Vec<OutfitData>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct BotAddedToInventory {
    pub item: BotData,
    pub open_inventory: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct BotInventory {
    pub items: Vec<BotData>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct BotRemovedFromInventory {
    pub item_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Achievement {
    pub achievement: AchievementData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Achievements {
    pub achievements: Vec<AchievementData>,
    pub default_category: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct AchievementsScore {
    pub score: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct LatencyPingResponse {
    pub request_id: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct JukeboxPlayListFull {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct JukeboxSongDisks {
    pub max_length: i32,
    pub song_disks: HashMap<i32, i32>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct NowPlaying {
    pub current_song_id: LegacyId,
    pub current_position: i32,
    pub next_song_id: LegacyId,
    pub next_position: i32,
    pub sync_count: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct OfficialSongId {
    pub official_song_id: String,
    pub song_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PlayList {
    pub synchronization_count: i32,
    pub play_list: Vec<PlayListEntry>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PlayListSongAdded {
    pub entry: PlayListEntry
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct TraxSongInfo {
    pub songs: Vec<SongInfoEntry>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct UserSongDisksInventory {
    pub song_disks: HashMap<i32, i32>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CreditVaultStatus {
    pub is_unlocked: bool,
    pub total_balance: i32,
    pub withdraw_balance: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct IncomeRewardClaimResponse {
    pub reward_category: i8,
    pub result: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct IncomeRewardStatus {
    pub data: Vec<IncomeReward>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct NavigatorCollapsedCategories {
    pub collapsed_categories: Vec<String>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct NavigatorLiftedRooms {
    pub lifted_rooms: Vec<LiftedRoomData>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct NavigatorMetaData {
    pub top_level_contexts: Vec<TopLevelContext>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct NavigatorSavedSearches {
    pub saved_searches: Vec<SavedSearch>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct NavigatorSearchResultBlocks {
    pub search_result: SearchResultSet
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct NewNavigatorPreferences {
    pub window_x: i32,
    pub window_y: i32,
    pub window_width: i32,
    pub window_height: i32,
    pub left_pane_hidden: bool,
    pub results_mode: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2ArenaEntered {
    pub player: Game2PlayerData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2EnterArenaFailed {
    pub reason: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2EnterArena {
    pub game_type: i32,
    pub field_type: i32,
    pub number_of_teams: i32,
    pub players: Vec<Game2PlayerData>,
    pub game_level: GameLevelData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2GameChatFromPlayer {
    pub user_id: LegacyId,
    pub chat_message: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2GameEnding {
    pub time_to_next_state: i32,
    pub game_result: Game2GameResult,
    pub teams: Vec<Game2TeamScoreData>,
    pub general_stats: Game2SnowWarGameStats
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2GameRejoin {
    pub room_before_game: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2PlayerExitedGameArena {
    pub user_id: LegacyId,
    pub player_game_object_id: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2PlayerRematches {
    pub user_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2StageEnding {
    pub time_to_next_state: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2StageLoad {
    pub game_type: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2StageRunning {
    pub time_to_stage_end: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2StageStarting {
    pub game_type: i32,
    pub room_type: String,
    pub count_down: i32,
    pub game_objects: GameObjectsData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2StageStillLoading {
    pub percentage: i32,
    pub finished_players: Vec<LegacyId>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct FigureSetIds {
    pub figure_set_ids: Vec<i32>,
    pub bound_furniture_names: Vec<String>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CameraPublishStatus {
    pub is_ok: bool,
    pub get_seconds_to_wait: i32,
    pub get_extra_data_id: Option<String>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CameraPurchaseOK {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CameraStorageUrl {
    pub url: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CompetitionStatus {
    pub is_ok: bool,
    pub get_error_reason: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct InitCamera {
    pub credit_price: i32,
    pub ducket_price: i32,
    pub publish_ducket_price: Option<i32>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ThumbnailStatus {
    pub is_ok: bool,
    pub is_render_limit_hit: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct FurniListAddOrUpdate {
    pub furni: FurniData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct FurniList {
    pub total_fragments: i32,
    pub fragment_no: i32,
    pub furni_fragment: Vec<FurniData>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct FurniListInvalidate {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct FurniListRemove {
    pub strip_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PostItPlaced {
    pub id: LegacyId,
    pub items_left: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct AvatarEffectActivated {
    pub effect_type: i32,
    pub duration: i32,
    pub is_permanent: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct AvatarEffectAdded {
    pub effect_type: i32,
    pub sub_type: i32,
    pub duration: i32,
    pub is_permanent: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct AvatarEffectExpired {
    pub effect_type: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct AvatarEffectSelected {
    pub effect_type: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct AvatarEffects {
    pub effects: Vec<AvatarEffectData>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct UserNftWardrobe {
    pub nft_avatars: Vec<NftWardrobeItem>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct UserNftWardrobeSelection {
    pub current_token_id: String,
    pub fallback_figure_string: String,
    pub fallback_gender: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct AvatarEffect {
    pub user_id: LegacyId,
    pub effect_id: i32,
    pub delay_milli_seconds: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CarryObject {
    pub user_id: LegacyId,
    pub item_type: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Dance {
    pub user_id: LegacyId,
    pub dance_style: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Expression {
    pub user_id: LegacyId,
    pub expression_type: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Sleep {
    pub user_id: LegacyId,
    pub sleeping: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct UseObject {
    pub user_id: LegacyId,
    pub item_type: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ActivityPoints {
    pub points: HashMap<i32, i32>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ClubGiftNotification {
    pub num_gifts: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ElementPointer {
    pub key: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct HabboAchievementNotification {
    pub data: AchievementLevelUpData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct HabboActivityPointNotification {
    pub amount: i32,
    pub change: i32,
    pub point_type: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct HabboBroadcast {
    pub message_text: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct InfoFeedEnable {
    pub enabled: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct MOTDNotification {
    pub messages: Vec<String>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct NotificationDialog {
    pub dialog_type: String,
    pub parameters: HashMap<String, String>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct OfferRewardDelivered {
    pub content_type: String,
    pub class_id: i32,
    pub name: String,
    pub description: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PetLevelNotification {
    pub pet_id: LegacyId,
    pub pet_name: String,
    pub level: i32,
    pub figure_data: PetFigureData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RestoreClient {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct UnseenItems {
    pub categories: HashMap<i32, Vec<i32>>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct AccountPreferences {
    pub ui_volume: i32,
    pub furni_volume: i32,
    pub trax_volume: i32,
    pub free_flow_chat_disabled: bool,
    pub room_invites_ignored: bool,
    pub room_camera_follow_disabled: bool,
    pub ui_flags: i32,
    pub prefered_chat_style: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct TradeOpenFailed {
    pub reason: i32,
    pub other_user_name: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct TradingAccept {
    pub user_id: LegacyId,
    pub user_accepts: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct TradingClose {
    pub user_id: LegacyId,
    pub reason: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct TradingCompleted {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct TradingConfirmation {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct TradingItemList {
    pub first_user_id: LegacyId,
    pub first_user_item_array: Vec<ItemDataStructure>,
    pub first_user_num_items: i32,
    pub first_user_num_credits: i32,
    pub second_user_id: LegacyId,
    pub second_user_item_array: Vec<ItemDataStructure>,
    pub second_user_num_items: i32,
    pub second_user_num_credits: i32,
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct TradingNotOpen {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct TradingOpen {
    pub user_id: LegacyId,
    pub user_can_trade: i32,
    pub other_user_id: LegacyId,
    pub other_user_can_trade: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct TradingOtherNotAllowed {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct TradingYouAreNotAllowed {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CitizenshipVipOfferPromoEnabled {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PerkAllowances {
    pub perks: Vec<Perk>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct BotError {
    pub error_code: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct BotForceOpenContextMenu {
    pub bot_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct BotSkillListUpdate {
    pub bot_id: LegacyId,
    pub skills: Vec<BotSkillData>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct FriendFurniCancelLock {
    pub stuff_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct FriendFurniOtherLockConfirmed {
    pub stuff_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct FriendFurniStartConfirmation {
    pub stuff_id: LegacyId,
    pub is_owner: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PromoArticles {
    pub articles: Vec<PromoArticleData>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CfhSanction {
    pub issue_id: LegacyId,
    pub account_id: LegacyId,
    pub sanction_type: CfhSanctionTypeData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CfhTopicsInit {
    pub call_for_help_categories: Vec<CallForHelpCategoryData>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct SanctionStatus {
    pub is_sanction_new: bool,
    pub is_sanction_active: bool,
    pub sanction_name: String,
    pub sanction_length_hours: i32,
    pub _unused1: i32,
    pub sanction_reason: String,
    pub sanction_creation_time: String,
    pub probation_hours_left: i32,
    pub next_sanction_name: String,
    pub next_sanction_length_hours: i32,
    pub _unused2: i32,
    pub has_custom_mute: bool,
    pub trade_lock_expiry_time: Option<String>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CreditBalance {
    pub balance: LegacyStringId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct BannedUsersFromRoom {
    pub room_id: LegacyId,
    pub banned_users: Vec<BannedUserData>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct FlatControllerAdded {
    pub flat_id: LegacyId,
    pub data: FlatControllerData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct FlatControllerRemoved {
    pub flat_id: LegacyId,
    pub user_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct FlatControllers {
    pub room_id: LegacyId,
    pub controllers: Vec<FlatControllerData>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct MuteAllInRoom {
    pub all_muted: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct NoSuchFlat {
    pub flat_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RoomSettingsData {
    pub room_id: LegacyId,
    pub name: String,
    pub description: String,
    pub door_mode: i32,
    pub category_id: i32,
    pub maximum_visitors: i32,
    pub maximum_visitors_limit: i32,
    pub tags: Vec<String>,
    pub trade_mode: i32,
    pub allow_pets: i32,
    pub allow_food_consume: i32,
    pub allow_walk_through: i32,
    pub hide_walls: i32,
    pub wall_thickness: i32,
    pub floor_thickness: i32,
    pub chat_settings: RoomChatSettings,
    pub allow_navigator_dynamic_cats: bool,
    pub room_moderation_settings: RoomModerationSettings
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RoomSettingsError {
    pub room_id: LegacyId,
    pub error_code: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RoomSettingsSaved {
    pub room_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct RoomSettingsSaveError {
    pub room_id: LegacyId,
    pub error_code: i32,
    pub info: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct ShowEnforceRoomCategoryDialog {
    pub selection_type: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct UserUnbannedFromRoom {
    pub room_id: LegacyId,
    pub user_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2FullGameStatus {
    pub full_status: FullGameStatusData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Game2GameStatus {
    pub status: GameStatusData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct AuthenticationOK {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CompleteDiffieHandshake {
    pub encrypted_public_key: String,
    pub server_client_encryption: Option<bool>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct DisconnectReason {
    pub reason: Option<i32>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct GenericError {
    pub error_code: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct IdentityAccounts {
    pub accounts: HashMap<i32, String>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct InitDiffieHandshake {
    pub encrypted_prime: String,
    pub encrypted_generator: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct IsFirstLoginOfDay {
    pub is_first_login_of_day: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct NoobnessLevel {
    pub noobness_level: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Ping {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct UniqueMachineID {
    pub machine_id: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct UserObject {
    pub id: LegacyId,
    pub name: String,
    pub figure: String,
    pub sex: String,
    pub custom_data: String,
    pub real_name: String,
    pub direct_mail: bool,
    pub respect_total: i32,
    pub respect_left: i32,
    pub pet_respect_left: i32,
    pub stream_publishing_allowed: bool,
    pub last_access_date: String,
    pub name_change_allowed: bool,
    pub account_safety_locked: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct UserRights {
    pub club_level: i32,
    pub security_level: i32,
    pub is_ambassador: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PetBreedingResult {
    pub result_data: PetBreedingResultData,
    pub other_result_data: PetBreedingResultData
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PetCommands {
    pub pet_id: LegacyId,
    pub all_commands: Vec<i32>,
    pub enabled_commands: Vec<i32>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PetExperience {
    pub pet_id: LegacyId,
    pub pet_room_index: i32,
    pub gained_experience: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PetFigureUpdate {
    pub room_index: i32,
    pub pet_id: LegacyId,
    pub figure_data: PetFigureData,
    pub has_saddle: bool,
    pub is_riding: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PetInfo {
    pub pet_id: LegacyId,
    pub name: String,
    pub level: i32,
    pub max_level: i32,
    pub experience: i32,
    pub experience_required_to_level: i32,
    pub energy: i32,
    pub max_energy: i32,
    pub nutrition: i32,
    pub max_nutrition: i32,
    pub respect: i32,
    pub owner_id: LegacyId,
    pub age: i32,
    pub owner_name: String,
    pub breed_id: i32,
    pub has_free_saddle: bool,
    pub is_riding: bool,
    pub skill_tresholds: Vec<i32>,
    pub access_rights: i32,
    pub can_breed: bool,
    pub can_harvest: bool,
    pub can_revive: bool,
    pub rarity_level: i32,
    pub max_well_being_seconds: i32,
    pub remaining_well_bein_seconds: i32,
    pub remaining_growing_seconds: i32,
    pub has_breeding_permission: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PetLevelUpdate {
    pub room_index: i32,
    pub pet_id: LegacyId,
    pub level: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PetPlacingError {
    pub error_code: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PetRespectFailed {
    pub required_days: i32,
    pub avatar_age_in_days: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct PetStatusUpdate {
    pub room_index: i32,
    pub pet_id: LegacyId,
    pub can_breed: bool,
    pub can_harvest: bool,
    pub can_revive: bool,
    pub has_breeding_permission: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct BadgePointLimits {
    pub data: HashMap<String, Vec<BadgeAndPointLimit>>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct BadgeReceived {
    pub badge_id: i32,
    pub badge_code: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct Badges {
    pub total_fragments: i32,
    pub fragment_no: i32,
    pub current_fragment: HashMap<i32, String>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct IsBadgeRequestFulfilled {
    pub request_code: String,
    pub fulfilled: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct NewUserExperienceGiftOffer {
    pub gift_options: Vec<NewUserExperienceGiftOptions>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct NewUserExperienceNotComplete {}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct MarketplaceBuyOfferResult {
    pub result: i32,
    pub offer_id: LegacyId,
    pub new_price: i32,
    pub requested_offer_id: LegacyId
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct MarketplaceCancelOfferResult {
    pub offer_id: LegacyId,
    pub success: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct MarketplaceCanMakeOfferResult {
    pub result_code: i32,
    pub token_count: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct MarketplaceConfiguration {
    pub is_enabled: bool,
    pub commission: i32,
    pub token_batch_price: i32,
    pub token_batch_size: i32,
    pub offer_min_price: i32,
    pub offer_max_price: i32,
    pub expiration_hours: i32,
    pub average_price_period: i32,
    pub selling_fee_percentage: i32,
    pub revenue_limit: i32,
    pub half_tax_limit: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct MarketplaceItemStats {
    pub average_price: i32,
    pub offer_count: i32,
    pub history_length: i32,
    pub data: Vec<MarketplaceItemStatsData>,
    pub furni_category_id: i32,
    pub furni_type_id: i32
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct MarketplaceMakeOfferResult {
    pub result: i32
}

#[derive(BaseParser, Clone, Debug, Default, PartialEq)]
pub struct MarketPlaceOffers {
    pub offers: Vec<MarketPlaceOffer>,
    pub total_items_found: i32
}

impl PacketVariable for MarketPlaceOffers {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        let mut offers: Vec<MarketPlaceOffer> = Vec::new();
        for _ in 0..*packet.read::<LegacyLength>() {
            let mut offer: MarketPlaceOffer = packet.read();
            offer.offer_count = packet.read();
            offers.push(offer);
        }
        let total_items_found = packet.read();

        (Self {
            offers, total_items_found
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        let mut packet = HPacket::from_header_id(0);

        for offer in self.offers.clone() {
            packet.append((offer.clone(), offer.offer_count));
        }
        packet.append(self.total_items_found);

        packet.get_bytes()[6..].to_vec()
    }
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct MarketPlaceOwnOffers {
    pub credits_waiting: i32,
    pub offers: Vec<MarketPlaceOffer>
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct CommunityVoteReceived {
    pub acknowledged: bool
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct UserClassification {
    pub classified_users: Vec<ClassifiedUser>
}
