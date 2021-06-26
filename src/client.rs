use async_std::{io::prelude::WriteExt,io::prelude::ReadExt, net::{TcpStream, ToSocketAddrs}};
use std::io::Error as IoError;

use crate::resp;

pub struct Client {
	pub stream: TcpStream,
}

impl Client {

	pub async fn new<A: ToSocketAddrs>(addr: A) -> Result<Self, IoError> {
		let stream = TcpStream::connect(addr).await?;
		Ok(Self{
			stream
		})
	}

	pub async fn set(&mut self, key: String, val: String) -> Result<(), Error>{
		let mut buf = vec![];
		let command = resp::Resp::Array(
			vec![
					resp::Resp::BulkString(b"set".to_vec()), 
					resp::Resp::BulkString(key.into_bytes()), 
					resp::Resp::BulkString(val.into_bytes())
				]
			);
		command.serialize(&mut buf);
		self.stream.write_all(&mut buf).await?;
		let num_bytes = self.stream.read(&mut buf).await?;
		let res = resp::parse_resp(&buf[..num_bytes])?;
		println!("{:?}", res);
		Ok(())
	}

	pub async fn ping(&mut self) -> Result<(), IoError> {
		let mut buf = vec![];
		let command = resp::Resp::Array(vec![resp::Resp::BulkString(b"PING".to_vec())]);
		command.serialize(&mut buf);
		self.stream.write_all(&mut buf).await?;
		let num_bytes = self.stream.read(&mut buf).await?;
		let res = resp::parse_resp(&buf[..num_bytes])?;
		println!("{:?}", res);
		Ok(())
	}
}

#[derive(Debug)]
pub struct Error{}

impl std::convert::From<IoError> for Error {
	fn from(err: IoError) -> Self {
	    Self{}
	}
}

impl std::convert::From<Error> for IoError {
	fn from(e: Error) -> Self {
	    IoError::last_os_error()
	}
}