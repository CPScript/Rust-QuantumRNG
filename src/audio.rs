use anyhow::{Result, anyhow};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;

/// Generates random bits using quantum noise from microphone electronics
///
/// # Arguments
/// * `num_bits` - Number of random bits to generate
///
/// # Returns
/// Vector of random bits (0s and 1s as u8)
pub fn audio_qrng(num_bits: usize) -> Result<Vec<u8>> {
    println!("Initializing audio subsystem for quantum noise collection...");
    
    let host = cpal::default_host();
    let device = host.default_input_device()
        .ok_or_else(|| anyhow!("No microphone available. Check connections and permissions"))?;
    
    println!("Using input device: {}", device.name()?);
    
    let config = device.default_input_config()?;
    println!("Sample format: {:?}, Sample rate: {}", config.sample_format(), config.sample_rate().0);
    
    let sample_format = config.sample_format();
    let config = config.into();
    
    let samples = Arc::new(Mutex::new(Vec::new()));
    let samples_clone = samples.clone();
    
    // Determine duration based on needed bits (with safety margin)
    // Assume 16-bit samples and we extract 1 bit per sample
    let duration_sec = (num_bits as f32 / config.sample_rate().0 as f32 * 1.5).max(1.0);
    
    // Setup callback function for audio data
    let stream = match sample_format {
        cpal::SampleFormat::I16 => device.build_input_stream(
            &config,
            move |data: &[i16], _: &_| {
                let mut samples = samples_clone.lock().unwrap();
                samples.extend_from_slice(data);
            },
            |err| eprintln!("Error in audio stream: {}", err),
            None,
        )?,
        cpal::SampleFormat::U16 => device.build_input_stream(
            &config,
            move |data: &[u16], _: &_| {
                let mut samples = samples_clone.lock().unwrap();
                samples.extend(data.iter().map(|&s| s as i16));
            },
            |err| eprintln!("Error in audio stream: {}", err),
            None,
        )?,
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config,
            move |data: &[f32], _: &_| {
                let mut samples = samples_clone.lock().unwrap();
                samples.extend(data.iter().map(|&s| (s * 32767.0) as i16));
            },
            |err| eprintln!("Error in audio stream: {}", err),
            None,
        )?,
        _ => return Err(anyhow!("Unsupported sample format: {:?}", sample_format)),
    };
    
    println!("Recording quantum noise from microphone... (keep room silent)");
    println!("For best results: Shield microphone from external noise");
    
    // Start recording
    stream.play()?;
    
    for i in (1..=duration_sec as u32).rev() {
        println!("Recording: {} seconds remaining...", i);
        thread::sleep(Duration::from_secs(1));
    }
    
    // Stop recording
    drop(stream);
    
    // Extract bits from samples
    let samples = samples.lock().unwrap();
    let mut bits = Vec::with_capacity(num_bits);
    
    println!("Collected {} audio samples, extracting quantum noise bits...", samples.len());
    
    for &sample in &*samples {
        // Extract least significant bit
        bits.push((sample & 1) as u8);
        if bits.len() >= num_bits {
            break;
        }
    }
    
    if bits.len() < num_bits {
        println!("Warning: Could only collect {} bits, requested {}", bits.len(), num_bits);
        return Err(anyhow!("Insufficient audio data collected. Try increasing recording time"));
    }
    
    println!("Quantum noise extraction complete");
    Ok(bits[..num_bits].to_vec())
}