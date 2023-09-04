use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::{Error, ErrorKind};
use std::str::FromStr;

pub const SOCKS_VERSION: u8 = 0x05;

#[derive(Debug, PartialEq)]
pub enum AuthMethods {
    NO_AUTH,
    GSSAPI,
    USER_PASS,
}

#[derive(Debug)]
pub enum ConnectionStatus {
    HELLO,
    AUTH,
}

pub fn parse_client_auth_header(header: [u8; 2]) -> Result<u8, std::io::Error> {
    let version = header[0].clone();
    if version != SOCKS_VERSION {
        return Err(Error::new(ErrorKind::Other, format!("Unsupported protocol version {}", version)));
    }
    let nmethods = header[1].clone();
    return Ok(nmethods);
}

pub fn parse_client_method(method: &u8) -> Option<AuthMethods> {
    match method {
        0x00 => Some(AuthMethods::NO_AUTH),
        0x01 => Some(AuthMethods::GSSAPI),
        0x02 => Some(AuthMethods::USER_PASS),
        _ => {None}
    }
}

pub fn parse_username_len(header: [u8; 2]) -> Result<usize, std::io::Error> {
    let version = header[0].clone();
    if version != 0x01 {
        return Err(Error::new(ErrorKind::Other, format!("Unsupported USER/PASS version {}", version)));
    }
    return Ok(header[1].clone() as usize)
}