use crate::client;

pub(crate) enum Resp {
    SimpleString(String),
    Error(Vec<u8>),
    Integer(i64),
    BulkString(Vec<u8>),
    Array(Vec<Resp>),
}

impl Resp {
    pub(crate) fn serialize(self, buf: &mut Vec<u8>) {
        match self {
            Self::Array(vals) => {
                buf.push(b"*"[0]);
                buf.append(&mut format!("{:?}", vals.len()).into_bytes());
                buf.push('\r' as u8);
                buf.push('\n' as u8);
                for val in vals {
                    val.serialize(buf);
                }
            }
            Self::BulkString(mut string) => {
                buf.push(b'$');
                buf.append(&mut format!("{:?}", string.len()).into_bytes());
                buf.push('\r' as u8);
                buf.push('\n' as u8);
                buf.append(&mut &mut string);
                buf.push('\r' as u8);
                buf.push('\n' as u8);
            }
            _ => {
                unimplemented!()
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
