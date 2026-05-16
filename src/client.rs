use std::{io::{self, Read}, net::UdpSocket};

pub fn client() {

    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(socket) => socket,
        Err(_e) => return,
    };
    loop {
        let mut buffer: [u8; 256] = [0; 256];

        let data = match io::stdin().read(&mut buffer) {
            Ok(data) => data,
            Err(_e) => return,
        };
        let message = if buffer[..data].ends_with(b"\n") {
            &buffer[..data-1]
        } else {
            &buffer[..data]
        };

        match socket.send_to(message, "0.0.0.0:7777") {
            Ok(socket) => socket,
            Err(_e) => return,
        };
    }
}