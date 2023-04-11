use std::collections::HashMap;
use crate::extension::parsers::stuffdata::StuffData::{CrackableStuffData, EmptyStuffData, HighScoreStuffData, IntArrayStuffData, LegacyStuffData, MapStuffData, StringArrayStuffData, VoteResultStuffData};
use crate::protocol::hpacket::HPacket;
use crate::protocol::vars::packetvariable::PacketVariable;

#[derive(Clone, Debug, PartialEq)]
pub enum StuffData {
    LegacyStuffData {
        legacy_string: String,
        unique_serial_data: Option<UniqueSerialData>
    },
    MapStuffData {
        map: HashMap<String, String>,
        unique_serial_data: Option<UniqueSerialData>
    },
    StringArrayStuffData {
        values: Vec<String>,
        unique_serial_data: Option<UniqueSerialData>
    },
    VoteResultStuffData {
        legacy_string: String,
        result: i32,
        unique_serial_data: Option<UniqueSerialData>
    },
    EmptyStuffData  {
        unique_serial_data: Option<UniqueSerialData>
    },
    IntArrayStuffData {
        values: Vec<i32>,
        unique_serial_data: Option<UniqueSerialData>
    },
    HighScoreStuffData {
        legacy_string: String,
        score_type: i32,
        clear_type: i32,
        entries: Vec<HighScoreData>
    },
    CrackableStuffData {
        legacy_string: String,
        hits: i32,
        target: i32,
        unique_serial_data: Option<UniqueSerialData>
    }
}

impl StuffData {
    pub fn get_unique_serial_data(&self) -> Option<UniqueSerialData> {
        match self {
            LegacyStuffData { unique_serial_data, .. } => unique_serial_data.clone(),
            MapStuffData { unique_serial_data, .. } => unique_serial_data.clone(),
            StringArrayStuffData { unique_serial_data, .. } => unique_serial_data.clone(),
            VoteResultStuffData { unique_serial_data, .. } => unique_serial_data.clone(),
            EmptyStuffData { unique_serial_data, .. } => unique_serial_data.clone(),
            IntArrayStuffData { unique_serial_data, .. } => unique_serial_data.clone(),
            HighScoreStuffData { .. } => None,
            CrackableStuffData { unique_serial_data, .. } => unique_serial_data.clone()
        }
    }
}

impl Default for StuffData {
    fn default() -> Self {
        EmptyStuffData {
            unique_serial_data: None
        }
    }
}

impl PacketVariable for StuffData {
    fn from_packet(bytes: Vec<u8>) -> (Self, usize) where Self: Sized {
        let mut packet = HPacket::from_header_id_and_bytes(0, bytes);

        let cat: i32 = packet.read();

        (match cat & 255 {
            0 => LegacyStuffData {
                legacy_string: packet.read(),
                unique_serial_data: if cat & 256 > 0 { packet.read() } else { None }
            },
            1 => MapStuffData {
                map: packet.read(),
                unique_serial_data: if cat & 256 > 0 { packet.read() } else { None }
            },
            2 => StringArrayStuffData {
                values: packet.read(),
                unique_serial_data: if cat & 256 > 0 { packet.read() } else { None }
            },
            3 => VoteResultStuffData {
                legacy_string: packet.read(),
                result: packet.read(),
                unique_serial_data: if cat & 256 > 0 { packet.read() } else { None }
            },
            4 => EmptyStuffData {
                unique_serial_data: if cat & 256 > 0 { packet.read() } else { None }
            },
            5 => IntArrayStuffData {
                values: packet.read(),
                unique_serial_data: if cat & 256 > 0 { packet.read() } else { None }
            },
            6 => HighScoreStuffData {
                legacy_string: packet.read(),
                score_type: packet.read(),
                clear_type: packet.read(),
                entries: packet.read()
            },
            7 => CrackableStuffData {
                legacy_string: packet.read(),
                hits: packet.read(),
                target: packet.read(),
                unique_serial_data: if cat & 256 > 0 { packet.read() } else { None }
            },
            _ => panic!("Unknown stuff data type")
        }, packet.read_index - 6)
    }

    fn to_packet(&self) -> Vec<u8> {
        match self {
            LegacyStuffData {
                legacy_string, unique_serial_data
            } => (
                0i32 | if unique_serial_data.is_some() { 256i32 } else { 0i32 },
                legacy_string.clone(), unique_serial_data.clone()
            ).to_packet(),
            MapStuffData {
                map, unique_serial_data
            } => (
                1i32 | if unique_serial_data.is_some() { 256i32 } else { 0i32 },
                map.clone(), unique_serial_data.clone()
            ).to_packet(),
            StringArrayStuffData {
                values, unique_serial_data
            } => (
                2i32 | if unique_serial_data.is_some() { 256i32 } else { 0i32 },
                values.clone(), unique_serial_data.clone()
            ).to_packet(),
            VoteResultStuffData {
                legacy_string, result, unique_serial_data
            } => (
                3i32 | if unique_serial_data.is_some() { 256i32 } else { 0i32 },
                legacy_string.clone(), *result, unique_serial_data.clone()
            ).to_packet(),
            EmptyStuffData {
                unique_serial_data
            } => (
                4i32 | if unique_serial_data.is_some() { 256i32 } else { 0i32 },
                unique_serial_data.clone()
            ).to_packet(),
            IntArrayStuffData {
                values, unique_serial_data
            } => (
                5i32 | if unique_serial_data.is_some() { 256i32 } else { 0i32 },
                values.clone(), unique_serial_data.clone()
            ).to_packet(),
            HighScoreStuffData {
                legacy_string, score_type, clear_type, entries
            } => (
                6i32,
                legacy_string.clone(), *score_type, *clear_type, entries.clone()
            ).to_packet(),
            CrackableStuffData {
                legacy_string, hits, target, unique_serial_data
            } => (
                7i32 | if unique_serial_data.is_some() { 256i32 } else { 0i32 },
                legacy_string.clone(), *hits, *target, unique_serial_data.clone()
            ).to_packet()
        }
    }
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct UniqueSerialData {
    pub number: i32,
    pub size: i32
}

#[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
pub struct HighScoreData {
    pub score: i32,
    pub users: Vec<String>
}

// #[derive(Clone, Debug, Default, PacketVariable, PartialEq)]
// pub struct StuffData {}