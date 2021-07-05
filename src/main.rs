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
use nix::unistd::{fork, ForkResult};
use rand::Rng;
use rand::distributions;

/// Create a responder for the hello world example from the ZMQ Guide.  This one is modified to
/// send a list of strings, however, for demonstrating the RECVMORE option.
///
fn responder() {
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();
    assert!(responder.bind("tcp://*:5555").is_ok());

    let multi_part_message = vec!("Multi", "part", "message", "world");
    loop {
        let message = msg::s_recv_string(&responder).unwrap();
        println!("Received {}", message.as_str().unwrap());
        thread::sleep(Duration::from_millis(1000));
        // responder.send("World", 0).unwrap();
        // msg::s_send_string(&responder, "World").expect("Failed to send message");
        msg::s_send_strings(&responder, &multi_part_message).expect("Failed to send message");
    }
}

/// Create a publisher for the weather server example from the ZMQ Guide
///
fn publisher() {
    let context = zmq::Context::new();
    let publisher = context.socket(zmq::PUB).unwrap();
    assert!(publisher.bind("tcp://*:5556").is_ok());

    let mut rng = rand::thread_rng();

    // Generate some random number generators
    let zipcode_rng = rand::distributions::Uniform::new(1, 99999);
    let temperature_rng = rand::distributions::Uniform::new(-80, 135);
    let humidity_rng = rand::distributions::Uniform::new(10, 60);
    loop {
        // Sample some random numbers
        let zipcode = rng.sample(zipcode_rng);
        let temperature = rng.sample(temperature_rng);
        let humidity = rng.sample(humidity_rng);
        let update = format!("{:05} {} {}", zipcode, temperature, humidity);
        // println!("Updating forecast: {}", update);
        msg::s_send_string(&publisher, &update.as_str());
        //publisher.send(update.as_str(), 0);
    }
}

/// Create a subscriber for the weather client example from the ZMQ Guide
///
/// # Arguments
/// * `zipcode` The zipcode to subscribe to; the stream is filtered for updates starting with this
///
fn subscriber(zipcode: Option<&str>) -> zmq::Result<()>{
    let context = zmq::Context::new();
    let subscriber = context.socket(zmq::SUB).unwrap();
    subscriber.connect("tcp://localhost:5556");

    let filter = match zipcode {
        Some(zip) => zip,
        None => "10001",
    }.as_bytes();
    subscriber.set_subscribe(filter);

    let mut total_temp = 0;
    let mut count = 0;
    for update_number in 0..100 {
        let update = subscriber.recv_string(0)?;
        match update {
           Ok(t)  => {
               println!("String: {}", t);
               let mut iter = t.split(char::is_whitespace);
               let _zipcode = iter.next().and_then(|word| word.parse::<i32>().ok());
               let temperature = iter.next().and_then(|word| word.parse::<i32>().ok());
               let _humidity = iter.next().and_then(|word| word.parse::<i32>().ok());
               if let Some(temp) = temperature {
                   println!("Adding temperature: {}", temp);
                   total_temp += temp;
                   count += 1;
               }
           },
           Err(t) => {
               println!("Vec[u8]: {:?}", t)
           },
        }
    }
    println!("Average temperature: {}", total_temp/count);
    Ok(())
}

/// The main function forkes a few processes to do some of the examples from the ZMQ Guide
///
fn main() {
    messages::msg::s_print_version();

    // let mut msg = zmq::Message::new();
    match unsafe{fork()} {
        Ok(ForkResult::Parent {child, ..}) => {
            responder();
        },
        Ok(ForkResult::Child) => {
            publisher();
        },
        Err(_) => println!("Failure in fork process"),
    }
}
