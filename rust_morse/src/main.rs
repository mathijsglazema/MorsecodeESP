mod gpio;
mod morse_interpreter;
use std::io::{self, Write};

fn main() {
    loop {
        let mut input: String = String::new();
        print!("Please enter a string: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input: &str = input.trim();

        let morse_code: String = morse_interpreter::string_to_morse(input);
        println!("{} in morse code is: {}", input, morse_code);
        gpio::blinking_led(morse_code);

        let mut continue_input: String = String::new();
        print!("Would you like to continue? (y/n): ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut continue_input)
            .expect("Failed to read line");
        if continue_input.trim().eq_ignore_ascii_case("n") {
            break;
        }
    }
}
