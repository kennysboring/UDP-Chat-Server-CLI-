use std::net::UdpSocket;

pub fn server() {
    let socket = match UdpSocket::bind("0.0.0.0:7777") {
        Ok(socket) => socket,
        Err(_e) => return,
    };
    
    loop {
        
        let mut buffer: [u8; 256] = [0; 256];

        let (number_bytes, address) = match socket.recv_from(&mut buffer){
            Ok((number_bytes, address)) => (number_bytes, address),
            Err(_e) => return,
        };
        
        let message = String::from_utf8_lossy(&buffer[..number_bytes]);
        let message = message.trim();

        println!("message: {message}, addres: {address}");
    }
}