use std::{fmt::Write, ops::Deref};

use crate::client;
use bytes::{BufMut, Bytes, BytesMut};

/// According to the Redis protocol spec, this union type defines the different types of responses in Redis
pub(crate) enum Resp {
    SimpleString(String),
    Error((String, String)),
    Integer(i64),
    BulkString(Vec<u8>),
    Array(Vec<Resp>),
}

impl Resp {
    pub(crate) fn serialize(self, b: &mut BytesMut) {
        match self {
            Self::Array(vals) => {
                b.put_u8(b'*');
                <BytesMut as BufMut>::put_u32( b, vals.len() as u32);
                b.put_u8(b'\r');
                b.put_u8(b"\n"[0]);
                for val in vals {
                    val.serialize(b);
                }
            }
            Self::BulkString(mut string) => {
                b.put_u8(b'$');
                <BytesMut as BufMut>::put_u32( b, string.len() as u32);
                b.put_u8(b'\r');
                b.put_u8('\n' as u8);
                b.copy_from_slice(&string[..]);
                b.copy_from_slice(&b"\r\n"[..]);
            }
            Self::SimpleString(string) => {
                b.put_u8(b'+');
                b.write_str(string.as_str());
                b.copy_from_slice(&b"\r\n"[..]);
            }
            Self::Integer(i) => {
                b.put_u8(b':');
                b.put_i64(i);
                b.copy_from_slice(&b"\r\n"[..]);
            }
            Self::Error((err_type, msg)) => {
                b.put_u8(b'-');
                b.copy_from_slice(err_type.as_bytes());
                b.copy_from_slice(msg.as_bytes());
                b.copy_from_slice(&b"\r\n"[..]);
            }
        }
    }
}

pub(crate) fn parse_resp(buf: &[u8]) -> Result<&str, client::Error> {
    if buf.is_empty() {
        return Err(client::Error {});
    }
    if buf[0] == b"-"[0] {
        return Err(
            //format!(
            //"error occured: {:?}", &buf[1..buf.len() - 2])
            client::Error {},
        );
    }

    Ok(std::str::from_utf8(&buf[1..buf.len() - 2]).unwrap())
}
