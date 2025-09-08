//! Authors: RustedAperture
//! Date: 2025/09/08

const DESCRIPTION: &str = r#"
Base Check

Given a string representing a number, and an integer base from 2 to 36, determine whether the number is valid in that base.

- The string may contain integers, and uppercase or lowercase characters.
- The check should be case-insensitive.
- The base can be any number 2-36.
- A number is valid if every character is a valid digit in the given base.
- Example of valid digits for bases: 
    - Base 2: 0-1
    - Base 8: 0-7
    - Base 10: 0-9
    - Base 16: 0-9 and A-F
    - Base 36: 0-9 and A-Z

https://www.freecodecamp.org/learn/daily-coding-challenge/2025-08-12
"#;

fn is_valid_number(s: &str, i: usize) -> bool {
    let chars: Vec<char> = ('0'..='9').chain('a'..='z').collect();
    let spliced_char_array: Vec<char> = chars[..i].to_vec();

    let s_lower = s.to_lowercase();
    
    let mut is_valid: bool = true;

    for c in s_lower.chars().collect::<Vec<char>>() {
        if !spliced_char_array.contains(&c) {
            is_valid = false;
        } 
    }

    return is_valid;
}

fn main() {
    println!("{}", DESCRIPTION);
    
    println!("Example: is_valid_number(\"10101\", 2) = {}", is_valid_number("10101", 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_base_two() {
        assert_eq!(is_valid_number("10101", 2), true);
    }

    #[test]
    fn test_invalid_base_two() {
        assert_eq!(is_valid_number("10201", 2), false);
    }

    #[test]
    fn test_valid_base_eight() {
        assert_eq!(is_valid_number("76543210", 8), true);
    }

    #[test]
    fn test_invalid_base_eight() {
        assert_eq!(is_valid_number("9876543210", 8), false);
    }

    #[test]
    fn test_valid_base_ten() {
        assert_eq!(is_valid_number("9876543210", 10), true);
    }

    #[test]
    fn test_invalid_base_ten() {
        assert_eq!(is_valid_number("ABC", 10), false);
    }

    #[test]
    fn test_valid_base_sixteen() {
        assert_eq!(is_valid_number("ABC", 16), true);
    }

    #[test]
    fn test_valid_base_thirty_two() {
        assert_eq!(is_valid_number("Z", 36), true);
    }

    #[test]
    fn test_valid_base_sixteen_two() {
        assert_eq!(is_valid_number("4B4BA9", 16), true);
    }

    #[test]
    fn test_invalid_base_sixteen() {
        assert_eq!(is_valid_number("5G3F8F", 16), false);
    }

    #[test]
    fn test_valid_base_seventeen() {
        assert_eq!(is_valid_number("5G3F8F", 17), true);
    }

    #[test]
    fn test_invalid_base_ten_two() {
        assert_eq!(is_valid_number("abc", 10), false);
    }

    #[test]
    fn test_valid_base_sixteen_three() {
        assert_eq!(is_valid_number("abc", 16), true);
    }

    #[test]
    fn test_valid_base_sixteen_four() {
        assert_eq!(is_valid_number("AbC", 16), true);
    }

    #[test]
    fn test_valid_base_sixteen_five() {
        assert_eq!(is_valid_number("AbC", 16), true);
    }

    #[test]
    fn test_valid_base_thirty_six() {
        assert_eq!(is_valid_number("z", 36), true);
    }
}