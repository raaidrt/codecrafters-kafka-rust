use crate::size::Size;
use deku::DekuWrite;

#[derive(Debug, PartialEq, DekuWrite)]
pub struct Response {
    #[deku(endian = "big")]
    message_size: i32,
    pub header: Header,
    #[deku(ctx = "*header")]
    pub body: Body,
}

#[derive(Debug, PartialEq, DekuWrite)]
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
        2 + self
            .api_keys
            .iter()
            .map(ApiVersion::size)
            .fold(0, |a, b| a + b)
    }
}

impl Response {
    pub fn new(header: Header, body: Body) -> Response {
        Response {
            message_size: 4 + header.size() + body.size(),
            header,
            body,
        }
    }
}

#[derive(Debug, PartialEq, DekuWrite)]
pub struct ApiVersionsBody {
    pub error_code: i16,
    pub api_keys: Vec<ApiVersion>,
}

#[derive(Debug, PartialEq, DekuWrite)]
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

#[derive(Debug, PartialEq, DekuWrite, Clone, Copy)]
#[deku(endian = "big")]
pub struct Header {
    pub correlation_id: i32,
}

impl Size for Header {
    fn size(&self) -> i32 {
        4
    }
}
