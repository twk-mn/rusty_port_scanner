// Import env to be able to take arg from user
use std::env;
// Import IpAddr and TcpStream to establish connection to target
use std::net::{IpAddr, TcpStream};

// Creating func is_port_open and take Ip  Addr and port as arguments, then return T/F if the port open/close
fn is_port_open(target: IpAddr, port: u16) -> bool {
    // Try and create a connection to the target IP and port with TcpStream
    match TcpStream::connect((target, port)) {
        // If successful => true
        Ok(_) => true,
        // If fails => false
        Err(_) => false,
    }
}

// Create main func
fn main() {
    // Get target as argument
    let args: Vec<String> = env::args().collect();
    // Pars arg as an IP address
    let target: IpAddr = args[1].parse::<IpAddr>().unwrap();

    // Create vector to store open ports
    let mut open_ports = Vec::new();

    // Loop through ports 1 to 1024
    for port in 1..1025 {
        // Check if port is open
        if is_port_open(target, port) {
            // If open, add to open_ports vector
            open_ports.push(port);
        }
    }
    // Print out the open ports
    print!("Open ports on {}: ", target);

    // Loop through the elements inside open_ports vector and print
    for (i, port) in open_ports.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{}", port);
    }

    // Print a newline character at the end
    println!();
}