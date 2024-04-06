use std::io::{self, Write};

mod dice;
mod game;

const ASCII_ART: &str = include_str!("ascii_art.txt");
const AVAIL_DICE: &str = "(e.g. d2, d4, d6, d8, d10, d12, d20, d100)";

fn main() -> io::Result<()> {
    println!("{}", ASCII_ART);

    // Prompt the player for a game name
    println!("Enter the name of the game:");
    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    let name = name.trim();

    // Create or load the game
    let mut game = game::Game::new(name);

    // Main game loop
    loop {
        // Prompt the player for input
        println!("Enter a command:\npick a die {} to roll,\n'hist' (display previous 5 rolls),\nor 'exit' (save and exit):", AVAIL_DICE);
        let mut input = String::new();
        io::stdout().flush()?; // Ensure prompt is displayed immediately
        io::stdin().read_line(&mut input)?;

        // Trim whitespace and convert input to lowercase
        let input = input.trim().to_lowercase();

        // Match the input command
        match input.as_str() {
            "exit" => {
                // Save game state before exiting
                game.save()?;
                println!("Goodbye!");
                break;
            }
            "hist" => {
                // Display previous 5 rolls
                game.display_previous_rolls(5);
            }
            _ => {
                // Roll the dice and update the game state
                if let Some(roll) = dice::roll_dice(&input) {
                    println!("Rolling {}... Result: {}", input, roll);
                    game.add_roll(&input, roll);
                } else {
                    println!("Invalid input. Please enter a valid dice type {}, 'hist', or 'exit'.", AVAIL_DICE);
                }
            }
        }
    }

    Ok(())
}
