use serde_json::{self};
use std::collections::HashMap;
use std::fs;
use std::io;

fn get_error_suggestions() -> HashMap<String, String> {
    match fs::read_to_string("error.json") {
        Ok(json_str) => serde_json::from_str(&json_str).unwrap_or_else(|_| HashMap::new()),
        Err(_) => HashMap::new(),
    }
}

fn save_error_suggestions(suggestions: &HashMap<String, String>) {
    let json_str =
        serde_json::to_string_pretty(suggestions).expect("Failed to serialize suggetions");
    fs::write("error.json", json_str).expect("Failed to write to error.json");
}

fn _extract_error_codes(message: &str) -> Vec<String> {
    let mut codes = Vec::new();
    let mut remaining = message;

    while let Some(start) = remaining.find("E") {
        let end = start + 5;
        if end <= remaining.len()
            && remaining[start..end]
                .chars()
                .skip(1)
                .all(|c| c.is_digit(10))
        {
            let code = &remaining[start..end];
            codes.push(code.to_string());
            remaining = &remaining[end..];
        } else {
            remaining = &remaining[start + 1..]
        }
    }
    codes
}

fn main() {
    println!("RustEdge Agent starting...");
    let mut esuggestions = get_error_suggestions();

    loop {
        println!("Enter a Rust error message (or 'quit' to exit):");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();

        if input == "quit" {
            break;
        }

        let error_code = _extract_error_codes(input);

        if error_code.is_empty() {
            println!("Couldn't find any error codes in you message. Try again!!");
        } else {
            for code in &error_code {
                match esuggestions.get(code) {
                    Some(suggestion) => println!("Suggestion for {}: {}", code, suggestion),
                    None => {
                        println!(
                            "NO suggestion found for {}. Would you like to add one? (yes)",
                            code
                        );
                        let mut add_input = String::new();
                        io::stdin()
                            .read_line(&mut add_input)
                            .expect("failed to read input");
                        let add_input = add_input.trim().to_lowercase();

                        if add_input == "yes" {
                            println!("Enter your suggestion for {}:", code);
                            let mut suggestion = String::new();
                            io::stdin()
                                .read_line(&mut suggestion)
                                .expect("Failed to read input");
                            let suggestion = suggestion.trim().to_string();

                            esuggestions.insert(code.clone(), suggestion);
                            save_error_suggestions(&esuggestions);
                            println!("suggestion added for{}!", code);
                        } else {
                            println!("NO suggestion available for {}.", code)
                        }
                    }
                }
            }
        }
    }
}
