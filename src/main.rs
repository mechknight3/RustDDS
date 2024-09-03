mod config_manager;
mod publisher;
mod subscriber;
mod signal_map;
mod signal;
mod hamming_code;

use config_manager::ConfigManager;
use rand::Rng;
use signal::Signal;
use signal_map::SignalMap;
use std::io::{self};


fn main() {
    let config_manager: ConfigManager = ConfigManager::new("config.xml");

    let mut signal_map = SignalMap::new();
    
    let mut rng = rand::thread_rng();
    
    for _ in 0..20000 {
        let id = rng.gen_range(1..100_000_000);  // Random ID between 1 and 100,000,000
        let value = rng.gen_range(0.0..1000.0);  // Random floating-point value between 0.0 and 1000.0
        signal_map.add_signal(Signal::new(id, value));
    }

    println!("1: Start Publisher");
    println!("2: Start Subscriber");

    let mut choice = String::new();

    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice = choice.trim();

    match choice {
        "1" => {
            println!("Starting Publisher...");
            publisher::start_publisher(&config_manager, &signal_map.signals_to_update());
        },
        "2" => {
            println!("Starting Subscriber...");
            subscriber::start_subscriber(&config_manager);
        },
        _ => {
            eprintln!("Invalid choice! Please run the program again and choose a valid option.");
        }
    }
}
