//! Authors: RustedAperture
//! Date: 2025/09/06

const DESCRIPTION: &str = r#"
Fibonacci Sequence

The Fibonacci sequence is a series of numbers where each number is the sum of the two preceding ones. When starting with 0 and 1, the first 10 numbers in the sequence are 0, 1, 1, 2, 3, 5, 8, 13, 21, 34.

Given an array containing the first two numbers of a Fibonacci sequence, and an integer representing the length of the sequence, return an array containing the sequence of the given length.

- Your function should handle sequences of any length greater than or equal to zero.
- If the length is zero, return an empty array.
- Note that the starting numbers are part of the sequence.

https://www.freecodecamp.org/learn/daily-coding-challenge/2025-08-13
"#;

fn fibonacci_sequence(a: [u32; 2], u: usize) -> Vec<u32> {
    match u {
        0 | 1 | 2 => return a[..u].to_vec(),
        _ => {
            let mut ret = Vec::with_capacity(u);

            ret.push(a[0]);
            ret.push(a[1]);

            for i in 2..u {
                let num_1 = ret[i-2];
                let num_2 = ret[i-1];
                ret.push( num_1 + num_2 );
            }

            return ret;
        }
    }
}

fn main() {
    println!("{}", DESCRIPTION);
    
    println!("Example: fibonacci_sequence([0, 1], 20) = {:?}", fibonacci_sequence([0, 1], 20));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_twenty() {
        assert_eq!(fibonacci_sequence([0, 1], 20), [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181]);
    }

    #[test]
    fn test_fib_one() {
        assert_eq!(fibonacci_sequence([21, 32], 1), [21]);
    }

    #[test]
    fn test_fib_zero() {
        assert_eq!(fibonacci_sequence([0, 1], 0), []);
    }

    #[test]
    fn test_fib_two() {
        assert_eq!(fibonacci_sequence([10, 20], 2), [10, 20]);
    }

    #[test]
    fn test_fib_five() {
        assert_eq!(fibonacci_sequence([123456789, 987654321], 5), [123456789, 987654321, 1111111110, 2098765431, 3209876541]);
    }
}