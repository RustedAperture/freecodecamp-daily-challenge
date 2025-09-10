//! Authors: RustedAperture
//! Date: 2025/09/10

const DESCRIPTION: &str = r#"
Anagram Checker

Given two strings, determine if they are anagrams of each other (contain the same characters in any order).

- Ignore casing and white space.

https://www.freecodecamp.org/learn/daily-coding-challenge/2025-08-15
"#;

fn are_anagrams(s1: &str, s2: &str) -> bool {
    let mut char_array_one: Vec<char> = s1.to_lowercase().chars().filter(|c| !c.is_whitespace()).collect();
    let mut char_array_two: Vec<char> = s2.to_lowercase().chars().filter(|c| !c.is_whitespace()).collect();

    char_array_one.sort_unstable();
    char_array_two.sort_unstable();

    return char_array_one == char_array_two;
}

fn main() {
    println!("{}", DESCRIPTION);
    
    println!("Example: are_anagrams(\"School master\", \"The classroom\") = \"{}\"", are_anagrams("School master", "The classroom"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_listen_silent() {
        assert_eq!(are_anagrams("listen", "silent"), true);
    }

    #[test]
    fn test_school_classroom() {
        assert_eq!(are_anagrams("School master", "The classroom"), true);
    }

    #[test]
    fn test_gentleman_elegant() {
        assert_eq!(are_anagrams("Hello", "World"), false);
    }

    #[test]
    fn test_apple_banana() {
        assert_eq!(are_anagrams("apple", "banana"), false);
    }

    #[test]
    fn test_cat_dog() {
        assert_eq!(are_anagrams("cat", "dog"), false);
    }
}