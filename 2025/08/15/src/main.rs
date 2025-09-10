//! Authors: RustedAperture
//! Date: 2025/09/10

const DESCRIPTION: &str = r#"
Jbelmud Text

Given a string, return a jumbled version of that string where each word is transformed using the following constraints:

- The first and last letters of the words remain in place
- All letters between the first and last letter are sorted alphabetically.
- The input strings will contain no punctuation, and will be entirely lowercase.

https://www.freecodecamp.org/learn/daily-coding-challenge/2025-08-15
"#;

fn jbelmu(s: &str) -> String {
    
    let words: Vec<&str> = s.split(" ").collect();
    let mut return_string: Vec<String> = Vec::with_capacity(words.len());

    for word in words {
        if word.len() > 2 {
            let mut chars: Vec<char> = word.chars().collect();

            let mut new_word: Vec<char> = vec![chars[0]];

            let last_index = chars.len() - 1;
            let middle_chars= &mut chars[1..last_index];
            middle_chars.sort_unstable();

            new_word.extend_from_slice(middle_chars);

            new_word.push(chars[last_index]);

            return_string.push(new_word.iter().collect());
        } else {
            return_string.push(word.to_string());
        }
    }
    
    return return_string.join(" ");
}

fn main() {
    println!("{}", DESCRIPTION);
    
    println!("Example: jbelmu(\"i love jumbled text\") = \"{}\"", jbelmu("i love jumbled text"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helloworld() {
        assert_eq!(jbelmu("hello world"), "hello wlord");
    }

    #[test]
    fn test_jumbled() {
        assert_eq!(jbelmu("i love jumbled text"), "i love jbelmud text");
    }

    #[test]
    fn test_freecodecamp() {
        assert_eq!(jbelmu("freecodecamp is my favorite place to learn to code"), "faccdeeemorp is my faiortve pacle to laern to cdoe");
    }

    #[test]
    fn test_quickfox() {
        assert_eq!(jbelmu("the quick brown fox jumps over the lazy dog"), "the qciuk borwn fox jmpus oevr the lazy dog");
    }
}