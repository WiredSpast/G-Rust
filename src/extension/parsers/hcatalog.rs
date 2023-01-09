use crate::protocol::hpacket::HPacket;
use crate::protocol::vars::legacy::LegacyId;
use crate::protocol::vars::packetvariable::PacketVariable;
use super::baseparser::BaseParser;

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct HCatalogIndex {
    root: HCatalogPageIndex,
    new_additions_available: bool,
    catalog_type: String
}

#[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct HCatalogPageIndex {
    visible: bool,
    icon: i32,
    page_id: i32, // Might be a LegacyId
    page_name: String,
    localization: String,
    offer_ids: Vec<i32>,
    children: Vec<HCatalogPageIndex>
}

// #[derive(BaseParser, Clone, Debug, Default, PacketVariable, PartialEq)]
// pub struct HCatalogPage {
//     page_id: i32, // Might be a LegacyId
//     catalog_type: String,
//     layout_code: String,
//     images: Vec<String>,
//     texts: Vec<String>,
//     offers: Vec<HOffer>,
//     offer_id: i32, // Might be a LegacyId
//     accept_season_currency_as_credits: bool,
//     front_page_items: Option<Vec<HFrontPageItem>>
// }