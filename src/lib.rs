pub mod protocol;
pub mod extension;

#[cfg(test)]
mod tests {
    use super::protocol::hpacket::HPacket;

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
}
