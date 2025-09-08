//! Authors: RustedAperture
//! Date: 2025/09/06

const DESCRIPTION: &str = r#"
Vowel Balance

Given a string, determine whether the number of vowels in the first half of the string is equal to the number of vowels in the second half.

- The string can contain any characters.
- The letters a, e, i, o, and u, in either uppercase or lowercase, are considered vowels.
- If there's an odd number of characters in the string, ignore the center character.

https://www.freecodecamp.org/learn/daily-coding-challenge/2025-08-11
"#;

fn is_balanced(s: &str) -> bool {
    let vowels: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    let mut lower: String = s.to_lowercase();

    let lower_len: usize = lower.len();
    let middle_index: usize = (lower_len - 1) / 2;
    if !(lower.contains(" ")) {
        lower = if lower_len % 2 == 0 {
            format!("{}{}{}", &lower[0..middle_index], " ", &lower[middle_index..])
        } else {
            format!("{}{}{}", &lower[0..middle_index], " ", &lower[middle_index + 1..])
        };
    }

    let lower_split: Vec<&str> = lower.split(" ").collect();

    let mut count: Vec<usize> = vec![0; lower_split.len()];

    for (index, word) in lower_split.iter().enumerate() {
        for v in vowels {
            count[index] += word.chars().filter(|&c| c == v).count();
        }
    }

    return count[0] == count[1];
}

fn main() {
    println!("{}", DESCRIPTION);
    
    println!("Example: is_balanced(\"racecar\") = {}", is_balanced("racecar"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_racecar() {
        assert_eq!(is_balanced("racecar"), true);
    }

    #[test]
    fn test_lorem_ipsum() {
        assert_eq!(is_balanced("Lorem Ipsum"), true);
    }

    #[test]
    fn test_kitty_ipsum() {
        assert_eq!(is_balanced("Kitty Ipsum"), false);
    }

    #[test]
    fn test_string() {
        assert_eq!(is_balanced("string"), false);
    }

    #[test]
    fn test_space() {
        assert_eq!(is_balanced(" "), true);
    }

    #[test]
    fn test_alphabet() {
        assert_eq!(is_balanced("abcdefghijklmnopqrstuvwxyz"), false);
    }

    #[test]
    fn test_random() {
        assert_eq!(is_balanced("123A#b!E&*456-o.U"), true);
    }
}
