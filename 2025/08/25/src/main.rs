//! Authors: RustedAperture
//! Date: 2026/02/11

const DESCRIPTION: &str = r#"
camelCase

Given a string, return its camel case version using the following rules:

- Words in the string argument are separated by one or more characters from the following set: space ( ), dash (-), or underscore (_). Treat any sequence of these as a word break.
- The first word should be all lowercase.
- Each subsequent word should start with an uppercase letter, with the rest of it lowercase.
- All spaces and separators should be removed.

https://www.freecodecamp.org/learn/daily-coding-challenge/2025-08-25
"#;

fn main() {
    println!("{}", DESCRIPTION);
}

fn camel_case(s: &str) -> String {
    let mut capital: bool = false;
    let mut return_string: Vec<char> = Vec::new();

    for c in s.chars() {
        if c == ' ' || c == '-' || c == '_' {
            capital = true;
            continue;
        }

        if capital {
            return_string.push(c.to_uppercase().next().unwrap());
        } else {
            return_string.push(c.to_lowercase().next().unwrap());
        }

        capital = false;
    }

    return_string.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camel_case() {
        assert_eq!(camel_case("hello world"), "helloWorld");
        assert_eq!(camel_case("HELLO WORLD"), "helloWorld");
        assert_eq!(camel_case("secret agent-X"), "secretAgentX");
        assert_eq!(camel_case("FREE cODE cAMP"), "freeCodeCamp");
        assert_eq!(
            camel_case(
                "ye old-_-sea  faring_buccaneer_-_with a - peg__leg----and a_parrot_ _named- _squawk"
            ),
            "yeOldSeaFaringBuccaneerWithAPegLegAndAParrotNamedSquawk"
        );
    }
}
