mod parse_line {
    use aoc2023::{parse_line, LineData, LineSymbols, PartNumber};

    #[test]
    fn two_numbers() {
        // When
        let line = "..123..234..";

        // When
        let result = parse_line(line);

        // Then
        assert_eq!(
            result,
            LineData {
                part_numbers: vec![
                    PartNumber {
                        value: 123,
                        start: 2,
                        end: 4
                    },
                    PartNumber {
                        value: 234,
                        start: 7,
                        end: 9
                    },
                ],
                line_symbols: LineSymbols::new()
            }
        );
    }

    #[test]
    fn two_number_and_symbol() {
        // When
        let line = ".12#34.";

        // When
        let result = parse_line(line);

        // Then
        let mut line_symbols = LineSymbols::new();
        line_symbols.insert(3);

        assert_eq!(
            result,
            LineData {
                part_numbers: vec![
                    PartNumber {
                        value: 12,
                        start: 1,
                        end: 2
                    },
                    PartNumber {
                        value: 34,
                        start: 4,
                        end: 5,
                    },
                ],
                line_symbols,
            }
        );
    }

    #[test]
    fn two_numbers_at_end() {
        // When
        let line = ".12.34.";

        // When
        let result = parse_line(line);

        // Then
        assert_eq!(
            result,
            LineData {
                part_numbers: vec![
                    PartNumber {
                        value: 12,
                        start: 1,
                        end: 2
                    },
                    PartNumber {
                        value: 34,
                        start: 4,
                        end: 5,
                    },
                ],
                line_symbols: LineSymbols::new()
            }
        );
    }
}

mod process_lines {
    use aoc2023::process_lines;

    #[test]
    fn puzzle_example() {
        // When
        let input = vec![
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            "......#...".to_string(),
            "......#...".to_string(),
            "617*......".to_string(),
            ".....+.58.".to_string(),
            "..592.....".to_string(),
            "......755.".to_string(),
            "...$.*....".to_string(),
            ".664.598..".to_string(),
        ];

        // When
        let result = process_lines(input.into_iter());

        // Then
        assert_eq!(result, 4361);
    }
}
