use crate::extension::extension::Extension;
use crate::extension::parsers::baseparser::BaseParser;
use crate::protocol::hmessage::HMessage;
use crate::protocol::hpacket::HPacket;

// pub(crate) enum InterceptListener<T: BaseParser + ?Sized> {
//     Raw(fn(ext: &mut Extension, msg: &mut HMessage)),
//     Parsed(fn(ext: &mut Extension, msg: &mut HMessage, object: &mut T))
// }
//
// impl <T: BaseParser> InterceptListener<T> {
//     pub(crate) fn intercept(&self, ext: &mut Extension, msg: &mut HMessage) {
//         match self {
//             Self::Raw(listener) => (listener)(ext, msg),
//             Self::Parsed(listener) => {
//                 let mut original_packet = msg.get_packet().clone();
//                 let mut object: T = original_packet.read();
//                 (listener)(ext, msg, &mut object);
//                 let mut new_packet = HPacket::from_header_id(original_packet.header_id());
//                 new_packet.append(object);
//                 if original_packet.get_bytes() != new_packet.get_bytes() {
//                     msg.get_packet().replace(6, object);
//                 }
//             }
//         };
//     }
// }