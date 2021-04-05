use iota_streams::ddml::types::Bytes;
use iota_streams::app_channels::api::tangle::{
    Author, Address, Transport
};
use anyhow::{Result, bail};

pub fn send_signed_message<T: Transport>(author: &mut Author<T>, channel_address: &String, announce_message_identifier: &String, public_payload: &String) -> Result<Address> {
    let public_payload = Bytes(public_payload.as_bytes().to_vec());
    println!("TEST: publick payload = {}", public_payload);
    let empty_masked_payload = Bytes("".as_bytes().to_vec());
    println!("TEST: masked payload = {}", empty_masked_payload);
    let announcement_link = match Address::from_str(&channel_address, &announce_message_identifier){
        Ok(announcement_link) => announcement_link,
        Err(()) => bail!("Failed to create Address from {}:{}", &channel_address, &announce_message_identifier),
    };
    println!("TEST: announcement link = {}", announcement_link);
    let (msg, seq) = author.send_signed_packet(&announcement_link, &public_payload, &empty_masked_payload).unwrap();
    println!("TEST: msg, seq = {}", msg);
    let seq_unwrapped = seq.unwrap_or(msg);
    println!("TEST: seq.unwrap_or(msg) = {}", seq_unwrapped);
    Ok(seq_unwrapped)
}