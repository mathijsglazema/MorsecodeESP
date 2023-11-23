use std::collections::HashMap;

pub fn string_to_morse(input: &str) -> String {
    let morse_code_map: HashMap<char, &str> = [
        ('a', ".-"),
        ('b', "-..."),
        ('c', "-.-."),
        ('d', "-.."),
        ('e', "."),
        ('f', "..-."),
        ('g', "--."),
        ('h', "...."),
        ('i', ".."),
        ('j', ".---"),
        ('k', "-.-"),
        ('l', ".-.."),
        ('m', "--"),
        ('n', "-."),
        ('o', "---"),
        ('p', ".--."),
        ('q', "--.-"),
        ('r', ".-."),
        ('s', "..."),
        ('t', "-"),
        ('u', "..-"),
        ('v', "...-"),
        ('w', ".--"),
        ('x', "-..-"),
        ('y', "-.--"),
        ('z', "--.."),
        ('1', ".----"),
        ('2', "..---"),
        ('3', "...--"),
        ('4', "....-"),
        ('5', "....."),
        ('6', "-...."),
        ('7', "--..."),
        ('8', "---.."),
        ('9', "----."),
        ('0', "-----"),
        (' ', "/"),
    ]
    .iter()
    .cloned()
    .collect();

    let mut morse_code_string: String = String::new();
    for character in input.to_lowercase().chars() {
        if let Some(morse_code) = morse_code_map.get(&character) {
            morse_code_string.push_str(morse_code);
            morse_code_string.push(' ');
        }
    }

    return morse_code_string;
}
