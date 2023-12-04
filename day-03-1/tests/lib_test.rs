mod process_lines {
    use aoc2023::process_lines;

    #[test]
    fn puzzle_example() {
        // When
        let input = vec!["Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string()];

        // When
        let result = process_lines(input.into_iter());

        // Then
        assert_eq!(result, 0);
    }
}
