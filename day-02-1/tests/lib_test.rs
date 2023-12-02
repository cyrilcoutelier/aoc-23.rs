mod process_lines {
    use aoc2023::process_lines;

    #[test]
    fn puzzle_example() {
        // When
        let input = vec!["two1nine".to_string()];

        // When
        let result = process_lines(input.into_iter());

        // Then
        assert_eq!(result, 0);
    }
}
