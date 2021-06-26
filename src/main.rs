//use async_std::io::Write;

use async_std::{io::{self, prelude::*}};

//use std::io::Write;

mod client;
mod resp;

#[async_std::main]
async fn main() -> io::Result<()>{
    let mut cl = client::Client::new("localhost:6379").await?;
    //ping
    cl.ping().await?;
    //set a value
    cl.set("name".to_string(), "muyiwa".to_string()).await?;
    Ok(())
}




