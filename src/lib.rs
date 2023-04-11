#[macro_use]
extern crate packetvar_derive;
#[macro_use]
extern crate parser_derive;

pub mod protocol;
pub mod extension;
pub mod misc;
pub mod services;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::protocol::vars::legacy::{LegacyId, LegacyLength, LegacyStringId};
    use crate::protocol::hpacket::HPacket;
    use crate::protocol::vars::longstring::LongString;
    use crate::extension::parsers::baseparser::BaseParser;

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
    fn read_array() {
        let mut packet = HPacket::from_bytes(vec![0, 0, 0, 14, 0, 0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3]);
        let nums: [i32; 3] = packet.read();
        println!("{nums:?}");
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
