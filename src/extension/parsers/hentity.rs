// use crate::protocol::hpacket::HPacket;
//
// #[derive(Debug, Clone, Default<User>)]
// pub enum HEntity {
//     User {
//         id: i32,
//         name: String,
//         motto: String,
//         figure_id: String,
//         index: i32,
//         tile: HPoint,
//         body_facing: i32,
//         head_facing: i32,
//         entity_type: i32,
//         gender: String,
//         stuff: (i32, i32, String, i32, bool),
//         favorite_group: String
//     },
//     Pet {
//         id: i32,
//         name: String,
//         motto: String,
//         figure_id: String,
//         index: i32,
//         tile: HPoint,
//         body_facing: i32,
//         head_facing: i32,
//         entity_type: i32,
//         stuff: (i32, i32, String, i32, bool, bool, bool, bool, bool, bool, i32, String)
//     },
//     Bot {
//         id: i32,
//         name: String,
//         motto: String,
//         figure_id: String,
//         index: i32,
//         tile: HPoint,
//         body_facing: i32,
//         head_facing: i32,
//         entity_type: i32,
//         stuff: (String, i32, String, Vec<i16>)
//     },
//     OldBot {
//         id: i32,
//         name: String,
//         motto: String,
//         figure_id: String,
//         index: i32,
//         tile: HPoint,
//         body_facing: i32,
//         head_facing: i32,
//         entity_type: i32
//     }
// }
//
// impl HEntity::User {
//     pub fn default() -> HEntity::User {
//         HEntity::User {
//             id: 0,
//             name: "".to_string(),
//             motto: "".to_string(),
//             figure_id: "".to_string(),
//             index: 0,
//             tile: Default::default(),
//             body_facing: 0,
//             head_facing: 0,
//             entity_type: 0,
//             gender: "".to_string(),
//             stuff: (0, 0, "".to_string(), 0, false),
//             favorite_group: "".to_string()
//         }
//     }
// }
//
// impl HEntity::User {
//     pub fn default() -> HEntity::User {
//         HEntity::User {
//             id: 0,
//             name: "".to_string(),
//             motto: "".to_string(),
//             figure_id: "".to_string(),
//             index: 0,
//             tile: Default::default(),
//             body_facing: 0,
//             head_facing: 0,
//             entity_type: 0,
//             gender: "".to_string(),
//             stuff: (0, 0, "".to_string(), 0, false),
//             favorite_group: "".to_string()
//         }
//     }
// }
//
// impl HEntity {
//     pub fn new(mut packet: HPacket) -> HEntity {
//         let id: i32 = packet.read(None).unwrap();
//         let name = packet.read_string(None);
//         let motto = packet.read_string(None);
//         let figure_id = packet.read_string(None);
//         let index: i32 = packet.read(None).unwrap();
//         let tile = HPoint {
//             x: packet.read(None).unwrap(),
//             y: packet.read(None).unwrap(),
//             z: packet.read_string(None).parse::<f32>().unwrap()
//         };
//         let body_facing: i32 = packet.read(None).unwrap();
//         let entity_type: i32 = packet.read(None).unwrap();
//
//         if entity_type == 1 {
//             let gender = packet.read_string(None);
//             // let stuff = (0, 0, )
//             HEntity::OldBot {
//                 id, name, motto, figure_id, index, tile, body_facing, head_facing: body_facing, entity_type
//             }
//         } else {
//             HEntity::OldBot {
//                 id, name, motto, figure_id, index, tile, body_facing, head_facing: body_facing, entity_type
//             }
//         }
//
//     }
//
//     pub fn parse(packet: HPacket) -> Vec<HEntity> {
//         let res: Vec<HEntity> = Vec::new();
//
//
//
//         res
//     }
//
//
// }