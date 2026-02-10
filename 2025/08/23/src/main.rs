//! Authors: RustedAperture
//! Date: 2026/02/09

const DESCRIPTION: &str = r#"
Unnatural Prime

Given an integer, determine if that number is a prime number or a negative prime number.

- A prime number is a positive integer greater than 1 that is only divisible by 1 and itself.
- A negative prime number is the negative version of a positive prime number.
- 1 and 0 are not considered prime numbers.

https://www.freecodecamp.org/learn/daily-coding-challenge/2025-08-23
"#;

fn is_unnatural_prime(n: i64) -> bool {
    if n.abs() <= 1 {
        return false;
    }

    let base_primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31];
    let square: f64 = (n as f64).sqrt();

    for i in base_primes {
        if i > square.floor() as i64 {
            break;
        }

        if n.abs() % i == 0 {
            return false;
        }
    }

    true
}

fn main() {
    println!("{}", DESCRIPTION);

    println!(
        "Example: is_unnatural_prime(99) = {:?}",
        is_unnatural_prime(99)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unnatural_primes() {
        assert_eq!(is_unnatural_prime(1), false);
        assert_eq!(is_unnatural_prime(-1), false);
        assert_eq!(is_unnatural_prime(19), true);
        assert_eq!(is_unnatural_prime(-23), true);
        assert_eq!(is_unnatural_prime(0), false);
        assert_eq!(is_unnatural_prime(97), true);
        assert_eq!(is_unnatural_prime(-61), true);
        assert_eq!(is_unnatural_prime(99), false);
        assert_eq!(is_unnatural_prime(44), false);
    }
}