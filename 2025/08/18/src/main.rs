//! Authors: RustedAperture
//! Date: 2025/09/011

const DESCRIPTION: &str = r#"
Factorializer

Given an integer from zero to 20, return the factorial of that number. The factorial of a number is the product of all the numbers between 1 and the given number.

- The factorial of zero is 1.

https://www.freecodecamp.org/learn/daily-coding-challenge/2025-08-18
"#;

fn factorial(i: i64) -> i64 {
    if i <= 1 {
        return 1;
    } else {
        let mut current = i;
        let mut return_result = i;

        while current > 1 {
            return_result *= current - 1;
            current -= 1;
        }

        return return_result;
    }
}

fn main() {
    println!("{}", DESCRIPTION);
    
    println!("Example: factorial(20) = {:?}", factorial(20));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        assert_eq!(factorial(0), 1)
    }

    #[test]
    fn test_five() {
        assert_eq!(factorial(5), 120)
    }

    #[test]
    fn test_twenty() {
        assert_eq!(factorial(20), 2432902008176640000)
    }
}