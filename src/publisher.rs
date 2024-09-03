use std::collections::HashMap;
use std::net::{UdpSocket, Ipv4Addr, SocketAddrV4};
use std::thread;
use std::time::{Duration, Instant};

use crate::config_manager::ConfigManager;
use crate::hamming_code::{encode, segment};
use crate::signal;
use signal::Signal;

pub fn start_publisher(config_manager: &ConfigManager, signal_map: &Vec<Signal>) {
    
    let max_message_size : u64 = config_manager.get_max_message_size();

    //Set up multicast socket
    let multicast_addr: Ipv4Addr = config_manager.get_multicast_address();
    let multicast_port: u16 = config_manager.get_multicast_port();
    //TODO: unsure if this can fail if so catch
    let multicast_socket: SocketAddrV4 = SocketAddrV4::new(multicast_addr, multicast_port);
    let publish_rate = config_manager.get_publish_rate();

    // Bind the socket to a local address
    let socket: UdpSocket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");
    
    // Set up Time to Live
    socket.set_ttl(1).expect("Failed to set TTL");


    //MOVE
    

    // Message to send
    //TODO Multithread  so package messages adds to quene 
    let messages: Vec<Vec<u8>> = package_messages(&signal_map, max_message_size);

    loop {

        //TODO: maybe slow??
        let start_time = Instant::now();

        //TODO Multithread 
        for message in &messages {
            socket
                .send_to(message, multicast_socket)
                .expect("Failed to send message");
        }


        
        // Calculate the elapsed time
        let elapsed_time: Duration = start_time.elapsed();        
        
        // Display the time taken to send the message
        println!("Message sent in {:?}", elapsed_time);
    
        
        if elapsed_time < Duration::from_millis(publish_rate) {
            let sleep_time: Duration = Duration::from_millis(publish_rate) - elapsed_time;
            thread::sleep(sleep_time);
        } else {
            // If the elapsed time exceeds the publish rate, print a warning
            println!(
                "Warning: Message took longer than the publish rate! Elapsed time: {:?}",
                elapsed_time
            );
        }
    }
}

pub fn package_messages(signals: &Vec<Signal>, max_message_size: u64) -> Vec<Vec<u8>> {

    let mut messages: Vec<Vec<u8>> = Vec::new();
    let mut signal_bytes: Vec<u8> = Vec::new();

    //convert to vector of bytes
    for signal in signals {
        signal_bytes.extend(signal.to_bytes());
    }

    //segment to 57 bits segements in 64 bits first bits empty
    let segments: Vec<u64> = segment(&signal_bytes);
    
    let mut encoded: Vec<u64> = Vec::new();

    //encode parity 
    for mut segment in segments  {
        encoded.push(encode(&mut segment))
    }

    let mut encoded_bytes: Vec<u8> = encoded.iter().flat_map(|&segment| segment.to_le_bytes()).collect();

    // Split encoded bytes into messages of max_message_size
    while !encoded_bytes.is_empty() {
        let chunk_size = std::cmp::min(encoded_bytes.len(), max_message_size as usize);
        let chunk = encoded_bytes.drain(..chunk_size).collect();
        messages.push(chunk);
    }

    messages
}


fn to_binary<T>(values: Vec<T>) -> Vec<u8>
where
    T: Into<u8>,
{
    values.into_iter().map(|v| v.into()).collect()
}


