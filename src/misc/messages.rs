pub(crate) struct IncomingMessageIds;
impl IncomingMessageIds {
    pub const ON_DOUBLE_CLICK: i16 = 1;
    pub const INFO_REQUEST: i16 = 2;
    pub const PACKET_INTERCEPT: i16 = 3;
    pub const FLAGS_CHECK: i16 = 4;
    pub const CONNECTION_START: i16 = 5;
    pub const CONNECTION_END: i16 = 6;
    pub const INIT: i16 = 7;
    pub const UPDATE_HOST_INFO: i16 = 10;
    pub const PACKET_TO_STRING_RESPONSE: i16 = 20;
    pub const STRING_TO_PACKET_RESPONSE: i16 = 21;
}

pub(crate) struct OutgoingMessageIds;
impl OutgoingMessageIds {
    pub const EXTENSION_INFO: i16 = 1;
    pub const MANIPULATED_PACKET: i16 = 2;
    pub const REQUEST_FLAGS: i16 = 3;
    pub const SEND_MESSAGE: i16 = 4;
    pub const PACKET_TO_STRING_REQUEST: i16 = 20;
    pub const STRING_TO_PACKET_REQUEST: i16 = 21;
    pub const EXTENSION_CONSOLE_LOG: i16 = 98;
}