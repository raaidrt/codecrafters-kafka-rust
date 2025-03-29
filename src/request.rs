use deku::prelude::*;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct Request {
    pub message_size: i32,
    pub request_api_key: i16,
    pub request_api_version: i16,
    pub correlation_id: i32,
}
