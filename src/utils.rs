use sha3::{Sha3_256, Digest};

/// Implements Von Neumann debiasing algorithm
/// Takes pairs of bits and outputs either nothing (if the bits are equal) 
/// or the first bit (if they're different)
///
/// # Arguments
/// * `bits` - Input bit sequence
///
/// # Returns
/// Debiased bit sequence
pub fn von_neumann_debias(bits: &[u8]) -> Vec<u8> {
    if bits.len() < 2 {
        return bits.to_vec();
    }
    
    let mut result = Vec::with_capacity(bits.len() / 4); // Expect ~25% retention
    
    // Process bit pairs
    for i in (0..bits.len() - 1).step_by(2) {
        let a = bits[i];
        let b = bits[i + 1];
        
        // Only keep first bit of unequal pairs
        if a != b {
            result.push(a);
        }
    }
    
    // Ensure we have some output
    if result.is_empty() && !bits.is_empty() {
        println!("Warning: Von Neumann debiasing produced zero bits. Input may have low entropy.");
        // Return at least one bit to avoid crashes
        result.push(bits[0]);
    }
    
    result
}

/// Cryptographically hash the random bits using SHA-3
///
/// # Arguments
/// * `bits` - Input bit sequence
///
/// # Returns
/// Cryptographically hashed bit sequence
pub fn hash_randomness(bits: &[u8]) -> Vec<u8> {
    if bits.is_empty() {
        return Vec::new();
    }
    
    // Convert bits to bytes
    let bytes = bits_to_bytes(bits);
    
    // Hash using SHA-3 (256-bit)
    let mut hasher = Sha3_256::new();
    hasher.update(&bytes);
    let result = hasher.finalize();
    
    // Convert hash back to bits
    let mut result_bits = Vec::with_capacity(result.len() * 8);
    for byte in result {
        for i in 0..8 {
            result_bits.push((byte >> i) & 1);
        }
    }
    
    result_bits
}

/// Convert bit vector to byte vector
///
/// # Arguments
/// * `bits` - Input bit sequence
///
/// # Returns
/// Byte sequence
pub fn bits_to_bytes(bits: &[u8]) -> Vec<u8> {
    // Pad to multiple of 8 if needed
    let padded_len = if bits.len() % 8 != 0 {
        bits.len() + (8 - bits.len() % 8)
    } else {
        bits.len()
    };
    
    let mut padded_bits = bits.to_vec();
    padded_bits.resize(padded_len, 0);
    
    // Convert bits to bytes
    let mut bytes = Vec::with_capacity(padded_len / 8);
    for chunk in padded_bits.chunks(8) {
        let mut byte = 0u8;
        for (i, &bit) in chunk.iter().enumerate() {
            if bit == 1 {
                byte |= 1 << i;
            }
        }
        bytes.push(byte);
    }
    
    bytes
}

/// Convert byte vector to bit vector
///
/// # Arguments
/// * `bytes` - Input byte sequence
///
/// # Returns
/// Bit sequence
pub fn bytes_to_bits(bytes: &[u8]) -> Vec<u8> {
    let mut bits = Vec::with_capacity(bytes.len() * 8);
    
    for &byte in bytes {
        for i in 0..8 {
            bits.push((byte >> i) & 1);
        }
    }
    
    bits
}

/// Basic entropy estimation (Shannon entropy)
///
/// # Arguments
/// * `bits` - Input bit sequence
///
/// # Returns
/// Estimated entropy (0.0-1.0, where 1.0 is perfect)
pub fn estimate_entropy(bits: &[u8]) -> f64 {
    if bits.is_empty() {
        return 0.0;
    }
    
    // Count zeros and ones
    let ones = bits.iter().filter(|&&b| b == 1).count();
    let zeros = bits.len() - ones;
    
    // Calculate probabilities
    let p0 = zeros as f64 / bits.len() as f64;
    let p1 = ones as f64 / bits.len() as f64;
    
    // Shannon entropy calculation
    let mut entropy = 0.0;
    if p0 > 0.0 {
        entropy -= p0 * p0.log2();
    }
    if p1 > 0.0 {
        entropy -= p1 * p1.log2();
    }
    
    entropy
}

/// Simple statistical tests for randomness quality
///
/// # Arguments
/// * `bits` - Input bit sequence
///
/// # Returns
/// String with test results
pub fn quick_randomness_test(bits: &[u8]) -> String {
    if bits.len() < 100 {
        return "Insufficient bits for testing (need at least 100)".to_string();
    }
    
    // Count zeros and ones
    let ones = bits.iter().filter(|&&b| b == 1).count();
    let zeros = bits.len() - ones;
    
    // Calculate statistics
    let mean = ones as f64 / bits.len() as f64;
    let ideal_mean = 0.5;
    let bias = (mean - ideal_mean).abs();
    
    // Count transitions (0->1 and 1->0)
    let mut transitions = 0;
    for i in 1..bits.len() {
        if bits[i] != bits[i-1] {
            transitions += 1;
        }
    }
    let transition_rate = transitions as f64 / (bits.len() - 1) as f64;
    
    // Entropy calculation
    let entropy = estimate_entropy(bits);
    
    // Format results
    format!(
        "Randomness Test Results:\n\
        - Bit count: {} ({}% ones, {}% zeros)\n\
        - Bias from ideal: {:.4}% (ideal: 0%)\n\
        - Bit transition rate: {:.4} (ideal: 0.5)\n\
        - Shannon entropy: {:.4} bits/bit (ideal: 1.0)\n\
        - Quality assessment: {}",
        bits.len(),
        (ones * 100) / bits.len(),
        (zeros * 100) / bits.len(),
        bias * 100.0,
        transition_rate,
        entropy,
        if bias < 0.05 && entropy > 0.95 { "Good" } 
        else if bias < 0.1 && entropy > 0.9 { "Fair" }
        else { "Poor - Consider debiasing" }
    )
}