use deku::{DekuRead, DekuWrite};

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct Response {
    pub message_size: i32,
    pub correlation_id: i32,
    pub error_code: i16,
}
