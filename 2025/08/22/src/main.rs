//! Authors: RustedAperture
//! Date: 2026/02/08

const DESCRIPTION: &str = r#"
Message Decoder

Given a secret message string, and an integer representing the number of letters that were used to
shift the message to encode it, return the decoded string.

- A positive number means the message was shifted forward in the alphabet.
- A negative number means the message was shifted backward in the alphabet.
- Case matters, decoded characters should retain the case of their encoded counterparts.
- Non-alphabetical characters should not get decoded.

https://www.freecodecamp.org/learn/daily-coding-challenge/2025-08-22
"#;

fn decode(input: &str, offset: i8) -> String {
    let words: Vec<&str> = input.split(" ").collect();
    let mut return_string: Vec<String> = Vec::with_capacity(words.len());

    for word in words {
        let chars: Vec<char> = word.chars().collect();
        let mut new_word: Vec<char> = vec![];

        for c in chars {
            if !c.is_alphabetic() {
                new_word.push(c);
                continue;
            }

            let mut u: u8 = c as u8;

            let upper = if c.is_ascii_uppercase() {90} else {122};
            let lower = if c.is_ascii_uppercase() {64} else {96};

            u = if offset.is_positive() {u - offset.abs() as u8} else {u + offset.abs() as u8};

            if offset.is_positive() && u <= lower {
                u = upper - (lower - u);
            } else if offset.is_negative() && u >= upper {
                u = (u - upper) + lower;
            }

            new_word.push(u as char);
        }

        return_string.push(new_word.iter().collect());
    }

    return return_string.join(" ");
}

fn main() {
    println!("{}", DESCRIPTION);

    println!(
        "Example: decode(\"Xlmw mw e wigvix qiwweki.\", 4) = {:?}",
        decode("Xlmw mw e wigvix qiwweki.", 4)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        assert_eq!(decode("Xlmw mw e wigvix qiwweki.", 4), "This is a secret message.");
        assert_eq!(decode("Byffi Qilfx!", 20), "Hello World!");
        assert_eq!(decode("Zqd xnt njzx?", -1), "Are you okay?");
        assert_eq!(decode("oannLxmnLjvy", 9), "freeCodeCamp");
    }
}