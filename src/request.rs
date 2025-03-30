use deku::prelude::*;

use crate::{response, size::Size};

pub mod error_code {
    pub const UNSUPPORTED_VERSION: i16 = 35;
}

pub mod api_key {
    pub const API_VERSIONS: i16 = 18;
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct Header {
    pub request_api_key: i16,
    pub request_api_version: i16,
    pub correlation_id: i32,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct Request {
    #[deku(endian = "big")]
    message_size: i32,
    pub header: Header,
}

impl Size for Header {
    fn size(&self) -> i32 {
        2 + 2 + 4
    }
}

impl Request {
    pub fn new(header: Header) -> Request {
        Request {
            message_size: header.size(),
            header,
        }
    }
    pub fn handle(self) -> response::Response {
        match self.header.request_api_key {
            api_key::API_VERSIONS => match self.header.request_api_version {
                4 => response::Response::new(
                    response::Header {
                        correlation_id: self.header.correlation_id,
                    },
                    response::Body::ApiVersions(response::ApiVersionsBody {
                        error_code: 0,
                        api_keys: vec![response::ApiVersion {
                            api_key: api_key::API_VERSIONS,
                            min_version: 0,
                            max_version: 4,
                        }],
                    }),
                ),
                _ => response::Response::new(
                    response::Header {
                        correlation_id: self.header.correlation_id,
                    },
                    response::Body::ApiVersions(response::ApiVersionsBody {
                        error_code: error_code::UNSUPPORTED_VERSION,
                        api_keys: vec![],
                    }),
                ),
            },
            _ => panic!("Unsupported API Key"),
        }
    }
}

fn format_hex_dump(bytes: &[u8]) -> String {
    bytes
        .chunks(8)
        .enumerate()
        .map(|(i, chunk)| {
            let addr = i * 8;
            let hex_groups = chunk
                .chunks(4)
                .map(|group| {
                    group
                        .iter()
                        .map(|b| format!("{:02x}", b))
                        .collect::<Vec<_>>()
                        .join(" ")
                })
                .collect::<Vec<_>>()
                .join("  "); // Double space between 4-byte groups

            format!("{:04x} | {}", addr, hex_groups)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn snapshot_valid_api_versions_request() {
    let header = Header {
        request_api_key: api_key::API_VERSIONS,
        request_api_version: 4,
        correlation_id: 4321,
    };

    let request = Request::new(header);
    let response = request.handle();

    let bytes = response.to_bytes().unwrap();

    // Represent the binary output in hex for readability
    let hex_string = format_hex_dump(&bytes);
    insta::assert_snapshot!(hex_string);
}
