//! Authors: RustedAperture
//! Date: 2025/09/06

const DESCRIPTION: &str = r#"
Character Battle

Given two strings representing your army and an opposing army, each character from your army battles the character at
the same position from the opposing army using the following rules:

- Characters a-z have a strength of 1-26, respectively.
- Characters A-Z have a strength of 27-52, respectively.
- Digits 0-9 have a strength of their face value.
- All other characters have a value of zero.
- Each character can only fight one battle.

For each battle, the stronger character wins. The army with more victories, wins the war. Return the following values:

- "Opponent retreated" if your army has more characters than the opposing army.
- "We retreated" if the opposing army has more characters than yours.
- "We won" if your army won more battles.
- "We lost" if the opposing army won more battles.
- "It was a tie" if both armies won the same number of battles.

https://www.freecodecamp.org/learn/daily-coding-challenge/2025-08-24
"#;

fn main() {
    println!("{}", DESCRIPTION);

    println!("Example: battle(\"Hello\", \"World\") = \"{}\"", battle("Hello", "World"));
}

fn get_score(c: char) -> i32 {
    let base = if c.is_uppercase() { 38 } else { 94 };
    if c.is_alphabetic() {
        return (c as u32 - base) as i32;
    } else if c.is_numeric() {
        return (c as u32 - 48) as i32;
    } else {
        return 0;
    }
}

fn get_valid_chars(s: &str) -> Vec<char> {
    return s.chars().collect();
}

fn battle(my_army: &str, opposing_army: &str) -> &'static str {
    let my_chars = get_valid_chars(my_army);
    let opposing_chars = get_valid_chars(opposing_army);

    if my_chars.len() > opposing_chars.len() {
        return "Opponent retreated";
    } else if my_chars.len() < opposing_chars.len() {
        return "We retreated";
    }

    let mut my_wins = 0;
    let mut opposing_wins = 0;

    for (c1, c2) in my_chars.iter().zip(opposing_chars.iter()) {
        let score1 = get_score(*c1);
        let score2 = get_score(*c2);

        if score1 > score2 {
            my_wins += 1;
        } else if score1 < score2 {
            opposing_wins += 1;
        }
    }

    if my_wins > opposing_wins {
        return "We won";
    } else if my_wins < opposing_wins {
        return "We lost";
    } else {
        return "It was a tie";
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battle() {
        assert_eq!(battle("Hello", "World"), "We lost");
        assert_eq!(battle("pizza", "salad"), "We won");
        assert_eq!(battle("C@T5", "D0G$"), "We won");
        assert_eq!(battle("kn!ght", "orc"), "Opponent retreated");
        assert_eq!(battle("PC", "Mac"), "We retreated");
        assert_eq!(battle("Wizards", "Dragons"), "It was a tie");
        assert_eq!(battle("Mr. Smith", "Dr. Jones"), "It was a tie");
    }
}