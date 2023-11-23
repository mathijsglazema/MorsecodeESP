use rppal::gpio::{Gpio, OutputPin};
use std::thread;
use std::time::Duration;

const LED_PIN: u8 = 17;

pub fn blinking_led(morse_input: String) {
    let mut led_output_pin: OutputPin = match Gpio::new() {
        Ok(gpio) => gpio.get(LED_PIN).unwrap().into_output(),
        Err(e) => panic!("Error: {}", e),
    };
    for character in morse_input.chars() {
        match character {
            '.' => {
                println!("LED on (dot)");
                led_output_pin.set_high();
                thread::sleep(Duration::from_millis(100)); // Short delay for dot
                println!("LED off");
                led_output_pin.set_low();
            }
            '-' => {
                println!("LED on (dash)");
                led_output_pin.set_high();
                thread::sleep(Duration::from_millis(300)); // Longer delay for dash
                println!("LED off");
                led_output_pin.set_low();
            }
            ' ' => {
                thread::sleep(Duration::from_millis(700)); // Wait for a new letter
            }
            _ => {}
        }
        thread::sleep(Duration::from_millis(300)); // Wait for a new symbol
    }
}