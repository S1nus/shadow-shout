use std::fs::read_to_string;

use async_std::{
    net::{TcpListener,
        TcpStream,
    },
    task,
};

use sodiumoxide::crypto::box_;

mod config;
use config::Config;

/*async fn socket_loop() {

    let addr = "0.0.0.0:7474".to_string();
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind to port");
    println!("Listening on {}", addr);

    while let Ok((stream, addr)) = listener.accept().await {
        println!("addr: {:?}", addr);
    }

}*/

fn main() {

    let config_text: String = read_to_string("config.toml")
        .expect("couldn't find config.toml")
        .parse()
        .expect("couldn't parse config file");

    let config : Config = toml::from_str(config_text.as_str())
        .expect("invalid toml syntax");

    println!("bootstrap node: {:?}", config.bootstrap_nodes);

    //let config : Config = 

    /*let (ourpk, oursk) = box_::gen_keypair();
    println!("Our PK: {:?}", ourpk);
    println!("Our SK: {:?}", oursk);

    let (theirpk, theirsk) = box_::gen_keypair();
    println!("their PK: {:?}", theirpk);
    println!("their SK: {:?}", theirsk);

    let nonce = box_::gen_nonce();
    println!("Nonce {:?}", nonce);

    task::block_on(socket_loop());*/

}
