use anyhow::{Result, anyhow};
use serialport::SerialPort;
use std::io::{BufRead, BufReader};
use std::time::Duration;

/// Generate random bits from Arduino-based quantum noise generator
///
/// # Arguments
/// * `port_name` - Serial port name (e.g., "COM3", "/dev/ttyUSB0")
/// * `num_bits` - Number of random bits to generate
///
/// # Returns
/// Vector of random bits (0s and 1s as u8)
pub fn serial_qrng(port_name: &str, num_bits: usize) -> Result<Vec<u8>> {
    println!("Opening serial port {}...", port_name);
    
    // Configure serial port
    let port = serialport::new(port_name, 9600)
        .timeout(Duration::from_millis(1000))
        .open()?;
    
    println!("Connected to Arduino quantum noise generator.");
    println!("Ensure proper circuit setup: Photodiode/LED connected to analog input");
    
    let mut reader = BufReader::new(port); // Read data from Arduino
    let mut bits = Vec::with_capacity(num_bits);
    let mut line = String::new();
    let mut errors = 0;
    let progress_interval = num_bits / 10;
    let mut next_progress = progress_interval;
    
    println!("Reading quantum noise bits from Arduino...");
    
    while bits.len() < num_bits {
        // Read a line from serial
        line.clear();
        match reader.read_line(&mut line) {
            Ok(_) => {
                let bit = line.trim().parse::<u8>();
                match bit {
                    Ok(b) if b == 0 || b == 1 => {
                        bits.push(b);
                        
                        if bits.len() >= next_progress {
                            println!("Progress: {}%", (bits.len() * 100) / num_bits);
                            next_progress += progress_interval;
                        }
                    },
                    _ => {
                        // Invalid data
                        errors += 1;
                        if errors % 100 == 0 {
                            println!("Warning: Received {} invalid readings", errors);
                        }
                    }
                }
            },
            Err(e) => {
                eprintln!("Error reading from serial port: {}", e);
                std::thread::sleep(Duration::from_millis(100));
            }
        }
    }
    
    println!("Quantum noise collection complete");
    
    if errors > 0 {
        println!("Total invalid readings: {}", errors);
    }
    
    Ok(bits[..num_bits].to_vec())
}