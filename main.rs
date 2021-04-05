use iota_streams::app_channels::api::tangle::Author;

mod my_api;
use crate::my_api::announce::start_a_new_channel;
use crate::my_api::send_message::send_signed_message;

use iota_streams::app::{
    transport::tangle::{
        PAYLOAD_BYTES,
        client:: {
            Client,
            SendTrytesOptions
        }
    }
};

use iota::client as iota_client;

fn main(){
    let mut send_opt = SendTrytesOptions::default();
    send_opt.min_weight_magnitude = 9;
    send_opt.local_pow = false;

    let url = "https://nodes.devnet.iota.org:443";

    let client: Client = Client::new(send_opt, iota_client::ClientBuilder::new().node(url).unwrap().build().unwrap());
    let encoding = "utf-8";
    let multi_branching_flag = true;
    let mut author = Author::new("TESTPROVAPROVATEST7", encoding, PAYLOAD_BYTES, multi_branching_flag, client);
    let channel_address = author.channel_address().unwrap().to_string();
    println!("TEST: channel_address = {}", &channel_address);
    let announce_message = start_a_new_channel(&mut author).unwrap();
    let announce_msgid = announce_message.msgid.to_string();
    println!("TEST: announce_message = {}", &announce_message.to_string());
    println!("TEST : announce_msgid = {}", &announce_msgid);
    let public_payload = "SONOIOSTOTESTANDO";
    let signed_message = send_signed_message(&mut author, &channel_address, &announce_msgid, &public_payload.to_string()).unwrap();
    
    println!("");
    println!("Now, in a new terminal window, use the subscriber to publish a `Subscribe` message on the channel");
    println!("");
    println!("cargo run {} {} {}", 
        channel_address, 
        announce_msgid, 
        signed_message.msgid
    );
    println!("");
    println!("Tangle Address/channel: {}", iota_client::bytes_to_trytes(author.channel_address().unwrap().as_ref()));
    println!("Tangle announce_message tag: {}", iota_client::bytes_to_trytes(announce_message.msgid.as_ref()));
    println!("Tangle signed_message tag: {}", iota_client::bytes_to_trytes(signed_message.msgid.as_ref()));
}