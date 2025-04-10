use anyhow::{Result, anyhow};
use opencv::{
    prelude::*,
    videoio,
    core,
    imgproc,
};

/// Generates random bits using quantum noise from webcam CMOS sensor
///
/// # Arguments
/// * `num_bits` - Number of random bits to generate
///
/// # Returns
/// Vector of random bits (0s and 1s as u8)
pub fn webcam_qrng(num_bits: usize) -> Result<Vec<u8>> {
    let mut cap = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    
    if !cap.is_opened()? {// Check if camera is opened
        return Err(anyhow!("Failed to open webcam. Ensure it's connected and not in use by another application"));
    }
    
    cap.set(videoio::CAP_PROP_FRAME_WIDTH, 160.0)?;// Set camera to lowest possible resolution for faster readout
    cap.set(videoio::CAP_PROP_FRAME_HEIGHT, 120.0)?;
    
    println!("Collecting quantum noise from webcam CMOS sensor...");//ensure dark conditions
    println!("For best results: Place webcam in dark container or cover lens");
    
    let mut random_bits = Vec::with_capacity(num_bits);
    let mut frame = Mat::default();
    let mut gray = Mat::default();
    let mut last_percent = 0; //progress tracking

    while random_bits.len() < num_bits {
        // Read a new frame
        if !cap.read(&mut frame)? {
            std::thread::sleep(std::time::Duration::from_millis(10));
            continue;
        }
        
        imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?; // Convert to grayscale
        
        // Extract the least significant bit of each pixel
        let bytes = gray.data_bytes()?;
        for &byte in bytes {
            random_bits.push(byte & 1);
            if random_bits.len() >= num_bits {
                break;
            }
        }
        
        let percent = (random_bits.len() * 100) / num_bits;
        if percent > last_percent && percent % 10 == 0 {
            println!("Progress: {}%", percent);
            last_percent = percent;
        }
    }
    
    println!("Noise collection complete");
    
    cap.release()?;// Release camera
    
    Ok(random_bits[..num_bits].to_vec())
}