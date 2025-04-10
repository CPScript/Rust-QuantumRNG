mod webcam;
mod audio;
mod serial;
mod utils;

use clap::{Parser, Subcommand};
use std::io::Write;
use std::fs::File;
use std::time::Instant;

#[derive(Parser)]
#[command(author, version, about = "Quantum Random Number Generator using consumer hardware")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Webcam { /// Generate random bits using webcam quantum noise
        #[arg(short, long, default_value_t = 1024)]
        num_bits: usize,
        
        #[arg(short, long)]
        output_file: Option<String>,
        
        #[arg(short='d', long, default_value_t = false)]
        apply_debiasing: bool,
        
        #[arg(short='s', long, default_value_t = false)]
        apply_hashing: bool,
    },
    
    Audio {/// Generate random bits using microphone quantum noise
        #[arg(short, long, default_value_t = 1024)]
        num_bits: usize,
        
        #[arg(short, long)]
        output_file: Option<String>,
        
        #[arg(short='d', long, default_value_t = false)]
        apply_debiasing: bool,
        
        #[arg(short='s', long, default_value_t = false)]
        apply_hashing: bool,
    },
    
    Serial {/// Generate random bits using Arduino-based quantum noise
        #[arg(short, long)]
        port: String,
        
        #[arg(short, long, default_value_t = 1024)]
        num_bits: usize,
        
        #[arg(short, long)]
        output_file: Option<String>,
        
        #[arg(short='d', long, default_value_t = false)]
        apply_debiasing: bool,
        
        #[arg(short='s', long, default_value_t = false)]
        apply_hashing: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Webcam { num_bits, output_file, apply_debiasing, apply_hashing } => {
            println!("Generating {num_bits} random bits using webcam quantum noise...");
            let start = Instant::now();
            let mut bits = webcam::webcam_qrng(*num_bits)?;
            let duration = start.elapsed();
            println!("Collection completed in {:.2?}", duration);
            
            process_bits(&mut bits, *apply_debiasing, *apply_hashing, output_file.as_deref())?;
        },
        
        Commands::Audio { num_bits, output_file, apply_debiasing, apply_hashing } => {
            println!("Generating {num_bits} random bits using microphone quantum noise...");
            let start = Instant::now();
            let mut bits = audio::audio_qrng(*num_bits)?;
            let duration = start.elapsed();
            println!("Collection completed in {:.2?}", duration);
            
            process_bits(&mut bits, *apply_debiasing, *apply_hashing, output_file.as_deref())?;
        },
        
        Commands::Serial { port, num_bits, output_file, apply_debiasing, apply_hashing } => {
            println!("Generating {num_bits} random bits using Arduino on port {port}...");
            let start = Instant::now();
            let mut bits = serial::serial_qrng(port, *num_bits)?;
            let duration = start.elapsed();
            println!("Collection completed in {:.2?}", duration);
            
            process_bits(&mut bits, *apply_debiasing, *apply_hashing, output_file.as_deref())?;
        },
    }

    Ok(())
}

fn process_bits(bits: &mut Vec<u8>, apply_debiasing: bool, apply_hashing: bool, output_file: Option<&str>) -> anyhow::Result<()> {
    let original_len = bits.len();
    
    // Calculate entropy statistics
    let mean = bits.iter().map(|&x| x as f64).sum::<f64>() / bits.len() as f64;
    println!("Generated {original_len} bits");
    println!("Raw bit mean: {mean:.4} (ideal: 0.5)");
    
    let mut result_bits = bits.clone();
    
    // Apply Von Neumann debiasing if requested
    if apply_debiasing {
        result_bits = utils::von_neumann_debias(&result_bits);
        let debiased_mean = result_bits.iter().map(|&x| x as f64).sum::<f64>() / result_bits.len().max(1) as f64;
        println!("After debiasing: {} bits", result_bits.len());
        println!("Debiased mean: {:.4} (ideal: 0.5)", debiased_mean);
    }
    
    // Apply SHA-3 hashing if requested
    if apply_hashing {
        result_bits = utils::hash_randomness(&result_bits);
        println!("After cryptographic hashing: {} bits", result_bits.len());
    }
    
    if let Some(filename) = output_file {
        let mut file = File::create(filename)?;
        
        // Write as binary data if bits are multiples of 8
        if result_bits.len() % 8 == 0 {
            let bytes = utils::bits_to_bytes(&result_bits);
            file.write_all(&bytes)?;
            println!("Random bits saved to {filename} ({} bytes)", bytes.len());
        } else {
            for bit in &result_bits {
                write!(file, "{}", bit)?;
            }
            println!("Random bits saved to {filename} as text ({} bits)", result_bits.len());
        }
    }
    
    *bits = result_bits;
    Ok(())
}