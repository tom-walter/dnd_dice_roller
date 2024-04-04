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
}
