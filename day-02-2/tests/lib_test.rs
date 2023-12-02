mod parse_line {
    use aoc2023::{parse_line, Color, Game, Round};

    #[test]
    fn puzzle_example() {
        // When
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        // When
        let result = parse_line(input);

        // Then
        let rounds = vec![
            Round::new_from_list(&[(Color::Blue, 3), (Color::Red, 4)]),
            Round::new_from_list(&[(Color::Red, 1), (Color::Green, 2), (Color::Blue, 6)]),
            Round::new_from_list(&[(Color::Green, 2)]),
        ];
        let expected = Game { id: 1, rounds };
        assert_eq!(result, expected);
    }
}

mod process_lines {
    use aoc2023::process_lines;

    #[test]
    fn puzzle_example() {
        // When
        let input = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
        ];

        // When
        let result = process_lines(input.into_iter());

        // Then
        assert_eq!(result, 2286);
    }
}
