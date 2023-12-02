mod extract_numbers {
    use aoc2023::extract_numbers;

    #[test]
    fn only_digits_12345() {
        // When
        let input = "12345";
        // When
        let result = extract_numbers(input);
        // Then
        assert_eq!(result, (1, 5));
    }

    #[test]
    fn one_digit_ab5to() {
        // When
        let input = "ab5to";
        // When
        let result = extract_numbers(input);
        // Then
        assert_eq!(result, (5, 5));
    }

    #[test]
    fn two_digits_90() {
        // When
        let input = "90";
        // When
        let result = extract_numbers(input);
        // Then
        assert_eq!(result, (9, 0));
    }

    #[test]
    fn mix_asdgasd8asd1gas4dg2asdgasdg() {
        // When
        let input = "asdgasd8asd1gas4dg2asdgasdg";
        // When
        let result = extract_numbers(input);
        // Then
        assert_eq!(result, (8, 2));
    }
}

mod process_lines {
    use aoc2023::process_lines;

    #[test]
    fn puzzle_subject_example() {
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
