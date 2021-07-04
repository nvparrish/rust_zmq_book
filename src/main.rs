#![crate_name = "zmq_book"]

//! Hello World server in Rust
//! Binds REP socket to tcp://*:5555
//! Expects "Hello" from client, replies with "World"
//! Follows the zero MQ tutorial for getting started
//! zeromq.org/get-started/?language=rust&library=rust-zmq#

use std::thread;
use std::time::Duration;
use zmq::Socket;
mod messages;
use messages::msg;

fn main() {
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();

    assert!(responder.bind("tcp://*:5555").is_ok());

    // let mut msg = zmq::Message::new();
    let multi_part_message = vec!("Multi", "part", "message", "world");
    loop {
        //responder.recv(&mut msg, 0).unwrap();
        let message = msg::s_recv_string(&responder).unwrap();
        println!("Received {}", message.as_str().unwrap());
        thread::sleep(Duration::from_millis(1000));
        // responder.send("World", 0).unwrap();
        // msg::s_send_string(&responder, "World").expect("Failed to send message");
        msg::s_send_strings(&responder, &multi_part_message).expect("Failed to send message");
    }
}
