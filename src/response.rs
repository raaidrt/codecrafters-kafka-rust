use crate::size::Size;
use deku::{DekuRead, DekuWrite};

#[derive(Debug, PartialEq, DekuWrite, DekuRead)]
pub struct Response {
    #[deku(endian = "big")]
    message_size: i32,
    pub header: Header,
    #[deku(ctx = "*header")]
    pub body: Body,
}

#[derive(Debug, PartialEq, DekuWrite, DekuRead)]
#[deku(ctx = "header: Header", id = "header.correlation_id")]
pub enum Body {
    #[deku(id = 18)]
    ApiVersions(ApiVersionsBody),
}

impl Size for Body {
    fn size(&self) -> i32 {
        match self {
            Self::ApiVersions(body) => body.size(),
        }
    }
}

impl Size for ApiVersionsBody {
    fn size(&self) -> i32 {
        2 + 4
            + self
                .api_keys
                .iter()
                .map(ApiVersion::size)
                .fold(0, |a, b| a + b)
            + 4
    }
}

impl ApiVersionsBody {
    pub fn new(
        error_code: i16,
        api_keys: Vec<ApiVersion>,
        throttle_time_ms: i32,
    ) -> ApiVersionsBody {
        ApiVersionsBody {
            error_code,
            length: api_keys.len() as i32,
            api_keys,
            throttle_time_ms,
        }
    }
}

impl Response {
    pub fn new(header: Header, body: Body) -> Response {
        Response {
            message_size: header.size() + body.size(),
            header,
            body,
        }
    }
}

#[derive(Debug, PartialEq, DekuWrite, DekuRead)]
pub struct ApiVersionsBody {
    #[deku(endian = "big")]
    pub error_code: i16,
    #[deku(endian = "big")]
    length: i32,
    #[deku(count = "length")]
    pub api_keys: Vec<ApiVersion>,
    pub throttle_time_ms: i32,
}

#[derive(Debug, PartialEq, DekuWrite, DekuRead)]
#[deku(endian = "big")]
pub struct ApiVersion {
    pub api_key: i16,
    pub min_version: i16,
    pub max_version: i16,
}

impl Size for ApiVersion {
    fn size(&self) -> i32 {
        2 + 2 + 2
    }
}

#[derive(Debug, PartialEq, DekuWrite, DekuRead, Clone, Copy)]
#[deku(endian = "big")]
pub struct Header {
    pub correlation_id: i32,
}

impl Size for Header {
    fn size(&self) -> i32 {
        4
    }
}
