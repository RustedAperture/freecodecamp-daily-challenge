//! Authors: RustedAperture
//! Date: 2025/09/11

const DESCRIPTION: &str = r#"
Targeted Sum

Given an array of numbers and an integer target, find two unique numbers in the array that add up to the target value. Return an array with the indices of those two numbers, or "Target not found" if no two numbers sum up to the target.

- The returned array should have the indices in ascending order.

https://www.freecodecamp.org/learn/daily-coding-challenge/2025-08-17
"#;

fn find_target(v: &[i8], i: i8) -> Result<Vec<i8>, String> {
    for (index1, value1) in v.iter().enumerate() {
        for (index2, value2) in v.iter().enumerate().skip(index1 + 1) {
            if value1 + value2 == i {
                return Ok(vec![index1 as i8, index2 as i8])
            }
        }
    }
    Err("Target not found".to_string())
}

fn main() {
    println!("{}", DESCRIPTION);
    
    println!("Example: find_target(&[2, 7, 11, 15], 9) = {:?}", find_target(&[2, 7, 11, 15], 9));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nine() {
        match find_target(&[2, 7, 11, 15], 9) {
            Ok(indices) => assert_eq!(indices, vec![0, 1]),
            Err(msg) => panic!("Expected indices but got error: {}", msg),
        }
    }

    #[test]
    fn test_six() {
        match find_target(&[3, 2, 4, 5], 6) {
            Ok(indices) => assert_eq!(indices, vec![1, 2]),
            Err(msg) => panic!("Expected indices but got error: {}", msg),
        }
    }

    #[test]
    fn test_fifteen() {
        match find_target(&[1, 3, 5, 6, 7, 8], 15) {
            Ok(indices) => assert_eq!(indices, vec![4, 5]),
            Err(msg) => panic!("Expected indices but got error: {}", msg),
        }
    }

    #[test]
    fn test_fourteen_error() {
        match find_target(&[1, 3, 5, 7], 14) {
            Err(msg) => assert_eq!(msg, "Target not found"),
            Ok(indices) => panic!("Expected error but got indices: {:?}", indices),
        }
    }
}