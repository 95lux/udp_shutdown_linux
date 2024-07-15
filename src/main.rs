use std::net::UdpSocket;
use std::process::Command;
use serde_derive::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
struct Config {
    port: u32,
    shutdown_cmd: String,
    reboot_cmd: String,
}

fn main() {

    let exe_path = env::current_exe().expect("Failed to get current exe path");

    let exe_dir = exe_path.parent().expect("Failed to get current directory");

    let config_path = exe_dir.join("config.toml");

    // Read the TOML file
    let toml_str = std::fs::read_to_string(config_path).expect("Failed to read config file");

    // Parse the TOML string into a Config struct
    let config: Config = toml::from_str(&toml_str).expect("Failed to parse config file");

    let address = format!("0.0.0.0:{}", config.port);
    // Create a UDP socket bound to all addresses, and specified port
    let socket = UdpSocket::bind(address.clone()).expect("Failed to bind socket");

    println!("UDP listener started on {}", address);
    println!("shutdown cmd: {}", config.shutdown_cmd);
    println!("reboot cmd: {}", config.reboot_cmd);

    // Buffer to hold incoming data
    let mut buf = [0; 1024];

    loop {
        // Receive data from the socket
        let (num_bytes, _) = socket.recv_from(&mut buf).expect("Failed to receive data");

        // Convert the received data to a string
        let received_data = String::from_utf8_lossy(&buf[..num_bytes]);

        // Print the received data
        println!("Received data: {}", received_data);

        if received_data == config.shutdown_cmd {
            println!("shutting down... ");
            shutdown_pc();
        }
        else if received_data == config.reboot_cmd {
            println!("rebooting... ");
            reboot_pc();
        }
    }
}

fn shutdown_pc() {
    // Execute the shutdown command
    println!("Shutting down PC...");
    Command::new("shutdown")
        .arg("-h")
        .arg("now")
        .spawn()
        .expect("Failed to execute shutdown command");
}

fn reboot_pc() {
    // Execute the shutdown command
    println!("Reboot PC...");
    Command::new("reboot")
        .spawn()
        .expect("Failed to execute shutdown command");
}
