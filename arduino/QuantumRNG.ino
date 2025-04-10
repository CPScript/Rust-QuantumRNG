/**
 * Quantum Random Number Generator for Arduino
 * 
 * This sketch generates quantum random bits from electronic noise
 * in a reverse-biased semiconductor junction (LED or photodiode).
 * 
 * Circuit:
 * - Connect a photodiode or LED (anode to analog pin, cathode to ground)
 * - Add a 1MΩ pull-up resistor from 5V to the analog pin
 * - Optional: Add light shielding to improve noise quality
 * 
 * The analog pin reads quantum noise from the semiconductor junction,
 * and the least significant bit is extracted to create random bits.
 */

const int NOISE_PIN = A0;       // Analog pin for noise source
const int SAMPLE_DELAY = 10;    // Delay between samples (milliseconds)
const int LED_INDICATOR = 13;   // Built-in LED for status

void setup() {
  Serial.begin(9600);// Initialize serial communication
  while (!Serial) {
    ; // Wait for serial port to connect
  }
  
  pinMode(LED_INDICATOR, OUTPUT);
  pinMode(NOISE_PIN, INPUT);
  
  randomSeed(analogRead(NOISE_PIN));
  
  for (int i = 0; i < 10; i++) {
    analogRead(NOISE_PIN);
    delay(10);
  }
  
  digitalWrite(LED_INDICATOR, HIGH);
  delay(100);
  digitalWrite(LED_INDICATOR, LOW);
  
  Serial.println("Arduino Quantum RNG ready");
}

void loop() {
  // Read analog value (contains quantum noise in LSBs)
  int analogValue = analogRead(NOISE_PIN);
  
  // Extract the least significant bit (quantum noise)
  int randomBit = analogValue & 1;
  
  // Send the bit over serial
  Serial.println(randomBit);
  
  // Visual indicator when sending a 1
  if (randomBit == 1) {
    digitalWrite(LED_INDICATOR, HIGH);
    delay(1);
    digitalWrite(LED_INDICATOR, LOW);
  }
  
  // Wait before next sample
  delay(SAMPLE_DELAY);
}
