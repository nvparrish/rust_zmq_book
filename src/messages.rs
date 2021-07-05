use zmq::Socket;
use zmq;

pub mod msg {
    pub fn s_print_version() {
        let (major, minor, patch) = zmq::version();
        println!("The version of ZMQ is {}.{}.{}", major, minor, patch);
    }

    pub fn s_send_string(socket: &zmq::Socket, message: &str) -> zmq::Result<()> {
        socket.send(message, 0)
    }

    pub fn s_recv_string(socket: &zmq::Socket) -> zmq::Result<zmq::Message> {
        let mut response = zmq::Message::new();
        let result = match socket.recv(&mut response, 0) {
            zmq::Result::Ok(()) => zmq::Result::Ok(response),
            zmq::Result::Err(e) => zmq::Result::Err(e),
        };
        result
    }

    pub fn s_send_strings(socket: &zmq::Socket, messages: &Vec<&str>) -> zmq::Result<()> {
        let mut result = zmq::Result::Ok(());
        for (i, message) in messages.iter().enumerate() {
            let flag = if i == messages.len() - 1_usize {
                0
            } else {
                zmq::SNDMORE
            };
            if let zmq::Result::Err(e) = socket.send(message, flag) {
                result = zmq::Result::Err(e);
            }
        }
        result
    }
}