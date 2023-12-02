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
    fn two_digits_91() {
        // When
        let input = "91";
        // When
        let result = extract_numbers(input);
        // Then
        assert_eq!(result, (9, 1));
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

    #[test]
    fn two_str_digits_oneasdfthree() {
        // When
        let input = "oneasdfthree";
        // When
        let result = extract_numbers(input);
        // Then
        assert_eq!(result, (1, 3));
    }

    #[test]
    fn single_str_digits_oneasdfthree() {
        // When
        let input = "rrrrrthreerrrr";
        // When
        let result = extract_numbers(input);
        // Then
        assert_eq!(result, (3, 3));
    }

    #[test]
    fn puzzle_example_two1nine() {
        // When
        let input = "two1nine";
        // When
        let result = extract_numbers(input);
        // Then
        assert_eq!(result, (2, 9));
    }

    #[test]
    fn puzzle_example_eightwothree() {
        // When
        let input = "eightwothree";
        // When
        let result = extract_numbers(input);
        // Then
        assert_eq!(result, (8, 3));
    }

    #[test]
    fn puzzle_example_abcone2threexyz() {
        // When
        let input = "abcone2threexyz";
        // When
        let result = extract_numbers(input);
        // Then
        assert_eq!(result, (1, 3));
    }

    #[test]
    fn puzzle_example_xtwone3four() {
        // When
        let input = "xtwone3four";
        // When
        let result = extract_numbers(input);
        // Then
        assert_eq!(result, (2, 4));
    }

    #[test]
    fn puzzle_example_4nineeightseven2() {
        // When
        let input = "4nineeightseven2";
        // When
        let result = extract_numbers(input);
        // Then
        assert_eq!(result, (4, 2));
    }

    #[test]
    fn puzzle_example_zoneight234() {
        // When
        let input = "zoneight234";
        // When
        let result = extract_numbers(input);
        // Then
        assert_eq!(result, (1, 4));
    }

    #[test]
    fn puzzle_example_7pqrstsixteen() {
        // When
        let input = "7pqrstsixteen";
        // When
        let result = extract_numbers(input);
        // Then
        assert_eq!(result, (7, 6));
    }
}

mod process_lines {
    use aoc2023::process_lines;

    #[test]
    fn puzzle_example() {
        // When
        let input = vec![
            "two1nine".to_string(),
            "eightwothree".to_string(),
            "abcone2threexyz".to_string(),
            "xtwone3four".to_string(),
            "4nineeightseven2".to_string(),
            "zoneight234".to_string(),
            "7pqrstsixteen".to_string(),
        ];

        // When
        let result = process_lines(input.into_iter());

        // Then
        assert_eq!(result, 281);
    }
}
