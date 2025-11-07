//! Authors: RustedAperture
//! Date: 2025/11/07

const DESCRIPTION: &str = r#"
3 Strikes

Given an integer between 1 and 10,000, return a count of how many numbers from 1 up to that integer whose square contains at least one digit 3.

https://www.freecodecamp.org/learn/daily-coding-challenge/2025-08-20
"#;

fn squares_with_three(i: i32) -> i32 {
    let mut remainder = i;
    let mut counter = 0;

    while remainder > 0 {
        let sq = remainder * remainder;
        let digits: Vec<char> = sq.to_string().chars().collect();
        if digits.contains(&'3') {
            counter += 1;
        }
        remainder -= 1;
    }

    return counter;
}

fn main() {
    println!("{}", DESCRIPTION);

    println!(
        "Example: squares_with_three(100) = {:?}",
        squares_with_three(100)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_squares_with_three_one() {
        assert_eq!(squares_with_three(1), 0);
    }

    #[test]
    fn test_squares_with_three_ten() {
        assert_eq!(squares_with_three(10), 1);
    }

    #[test]
    fn test_squares_with_three_hundred() {
        assert_eq!(squares_with_three(100), 19);
    }

    #[test]
    fn test_squares_with_three_thousand() {
        assert_eq!(squares_with_three(1000), 326);
    }

    #[test]
    fn test_squares_with_three_ten_thousand() {
        assert_eq!(squares_with_three(10000), 4531);
    }
}
