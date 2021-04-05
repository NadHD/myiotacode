use iota_streams::app_channels::api::tangle::{
    Author, Transport, Address
};
use anyhow::{Result};

pub fn start_a_new_channel<T: Transport>(author: &mut Author<T>) -> Result<Address>{
    let announce_result = author.send_announce()?;
    println!("TEST: Announce Result = {}", announce_result.to_string());
    println!("Channel published");
    Ok(announce_result)
}