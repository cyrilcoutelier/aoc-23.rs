#![warn(clippy::pedantic)]

const RADIX: u32 = 10;

fn extract_numbers(line: &str) -> (i32, i32) {
    let chars = line.chars();
    let mut digits = chars
        .filter_map(|c| c.to_digit(RADIX))
        .map(|digit| i32::try_from(digit).unwrap());
    let first_number = digits.next().unwrap();
    let last_number = digits.last().unwrap_or(first_number);
    (first_number, last_number)
}

#[cfg(test)]
mod tests_extract_numbers {
    use crate::extract_numbers;

    #[test]
    fn test_12345() {
        // only digits
        // When
        let input = "12345";
        // When
        let result = extract_numbers(input);
        // Then
        assert_eq!(result, (1, 5));
    }

    #[test]
    fn test_ab5to() {
        // one digit
        // When
        let input = "ab5to";
        // When
        let result = extract_numbers(input);
        // Then
        assert_eq!(result, (5, 5));
    }

    #[test]
    fn test_90() {
        // two digits
        // When
        let input = "90";
        // When
        let result = extract_numbers(input);
        // Then
        assert_eq!(result, (9, 0));
    }

    #[test]
    fn test_asdgasd8asd1gas4dg2asdgasdg() {
        // mix of digits and other stuff
        // When
        let input = "asdgasd8asd1gas4dg2asdgasdg";
        // When
        let result = extract_numbers(input);
        // Then
        assert_eq!(result, (8, 2));
    }
}

fn combine_numbers(left: i32, right: i32) -> i32 {
    left * 10 + right
}

pub fn process_lines<T>(lines: T) -> i32
where
    T: Iterator<Item = String>,
{
    lines
        .map(|line| extract_numbers(&line))
        .map(|(left, right)| combine_numbers(left, right))
        .sum()
}

#[cfg(test)]
mod tests_process_lines {
    use crate::process_lines;

    #[test]
    fn test_from_puzzle_subject() {
        // When
        let input = vec![
            "1abc2".to_string(),
            "pqr3stu8vwx".to_string(),
            "a1b2c3d4e5f".to_string(),
            "treb7uchet".to_string(),
        ];

        // When
        let result = process_lines(input.into_iter());

        // Then
        assert_eq!(result, 142);
    }
}
