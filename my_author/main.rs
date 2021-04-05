//Importing dependencies
use iota::client as iota_client;

use iota_streams::app_channels::api::tangle::{
    Address, Transport, Subscriber
};
use iota_streams::app::transport::tangle::{
    PAYLOAD_BYTES,
    client::{
        Client,
        SendTrytesOptions
    }
};

use anyhow::{Result, bail};
use std::env;

fn main() {
    // Change the default settings to use a lower minimum weight magnitude for the Devnet
    let mut send_opt = SendTrytesOptions::default();
    send_opt.min_weight_magnitude = 9;
    send_opt.local_pow = false;

    let url = "https://nodes.devnet.iota.org:443";

    // Connect to an IOTA node
    let client: Client = Client::new(send_opt, iota_client::ClientBuilder::new().node(url).unwrap().build().unwrap());
    let encoding = "utf-8";
    let mut subscriber = Subscriber::new("PROVAPROVAPROVAPROVA", encoding, PAYLOAD_BYTES, client);

    let args: Vec<String> = env::args().collect();

    let channel_address = &args[1];
    let announce_message_identifier = &args[2];
    let signed_message_identifier = &args[3];

    match get_announcement(&mut subscriber, &channel_address.to_string(), &announce_message_identifier.to_string()){
        Ok(()) => (),
        Err(error) => println!("Failed with error {}", error),
    }
    
    match get_signed_messages(&mut subscriber, &channel_address.to_string(), &signed_message_identifier.to_string()){
        Ok(()) => (),
        Err(error) => println!("Failed with error {}", error),
    }
}

//Changing the addres and identifier into a link
fn create_message_link(channel_address: &String, message_identifier: &String) -> Result<Address> {
    let message_link = match Address::from_str(&channel_address, &message_identifier) {
        Ok(message_link) => message_link,
        Err(()) => bail!("Failed to create Address from {}:{}", &channel_address, &message_identifier),
    };
    Ok(message_link)
}

fn get_announcement<T: Transport>(subscriber: &mut Subscriber<T>, channel_address: &String, announce_message_identifier: &String) -> Result<()>{
    let announcement_link = match create_message_link(&channel_address, &announce_message_identifier){
        Ok(announcement_link) => announcement_link,
        Err(error) => bail!(error),
    };

    //Printing the Announce message
    println!("Receiving announcement message");
    subscriber.receive_announcement(&announcement_link)?;

    Ok(())
}

fn get_signed_messages<T: Transport>(subscriber: &mut Subscriber<T>, channel_address: &String, signed_message_identifier: &String) -> Result<()> {
    // Convert the channel address and message identifier to a link
    let message_link = match create_message_link(&channel_address, &signed_message_identifier){
        Ok(message_link) => message_link,
        Err(error) => bail!(error),
    };

    // First returned value is the senders public key. We wont be using that in this tutorial
    let (_, public_payload, masked_payload) = subscriber.receive_signed_packet(&message_link)?;
    println!("Found and verified message");
    println!("Public message: {:?}, private message: {:?}", 
        String::from_utf8(public_payload.0), String::from_utf8(masked_payload.0));
    Ok(())
}
