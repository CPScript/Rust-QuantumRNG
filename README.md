# Quantum Random Number Generator (QRNG)

---

## Overview

This project implements a high-performance, quantum random number generator (QRNG) that uses a "quantum mechanical phenomena" available through everyday hardware. Unlike algorithmic pseudo-random number generators (PRNGs), quantum randomness derives from fundamental physical indeterminism, providing truly unpredictable and statistically unbiased random bits.

### Quantum Sources

The system provides three complementary approaches to make an attempt at quantum randomness:

1. **Optical QRNG**: Exploits quantum shot noise in webcam CMOS sensors
2. **Audio QRNG**: Captures Johnson-Nyquist noise in microphone electronics 
3. **Hardware QRNG**: Interfaces with Arduino-based photodiode quantum noise generators

## Features

- **Multiple Quantum Sources**: Flexibility to use webcams, microphones, or custom hardware
- **Enterprise-Grade Architecture**: Modular design with clear separation of concerns
- **Randomness Post-Processing**:
  - Von Neumann debiasing for bias elimination
  - SHA-3 cryptographic conditioning
- **Comprehensive Quality Assessment**: Built-in statistical analysis and entropy estimation
- **High-Performance Implementation**: Optimized Rust code with multi-threading capabilities
- **Production-Ready Error Handling**: Graceful recovery from all I/O operations
- **Format Flexibility**: Outputs binary or text formats for easy integration

## Technical Implementation

### Installation

```bash
# Clone repository
git clone https://github.com/CPScript/rust-quantumRNG.git
cd QuantumRNG

# Install dependencies (Ubuntu/Debian)
sudo apt-get install libopencv-dev libasound2-dev

# Build project
cargo build --release
```

### Usage Examples

```bash
# Generate 10,000 random bits using webcam quantum noise
./QuantumRNG webcam --num-bits 10000 --output-file quantum_bits.bin

# Generate 8,192 bits using microphone with debiasing and cryptographic hashing
./QuantumRNG audio --num-bits 8192 --apply-debiasing --apply-hashing --output-file secure_random.bin

# Generate 4,096 bits from Arduino-based hardware QRNG
./QuantumRNG serial --port /dev/ttyUSB0 --num-bits 4096 --output-file hardware_qrng.bin
```

## Quantum Physics Background

### Optical QRNG (Webcam)

The webcam implementation leverages quantum shot noise - statistical fluctuations in the number of photons detected by the CMOS sensor. These fluctuations arise from the quantum nature of light and are fundamentally random according to quantum mechanics.

In low-light conditions, CMOS sensors exhibit quantum-limited noise behavior, where randomness is dominated by the probabilistic arrival of individual photons. The software extracts the least significant bits from pixel values, which contain this quantum noise component.

### Audio QRNG (Microphone)

Microphone-based randomness exploits Johnson-Nyquist noise - thermal agitation of charge carriers in conductors. At the quantum level, this noise arises from zero-point fluctuations in the electromagnetic field, governed by Heisenberg's uncertainty principle.

When amplified in sensitive microphone electronics, these quantum fluctuations manifest in the least significant bits of audio samples, providing a continuous source of quantum randomness.

### Hardware QRNG (Arduino)

The Arduino implementation utilizes avalanche noise in reverse-biased semiconductor junctions. This noise mechanism involves quantum tunneling effects where electrons spontaneously penetrate potential barriers that would be classically insurmountable.

A simple circuit using a photodiode or LED, combined with appropriate amplification, can extract this quantum noise efficiently.

## Performance Characteristics

| Method | Bit Rate | Setup Complexity | Quantum Purity |
|--------|----------|------------------|----------------|
| Webcam | 1-5 Kbits/sec | Medium | Medium |
| Audio | 10-20 Kbits/sec | Low | Low-Medium |
| Arduino | 0.5-1 Kbits/sec | High | High |

### Statistical Quality

For all methods, the raw bit streams typically achieve:
- Bit balance: 0.48-0.52 (ideally 0.5)
- Shannon entropy: 0.90-0.98 bits/bit (ideally 1.0)
- First-order autocorrelation: < 0.03 (ideally 0)

Post-processing with Von Neumann debiasing and SHA-3 hashing significantly improves these metrics.

## Testing Randomness Quality

The implementation includes basic statistical tests, but for production use, validate with established test suites:

```bash
# Export 1 million random bits
./QuantumRNG webcam --num-bits 1000000 --output-file test.bin

# Test with dieharder
dieharder -a -f test.bin

# Test with NIST Statistical Test Suite
./assess 1000000 < test.bin
```

## Security Considerations

While this implementation brings quantum randomness to consumer hardware, it represents a "best effort" approach and should be assessed for your specific security requirements.

For applications requiring certified quantum randomness, consider:
- Commercial hardware QRNGs
- Quantum randomness beacon services (e.g., NIST Randomness Beacon)
- Hybrid approaches combining this QRNG with existing CSPRNGs

## Circuit Diagrams

### Basic Arduino Quantum Noise Circuit

```
+5V ---[1MΩ resistor]---+---[Photodiode/LED]--- GND
                         |
                    Analog Input
```

### Enhanced Quantum Noise Circuit

```
+5V ---[1MΩ]--- Analog Pin ---[Zener diode 3.3V]--- GND
```

## Advanced Usage

### Integration with Cryptographic Applications

```rust
// Example: Using quantum randomness for AES-256 key generation
let quantum_bits = std::fs::read("quantum_bits.bin")?;
let key = &quantum_bits[0..32]; // 256 bits for AES-256
let cipher = Aes256Gcm::new(key.into());
```

### Combining Multiple Quantum Sources

```bash
# Generate bits from multiple sources and combine them
./QuantumRNG webcam --num-bits 1000 --output-file webcam.bin
./QuantumRNG audio --num-bits 1000 --output-file audio.bin

# Combine with XOR operation (in bash)
xxd -p webcam.bin | tr -d '\n' > combined.hex
xxd -p audio.bin | tr -d '\n' | python -c "import sys; \
  a=sys.stdin.read(); b=open('combined.hex').read(); \
  print(''.join(hex(int(a[i:i+2],16)^int(b[i:i+2],16))[2:] for i in range(0,len(a),2)))" | \
  xxd -p -r > quantum_bits.bin
```

## Future Directions

- FPGA-based implementations for higher throughput
- Integration with cloud-based quantum computers
- Hardware-accelerated post-processing
- Real-time entropy monitoring and quality control
- Continuous randomness generation as a system service

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgments

This project builds upon theoretical and practical work in the field of quantum random number generation and applies these principles to consumer hardware.
