use std::{collections::HashMap, net::{Ipv4Addr, SocketAddrV4, UdpSocket}};
use crate::signal;
use signal::Signal;

use crate::config_manager::ConfigManager;
use crate::hamming_code::{merge, decode};

pub fn start_subscriber(config_manager: &ConfigManager) {
    
    let max_message_size : u64 = config_manager.get_max_message_size();
    
    //Set up multicast socket
    let multicast_addr = config_manager.get_multicast_address();
    let multicast_port = config_manager.get_multicast_port();
    //TODO: unsure if this can fail if so catch
    let local_addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, multicast_port);

    // Bind the socket to the local address
    let socket = UdpSocket::bind(local_addr).expect("Failed to bind socket");

    // Join the multicast group
    socket
        .join_multicast_v4(&multicast_addr, &Ipv4Addr::UNSPECIFIED)
        .expect("Failed to join multicast group");

    let mut buf: [u8; 65535] = [0u8; 65535];

    // Loop to continuously receive messages
    loop {
        let (amt, src) = socket.recv_from(&mut buf).expect("Failed to receive message");
        let message = &buf[..amt];

        // Unpack the message into a HashMap<u32, f64>
        let signals = unpack_message(message);

        // Handle the signals (for demonstration purposes, we'll just print them)
        for signal in &signals {
            println!("Received signal ID: {}, Value: {}", signal.uid, signal.data);
        }
    }
}

pub fn unpack_message(message: &[u8]) -> Vec<Signal> {
    let mut signals: Vec<Signal> = Vec::new();  // Use a Vec instead of HashMap
    let mut decoded: Vec<u64> = Vec::new();

    // Decode the message into u64 segments
    for chunk in message.chunks_exact(8) {
        let mut segment = u64::from_le_bytes(chunk.try_into().unwrap());
        decoded.push(decode(&mut segment));
    }

    // Merge the decoded segments into a binary byte vector
    let decoded_binary: Vec<u8> = merge(&decoded);

    // Break the merged binary data into 12-byte chunks and convert each into a Signal
    for chunk in decoded_binary.chunks(12) {
        if chunk.len() == 12 {
            if let Some(post_signal) = Signal::from_bytes(chunk) {
                println!("UID: {} Data: {}", post_signal.uid, post_signal.data);
                signals.push(post_signal); // Push the successfully created Signal into the vector
            } else {
                println!("Failed to create Signal from 12-byte chunk.");
            }
        }
    }

    signals
}
