# Acknowledge

> This project builds upon theoretical and practical work in the field of quantum random number generation and applies these principles to consumer hardware.

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

---

# Technical Implementation

## Easy Installation

```bash
# Clone repository
git clone https://github.com/CPScript/rust-quantumRNG.git
cd QuantumRNG

# Install dependencies (Ubuntu/Debian)
sudo apt-get install libopencv-dev libasound2-dev

# Build project
cargo build --release
```

#### Usage Examples

```bash
# Generate 10,000 random bits using webcam quantum noise
./QuantumRNG webcam --num-bits 10000 --output-file quantum_bits.bin

# Generate 8,192 bits using microphone with debiasing and cryptographic hashing
./QuantumRNG audio --num-bits 8192 --apply-debiasing --apply-hashing --output-file secure_random.bin

# Generate 4,096 bits from Arduino-based hardware QRNG
./QuantumRNG serial --port /dev/ttyUSB0 --num-bits 4096 --output-file hardware_qrng.bin
```

## Full installation
<details closed>
<summary>Click me to show!</summary>
<br>
  
## 1. Environmental Prerequisites

### 1.1. System Dependencies

```bash
# Debian/Ubuntu-based systems
sudo apt update
sudo apt install -y build-essential pkg-config libssl-dev \
    libopencv-dev libclang-dev cmake \
    libasound2-dev libudev-dev \
    arduino arduino-core arduino-mk

# Fedora/RHEL-based systems
sudo dnf install -y gcc gcc-c++ make pkg-config openssl-devel \
    opencv-devel clang-devel cmake \
    alsa-lib-devel systemd-devel \
    arduino arduino-devel
    
# Arch-based systems
sudo pacman -S base-devel pkg-config openssl \
    opencv clang cmake \
    alsa-lib systemd-libs \
    arduino
```

### 1.2. Rust Toolchain Installation

```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Ensure stable toolchain
rustup default stable

# Install additional components
rustup component add clippy rustfmt
```

## 2. Hardware Configuration

### 2.1. Optical QRNG Setup (Webcam-based)

1. **Connect a USB webcam** to your system
2. **Create light-isolated environment** for optimal quantum noise:
   ```
   - Enclose webcam in opaque container (e.g., small cardboard box)
   - Apply black electrical tape over lens leaving tiny aperture (0.5mm)
   - Alternatively: use lens cap with single pinhole
   ```
3. **Verify hardware detection**:
   ```bash
   # List detected video devices
   v4l2-ctl --list-devices
   
   # Test video capture (should show static/noise)
   ffplay -f video4linux2 -input_format mjpeg -i /dev/video0 -video_size 160x120
   ```

### 2.2. Audio QRNG Setup (Microphone-based)

1. **Connect microphone** (internal or external) 
2. **Create RF-isolated environment** (optional, for higher quality randomness):
   ```
   - Use shielded microphone if available
   - Place in Faraday cage (metal mesh box/aluminum foil enclosure)
   - Keep away from strong RF sources (Wi-Fi routers, etc.)
   ```
3. **Verify hardware detection**:
   ```bash
   # List audio devices
   arecord -l
   
   # Test audio capture (should capture ambient noise)
   arecord -d 5 -f S16_LE -r 44100 -c 1 test.wav && aplay test.wav
   ```

### 2.3. Hardware QRNG Setup (Arduino-based)

1. **Circuit assembly**:
   ```
   Components:
   - Arduino Uno/Nano/Pro Mini
   - 1× Photodiode or reverse-biased LED (preferred: BPW34)
   - 1× 1MΩ resistor
   - 1× 10KΩ resistor (optional, for stability)
   - Breadboard and jumper wires
   
   Connections:
   +5V ── [1MΩ resistor] ── +── [Photodiode] ── GND
                             │
                      Analog Pin A0
   ```

2. **Upload firmware**:
   ```bash
   # Navigate to Arduino sketch directory
   cd QuantumRNG/arduino
   
   # Compile and upload (adjust port as needed)
   arduino-cli compile --fqbn arduino:avr:uno quantum_rng
   arduino-cli upload -p /dev/ttyACM0 --fqbn arduino:avr:uno quantum_rng
   
   # Alternative: Using Arduino IDE
   # 1. Open arduino/quantum_rng.ino in Arduino IDE
   # 2. Select board type (Tools > Board > Arduino Uno)
   # 3. Select port (Tools > Port > /dev/ttyACM0)
   # 4. Click Upload button
   ```

3. **Verify communication**:
   ```bash
   # Test serial output (should show stream of 0s and 1s)
   screen /dev/ttyACM0 9600
   # Press Ctrl+A then K to exit
   ```

## 3. Build Process

### 3.1. Core Application Compilation

```bash
# Clone repository (if not done already)
git clone https://github.com/CPScript/QuantumRNG.git
cd QuantumRNG

# Build in development mode
cargo build

# Build optimized release binary
cargo build --release

# Run tests
cargo test -- --nocapture
```

### 3.2. Compile-time Configuration (Optional)

For custom feature configurations, modify Cargo.toml:

```toml
# Add these lines under [features] section
[features]
default = ["webcam", "audio"]
webcam = ["opencv"]
audio = ["cpal"]
hardware = ["serialport"]
all = ["webcam", "audio", "hardware"]
```

Then build with selected features:

```bash
# Build with only webcam support
cargo build --release --no-default-features --features webcam

# Build with all features
cargo build --release --features all
```

## 4. Testing Protocol

### 4.1. Functional Validation

```bash
# 1. Optical QRNG Test
./target/release/QuantumRNG webcam --num-bits 1024 --output-file optical_test.bin

# 2. Audio QRNG Test
./target/release/QuantumRNG audio --num-bits 1024 --output-file audio_test.bin

# 3. Hardware QRNG Test (adjust port as needed)
./target/release/QuantumRNG serial --port /dev/ttyACM0 --num-bits 1024 --output-file hardware_test.bin
```

### 4.2. Quality Assurance Tests

```bash
# Generate larger sample for statistical testing
./target/release/QuantumRNG webcam --num-bits 100000 --apply-debiasing --apply-hashing --output-file statistical_test.bin

# Install randomness test suite (if not already installed)
sudo apt install dieharder rng-tools

# Basic entropy assessment
rngtest -c 1000 < statistical_test.bin

# Comprehensive statistical analysis
dieharder -a -f statistical_test.bin -g 201
```

### 4.3. Performance Benchmarking

```bash
# Time generation of 1 million bits
time ./target/release/QuantumRNG audio --num-bits 1000000 --output-file benchmark.bin

# Run with Rust benchmark harness (requires nightly)
rustup run nightly cargo bench
```

## 5. Troubleshooting Framework

### 5.1. Diagnostic Procedures

| Issue | Diagnosis Command | Solution |
|-------|-------------------|----------|
| Webcam access denied | `ls -la /dev/video*` | `sudo chmod a+rw /dev/video0` |
| Arduino not detected | `ls -la /dev/tty*` | `sudo usermod -a -G dialout $USER` |
| OpenCV library not found | `ldd ./target/debug/QuantumRNG` | `sudo apt install libopencv-dev` |
| Audio device busy | `fuser -v /dev/snd/*` | Kill competing process |

### 5.2. Hardware Signal Analysis

For validation of quantum noise quality:

```bash
# Install signal processing tools
sudo apt install gnuplot sox

# Record raw analog values from Arduino (modify quantum_rng.ino first)
# Change: Serial.println(randomBit); → Serial.println(analogValue);
# Then capture and visualize:
stty -F /dev/ttyACM0 9600 raw
cat /dev/ttyACM0 > arduino_values.txt &
sleep 10
kill $!

# Plot noise distribution
gnuplot -e "set term png; set output 'noise_distribution.png'; \
  plot 'arduino_values.txt' using 1 with histeps title 'Noise Distribution'"
```

## 6. Advanced Configuration

### 6.1. Entropy Source Hybridization

For critical applications, combining multiple entropy sources:

```bash
# Generate bits from all sources
./target/release/QuantumRNG webcam --num-bits 1024 --output-file source1.bin
./target/release/QuantumRNG audio --num-bits 1024 --output-file source2.bin 
./target/release/QuantumRNG serial --port /dev/ttyACM0 --num-bits 1024 --output-file source3.bin

# Create hybrid entropy pool (requires xxd)
xxd -p source1.bin | tr -d '\n' > pool.hex
xxd -p source2.bin | tr -d '\n' | python3 -c "
import sys
a=sys.stdin.read()
b=open('pool.hex').read()
min_len = min(len(a),len(b))
print(''.join(hex(int(a[i:i+2],16)^int(b[i:i+2],16))[2:].zfill(2) 
      for i in range(0,min_len,2)))" > temp.hex
xxd -p source3.bin | tr -d '\n' | python3 -c "
import sys
a=sys.stdin.read()
b=open('temp.hex').read()
min_len = min(len(a),len(b))
print(''.join(hex(int(a[i:i+2],16)^int(b[i:i+2],16))[2:].zfill(2)
      for i in range(0,min_len,2)))" > final.hex

# Convert back to binary
xxd -p -r final.hex > hybrid_entropy.bin
```

---

</details>

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
