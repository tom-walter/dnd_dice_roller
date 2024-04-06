// module for rolling dice
use rand::Rng;


// Function to roll a dice with specified number of sides
pub fn roll_dice(dice_type: &str) -> Option<u32> {
    // Parse the input string to determine the type of dice
    let sides: u32 = match &dice_type[1..] {
        "2" => 2,
        "4" => 4,
        "6" => 6,
        _ => return None,
    };

    // Generate a random number within the appropriate range
    let result = rand::thread_rng().gen_range(1..=sides);

    Some(result)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll_dice_negative() {
        // Test negative case (invalid input)
        assert_eq!(roll_dice("d13"), None);
        assert_eq!(roll_dice("invalid"), None);
    }

    #[test]
    fn test_roll_dice_positive() {
        // Test positive cases (valid input)
        assert!(match roll_dice("d2") {
            Some(num) => num >= 1 && num <= 2,
            None => false,
        });
        assert!(match roll_dice("d4") {
            Some(num) => num >= 1 && num <= 4,
            None => false,
        });
        assert!(match roll_dice("d6") {
            Some(num) => num >= 1 && num <= 6,
            None => false,
        });
    }
}
