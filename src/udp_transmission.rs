pub use std::net::UdpSocket;
const MAX_MESSAGE_SIZE: usize = 65507;
use crate::kmeans_struct::*;
//發送長訊息
pub fn send_message(socket: &UdpSocket, message: MessageType) {
    let serialized_msg = serde_json::to_string(&message).expect("Failed to serialize message");
    let mut remaining = serialized_msg.as_bytes();
    let mut _offset = 0;
    while !remaining.is_empty() {
        let chunk = &remaining[..MAX_MESSAGE_SIZE.min(remaining.len())];
        socket.send_to(chunk, "127.0.0.1:8888").expect("Failed to send message");
        socket.send_to(chunk, "127.0.0.1:8889").expect("Failed to send message");
        remaining = &remaining[chunk.len()..];
        _offset += chunk.len();
    }
    // println!("發送完畢");
}
//接收長訊息
pub fn receive_message(socket: &UdpSocket) -> (Box<[u8]>, usize) {
    let mut received_message = Vec::new();
    let mut total_bytes = 0;
    loop {
        let mut buffer = [0; MAX_MESSAGE_SIZE];
        match socket.recv(&mut buffer) {
            Ok(received_bytes) => {
                total_bytes += received_bytes;
                received_message.extend_from_slice(&buffer[..received_bytes]);
                if received_bytes < MAX_MESSAGE_SIZE {
                    // println!("接收完畢");
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error receiving message: {}", e);
                break;
            }
        }
    }
    (received_message.into_boxed_slice(), total_bytes)
}
//自動獲取Port(單機測試)
pub fn get_port(start_port: i32) -> Option<UdpSocket>{
    let mut receive_port = start_port;
    let mut _receive_socket = None;
    loop {
        match UdpSocket::bind(format!("127.0.0.1:{}", receive_port)) {
            Ok(socket) => {
                println!("Successfully bound to port {}", receive_port);
                _receive_socket = Some(socket);
                break;
            }
            Err(_) => {
                println!("Failed to bind to port {}, trying next port", receive_port);
                receive_port += 1;
            }
        }
    }
    _receive_socket
}