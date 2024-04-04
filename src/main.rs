use std::io::{self, Write};
mod dice;


fn main() {
    loop {
        // Print available dice options
        println!("Available dice: d2, d4, d6 (Type 'done' or 'exit' to quit)");

        // Prompt the user for input
        print!("Enter the type of dice to roll: ");
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Trim whitespace and convert input to lowercase
        let input = input.trim().to_lowercase();

        // Check if the user wants to exit
        if input == "done" || input == "exit" {
            println!("Goodbye!");
            break;
        }

        // Roll the dice and print the result
        match dice::roll_dice(&input) {
            Some(result) => println!("\nRolling {}... Result: {}\n", input, result),
            None => println!("Invalid input. Please enter a valid dice type (e.g., d2, d4, d6)."),
        }
    }
}
