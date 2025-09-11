//! Authors: RustedAperture
//! Date: 2025/09/11

const DESCRIPTION: &str = r#"
Sum of Squares

Given a positive integer up to 1,000, return the sum of all the integers squared from 1 up to the number.

https://www.freecodecamp.org/learn/daily-coding-challenge/2025-08-19
"#;

fn sum_of_squares(i: i32) -> i32 {
    let mut counter = i;
    let mut return_result = 0;

    while counter > 0 {
        return_result += counter.pow(2);
        counter -= 1;
    }

    return return_result;
}

fn main() {
    println!("{}", DESCRIPTION);
    
    println!("Example: sum_of_squares(25) = {:?}", sum_of_squares(25));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_five() {
        assert_eq!(sum_of_squares(5), 55);
    }

    #[test]
    fn test_ten() {
        assert_eq!(sum_of_squares(10), 385);
    }

    #[test]
    fn test_twenty_five() {
        assert_eq!(sum_of_squares(25), 5525);
    }

    #[test]
    fn test_five_hundred() {
        assert_eq!(sum_of_squares(500), 41791750);
    }

    #[test]
    fn test_thousand() {
        assert_eq!(sum_of_squares(1000), 333833500);
    }
}
