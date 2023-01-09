#[macro_use]
extern crate packetvar_derive;
#[macro_use]
extern crate parser_derive;

pub mod protocol;
pub mod extension;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::extension::parsers::hwallitem::{HWallItem, HWallItems};
    use crate::protocol::vars::legacy::{LegacyId, LegacyLength, LegacyStringId};
    use crate::protocol::hpacket::HPacket;
    use crate::protocol::vars::longstring::LongString;
    use crate::extension::parsers::hfriend::HFriend;
    use crate::extension::parsers::baseparser::BaseParser;

    #[test]
    fn packet_read_implicit() {
        let mut packet = HPacket::from_bytes(vec![0, 0, 0, 81, 0, 152, 1, 1, 1, 0, 1, 2, 1, 2, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 1, 2, 3, 4, 1, 2, 3, 4, 5, 6, 7, 8, 0, 3, 'a' as u8, 'b' as u8, 'c' as u8, 123]);
        let a: u8 = packet.read();
        let b: i8 = packet.read();
        let c: bool = packet.read();
        let d: bool = packet.read();
        let e: u16 = packet.read();
        let f: i16 = packet.read();
        let g: u32 = packet.read();
        let h: i32 = packet.read();
        let i: u64 = packet.read();
        let j: i64 = packet.read();
        let k: u128 = packet.read();
        let l: i128 = packet.read();
        let m: f32 = packet.read();
        let n: f64 = packet.read();
        let o: String = packet.read();
        println!("{a}, {b}, {c}, {d}, {e}, {f}, {g}, {h}, {i}, {j}, {k}, {l}, {m}, {n}, {o}");
        println!("{}", packet.read::<u8>());
    }

    #[test]
    fn packet_append() {
        let mut packet = HPacket::from_header_id(0);
        println!("{packet:?}");
        packet.append(260);
        println!("{packet:?}");
        packet.append(String::from("abc"));
        println!("{packet:?}");
        packet.append(true);
        println!("{packet:?}");
        packet.append(false);
        println!("{packet:?}");
        packet.append(5 as u8);
        println!("{packet:?}");
    }

    #[test]
    fn packet_replace() {
        let mut packet = HPacket::from_bytes(vec![0, 0, 0, 11, 0, 152, 0, 3, 'a' as u8, 'b' as u8, 'c' as u8, 1, 2, 3, 4]);
        println!("{packet:?}");
        packet.replace(6, String::from("defghi"));
        println!("{packet:?}");
    }

    #[test]
    fn read_bytes() {
        let mut packet = HPacket::from_bytes(vec![0, 0, 0, 11, 0, 152, 0, 3, 'a' as u8, 'b' as u8, 'c' as u8, 1, 2, 3, 4]);
        println!("{packet:?}");
        println!("{:?}", packet.read_bytes(5));
        println!("{:?}", packet.read_bytes(3));
        println!("{:?}", packet.read_bytes_at(5, 6));

    }

    #[test]
    fn legacy_int() {
        let mut a = LegacyId(9);
        a += LegacyId(5);
        a += 2i32;
        let b = *a;
        println!("{b}");
    }

    #[test]
    fn read_legacy_int() {
        let mut packet = HPacket::from_bytes(vec![0, 0, 0, 11, 0, 11, 0, 0, 0, 5, 0, 0, 1, 1]);
        let a: LegacyId = packet.read();
        println!("{a}");
    }

    #[test]
    fn read_legacy_length() {
        let mut packet = HPacket::from_bytes(vec![0, 0, 0, 11, 0, 11, 0, 1, 0, 5, 0, 0, 1, 1]);
        let a: LegacyLength = packet.read();
        println!("{a}");
    }

    #[test]
    fn long_string() {
        let mut packet = HPacket::from_bytes(vec![0, 0, 0, 11, 0, 11, 0, 0, 0, 5, 'a' as u8, 'b' as u8, 'c' as u8, 'd' as u8, 'e' as u8]);
        let a: LongString = packet.read();
        println!("{a}");
        packet.append(LongString::from("abcdefghijklmnopqrstuvwxyz"));
        println!("{packet:?}");
        let b: LongString = packet.read();
        println!("{b}");
    }

    #[test]
    fn hwallitem_append() {
        let mut packet = HPacket::from_bytes(vec![0, 0, 0, 2, 0, 85]);
        packet.append(HWallItem {
            id: LegacyStringId(12534),
            type_id: 254,
            location: ":w=4,9 l=30,57 l".to_string(),
            state: "".to_string(),
            seconds_to_expiration: 5,
            usage_policy: 3,
            owner_id: LegacyId(12435)
        });
        println!("{packet:?}");
    }

    #[test]
    fn hwallitem_read() {
        let mut packet = HPacket::from_bytes(vec![0, 0, 0, 45, 0, 85, 0, 5, 49, 50, 53, 51, 52, 0, 0, 0, 254, 0, 16, 58, 119, 61, 52, 44, 57, 32, 108, 61, 51, 48, 44, 53, 55, 32, 108, 0, 0, 0, 0, 0, 5, 0, 0, 0, 3, 0, 0, 48, 147]);
        let item: HWallItem = packet.read();
        println!("{item:?}");
        println!("{} / {}", packet.read_index, packet.bytes_length());
    }

    #[test]
    fn hwallitem_parse() {
        let mut packet = HPacket::from_bytes(vec![0, 0, 0, 53, 0, 85, 0, 0, 0, 0, 0, 0, 0, 1, 0, 5, 49, 50, 53, 51, 52, 0, 0, 0, 254, 0, 16, 58, 119, 61, 52, 44, 57, 32, 108, 61, 51, 48, 44, 53, 55, 32, 108, 0, 0, 0, 0, 0, 5, 0, 0, 0, 3, 0, 0, 48, 147]);
        let HWallItems{owners, items} = packet.read();
        println!("{items:?}, {owners:?}");
    }

    #[test]
    fn hwallitem_append_items() {
        let mut packet = HPacket::from_header_id(8);
        let items = vec![
            HWallItem {
                id: LegacyStringId(12534),
                type_id: 254,
                location: ":w=4,9 l=30,57 l".to_string(),
                state: "".to_string(),
                seconds_to_expiration: 5,
                usage_policy: 3,
                owner_id: LegacyId(12435)
            },
            HWallItem {
                id: LegacyStringId(45243),
                type_id: 524,
                location: ":w=9,4 l=15,74 r".to_string(),
                state: "1".to_string(),
                seconds_to_expiration: -1,
                usage_policy: 0,
                owner_id: LegacyId(54321)
            }
        ];
        let owners: HashMap<LegacyId, String> = HashMap::from([
            (LegacyId(12435), "WiredSpast".to_string()),
            (LegacyId(54321), "Kouris".to_string())
        ]);

        packet.append((owners, items));
        println!("{packet:?}");
    }

    #[test]
    fn hwallitem_read_items() {
        let mut packet = HPacket::from_bytes(vec![0, 0, 0, 125, 0, 8, 0, 0, 0, 2, 0, 0, 212, 49, 0, 6, 75, 111, 117, 114, 105, 115, 0, 0, 48, 147, 0, 10, 87, 105, 114, 101, 100, 83, 112, 97, 115, 116, 0, 0, 0, 2, 0, 5, 49, 50, 53, 51, 52, 0, 0, 0, 254, 0, 16, 58, 119, 61, 52, 44, 57, 32, 108, 61, 51, 48, 44, 53, 55, 32, 108, 0, 0, 0, 0, 0, 5, 0, 0, 0, 3, 0, 0, 48, 147, 0, 5, 52, 53, 50, 52, 51, 0, 0, 2, 12, 0, 16, 58, 119, 61, 57, 44, 52, 32, 108, 61, 49, 53, 44, 55, 52, 32, 114, 0, 1, 49, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 212, 49]);
        let (owners, items, a): (HashMap<LegacyId, String>, Vec<HWallItem>, i32) = packet.read();
        println!("{owners:?}, {items:?}");
        println!("{a}");
    }

    #[test]
    fn read_array() {
        let mut packet = HPacket::from_bytes(vec![0, 0, 0, 14, 0, 0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3]);
        let nums: [i32; 3] = packet.read();
        println!("{nums:?}");
    }

    #[test]
    fn derive_packet_variable_struct() {
        let mut packet = HPacket::from_bytes(vec![0, 0, 0, 14, 0, 0, 0, 0, 0, 1, 0, 2, 'a' as u8, 'b' as u8, 0, 0, 0, 2, 1, 0, 0, 2, 'f' as u8, 'g' as u8, 0, 0, 0, 5, 0, 3, 'm' as u8, 't' as u8, 't' as u8, 0, 4, 'n' as u8, 'a' as u8, 'm' as u8, 'e' as u8, 0, 2, 'f' as u8, 'b' as u8, 1, 0, 1, 0, 5]);
        let a: HFriend = packet.read();
        println!("{a:?}");
        let mut reconstructed_packet = HPacket::from_header_id(0);
        reconstructed_packet.append(a);
        println!("{reconstructed_packet:?}");
    }

    #[test]
    fn base_parser() {
        let mut packet = HPacket::from_bytes(vec![0, 0, 0, 45, 0, 85, 0, 5, 49, 50, 53, 51, 52, 0, 0, 0, 254, 0, 16, 58, 119, 61, 52, 44, 57, 32, 108, 61, 51, 48, 44, 53, 55, 32, 108, 0, 0, 0, 0, 0, 5, 0, 0, 0, 3, 0, 0, 48, 147]);
        let a = HWallItem::parse(&mut packet);
        println!("{a:?}");
    }

    #[test]
    fn packet_stringify() {
        let mut packet = HPacket::from_header_id(5);
        packet.append((
            200u8,
            1235i32,
            -321i16,
            122u16,
            1562233456i64,
            53.652f64,
            69.42042042042420420f32,
            true,
            "abcdefghijklmnopqrstuvwxyz".to_string()
        ));
        println!("{packet:?}");
        println!("{}", packet.bytes_length());
        println!("{}", packet.stringify());
        let recon_packet = HPacket::from_string(packet.stringify());
        println!("{recon_packet:?}");
    }

    #[test]
    fn read_option() {
        let mut packet = HPacket::from_bytes(vec![0, 0, 0, 6, 0, 5, 0, 0, 0, 1]);
        let a: Option<i32> = packet.read();
        let b: Option<i32> = packet.read();
        println!("{a:?}, {b:?}");
    }
}
