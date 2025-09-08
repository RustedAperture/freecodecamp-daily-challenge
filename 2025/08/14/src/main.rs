//! Authors: RustedAperture
//! Date: 2025/09/06

const DESCRIPTION: &str = r#"
S  P  A  C  E  J  A  M

Given a string, remove all spaces from the string, insert two spaces between every character, convert all alphabetical letters to uppercase, and return the result.

- Non-alphabetical characters should remain unchanged (except for spaces).

https://www.freecodecamp.org/learn/daily-coding-challenge/2025-08-14
"#;

fn space_jam(s: &str) -> String {
    let s_upper: String = s.to_uppercase();
    let s_upper_no_whitespace: String = s_upper.chars().filter(|c| !c.is_whitespace()).collect();
    return s_upper_no_whitespace.chars().map(|c| c.to_string()).collect::<Vec<String>>().join("  ");
}

fn main() {
    println!("{}", DESCRIPTION);
    
    println!("Example: space_jam(\"freeCodeCamp\") = \"{}\"", space_jam("freeCodeCamp"));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_freecodecamp() {
        assert_eq!(space_jam("freeCodeCamp"), "F  R  E  E  C  O  D  E  C  A  M  P");
    }

    #[test]
    fn test_freecodecamp_with_spaces() {
        assert_eq!(space_jam("   free   Code   Camp   "), "F  R  E  E  C  O  D  E  C  A  M  P");
    }

    #[test]
    fn test_helloworld() {
        assert_eq!(space_jam("Hello World?!"), "H  E  L  L  O  W  O  R  L  D  ?  !");
    }

    #[test]
    fn test_catsanddogs() {
        assert_eq!(space_jam("C@t$ & D0g$"), "C  @  T  $  &  D  0  G  $");
    }

    #[test]
    fn test_allyourbase() {
        assert_eq!(space_jam("allyourbase"), "A  L  L  Y  O  U  R  B  A  S  E");
    }
}