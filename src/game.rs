use chrono::{Local, DateTime, TimeZone};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

const DT_FMT: &str = "%Y-%m-%d %H:%M:%S";

// Roll struct to represent a single dice roll
pub struct Roll {
    dice_type: String,
    rolled: u32,
    timestamp: DateTime<Local>,
}

impl Roll {
    // Constructor for Roll
    pub fn new(dice_type: &str, rolled: u32) -> Roll {
        Roll {
            dice_type: dice_type.to_string(),
            rolled,
            timestamp: Local::now(),
        }
    }

    // Convert Roll to string representation
    pub fn to_string(&self) -> String {
        format!(
            "{};{};{}",
            self.timestamp.format(DT_FMT),
            self.dice_type,
            self.rolled
        )
    }
}

// Game struct to represent the game and its rolls
pub struct Game {
    name: String,
    filepath: String,
    rolls: Vec<Roll>,
}

impl Game {
    // Constructor for Game
    pub fn new(name: &str) -> Game {
        let filepath = format!("{}.txt", name);
        let rolls = Game::load(&filepath);
        Game {
            name: name.to_string(),
            filepath,
            rolls,
        }
    }

    // Load rolls from a save file
    pub fn load(filepath: &str) -> Vec<Roll> {
        if let Ok(file) = File::open(filepath) {
            let reader = BufReader::new(file);
            let mut rolls = Vec::new();

            for line in reader.lines() {
                if let Ok(line) = line {
                    let parts: Vec<&str> = line.split(';').collect();
                    if parts.len() == 3 {
                        if let Ok(timestamp) = Local.datetime_from_str(parts[0], DT_FMT) {
                            let roll = Roll {
                                timestamp,
                                dice_type: parts[1].to_string(),
                                rolled: parts[2].parse().unwrap_or(0),
                            };
                            rolls.push(roll);
                        }
                    }
                }
            }
            rolls
        } else {
            Vec::new()
        }
    }

    // Save rolls to the save file
    pub fn save(&self) -> io::Result<()> {
        let mut file = File::create(&self.filepath)?;
        writeln!(file, "{}", self.name)?;

        for roll in &self.rolls {
            writeln!(file, "{}", roll.to_string())?;
        }

        Ok(())
    }

    pub fn add_roll(&mut self, dice_type: &str, rolled: u32) {
        let roll = Roll::new(dice_type, rolled);
        self.rolls.push(roll)
    }

    // Display previous 'n' rolls
    pub fn display_previous_rolls(&self, n: usize) {
        if self.rolls.is_empty() {
            println!("No rolls available for {}", self.name);
        } else {
            let start_index = if self.rolls.len() < n { 0 } else { self.rolls.len() - n };
            for roll in &self.rolls[start_index..] {
                println!(
                    "Timestamp: {}, Dice type: {}, Rolled: {}",
                    roll.timestamp.format(DT_FMT),
                    roll.dice_type,
                    roll.rolled
                );
            }
        }
    }
}