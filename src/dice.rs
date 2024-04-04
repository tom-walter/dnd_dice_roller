// module for rolling dice
use rand::Rng;

pub fn roll_d2() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..3)
}

pub fn roll_d4() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..5)
}

pub fn roll_d6() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..7)
}

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
    fn test_roll_d2() {
        let result = roll_d2();
        assert!(result == 1 || result == 2, "Invalid result: {}", result);
    }

    #[test]
    fn test_roll_d4() {
        let result = roll_d4();
        assert!(result >= 1 && result <= 4, "Invalid result: {}", result);
    }

    #[test]
    fn test_roll_d6() {
        let result = roll_d6();
        assert!(result >= 1 && result <= 6, "Invalid result: {}", result);
    }

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
