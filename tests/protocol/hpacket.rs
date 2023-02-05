use g_rust::protocol::hdirection::HDirection;
use g_rust::protocol::hpacket::HPacket;

#[test]
fn from_bytes() {
    let mut packet = HPacket::from_bytes(vec![1, 2, 3, 4, 0, 152]);
    assert_eq!(vec![0, 0, 0, 2, 0, 152], packet.get_bytes());
    assert_eq!(6, packet.bytes_length());
    assert_eq!(2, packet.length());
    assert!(!packet.is_corrupted());
    assert_eq!("", packet.identifier);
    assert_eq!(HDirection::None, packet.identifier_direction);
    assert!(packet.is_complete());
    assert_eq!(6, packet.read_index);
    assert_eq!(1, packet.eof());
    assert_eq!(152, packet.header_id());
}

#[test]
fn read() {
    let mut packet = HPacket::from_bytes(vec![0, 0, 0, 81, 0, 152, 1, 1, 1, 0, 1, 2, 1, 2, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 1, 2, 3, 4, 1, 2, 3, 4, 5, 6, 7, 8, 0, 3, 'a' as u8, 'b' as u8, 'c' as u8]);
    let a: u8 = packet.read();
    assert_eq!(1, a);
    let b: i8 = packet.read();
    assert_eq!(1, b);
    let c: bool = packet.read();
    assert_eq!(true, c);
    let d: bool = packet.read();
    assert_eq!(false, d);
    let e: u16 = packet.read();
    assert_eq!(258, e);
    let f: i16 = packet.read();
    assert_eq!(258, f);
    let g: u32 = packet.read();
    assert_eq!(16909060, g);
    let h: i32 = packet.read();
    assert_eq!(16909060, h);
    let i: u64 = packet.read();
    assert_eq!(72623859790382856, i);
    let j: i64 = packet.read();
    assert_eq!(72623859790382856, j);
    let k: u128 = packet.read();
    assert_eq!(1339673755198158349044581307228491536, k);
    let l: i128 = packet.read();
    assert_eq!(1339673755198158349044581307228491536, l);
    let m: f32 = packet.read();
    assert_eq!(0.000000000000000000000000000000000000023879393, m);
    let n: f64 = packet.read();
    assert_eq!(0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000820788039913184, n);
    let o: String = packet.read();
    assert_eq!("abc", o);
}

#[test]
fn read_tuple() {
    let mut packet = HPacket::from_bytes(vec![0, 0, 0, 44, 0, 152, 1, 1, 1, 0, 1, 2, 1, 2, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 1, 2, 3, 4, 1, 2, 3, 4, 5, 6, 7, 8, 0, 3, 'a' as u8, 'b' as u8, 'c' as u8]);
    let res: (u8, i8, bool, bool, u16, i16, u32, i32, u64, i64, u128, i128, f32, f64, String) = packet.read();

    assert_eq!(1, res.0);
    assert_eq!(1, res.1);
    assert_eq!(true, res.2);
    assert_eq!(false, res.3);
    assert_eq!(258, res.4);
    assert_eq!(258, res.5);
    assert_eq!(16909060, res.6);
    assert_eq!(16909060, res.7);
    assert_eq!(72623859790382856, res.8);
    assert_eq!(72623859790382856, res.9);
    assert_eq!(1339673755198158349044581307228491536, res.10);
    assert_eq!(1339673755198158349044581307228491536, res.11);
    assert_eq!(0.000000000000000000000000000000000000023879393, res.12);
    assert_eq!(0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000820788039913184, res.13);
    assert_eq!("abc", res.14);
}

#[test]
fn packet_read_vec() {
    let mut packet = HPacket::from_bytes(vec![0, 0, 0, 81, 0, 152, 0, 0, 0, 10, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 1, 0]);
    let res: Vec<u32> = packet.read();

    assert_eq!(vec![1, 256, 257, 65536, 65537, 65792, 65793, 16777216, 16777217, 16777472], res);
}