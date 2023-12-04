use std::collections::HashSet;

const RADIX: u32 = 10;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct PartNumber {
    pub value: usize,
    pub start: isize,
    pub end: isize,
}

impl PartNumber {
    fn new(digit: usize, start: isize) -> Self {
        let end = start;
        PartNumber {
            value: digit,
            start,
            end,
        }
    }

    fn add_digit(&mut self, digit: usize) {
        self.end += 1;
        self.value *= 10;
        self.value += digit;
    }
}

pub type LineSymbols = HashSet<isize>;

#[derive(Eq, PartialEq, Debug)]
pub struct LineData {
    pub part_numbers: Vec<PartNumber>,
    pub line_symbols: LineSymbols,
}

fn is_matching(part_number: &PartNumber, line_symbols: &LineSymbols) -> bool {
    (part_number.start - 1..=part_number.end + 1).any(|pos| line_symbols.contains(&pos))
}

enum Entry {
    Void,
    Symbol(char),
    Number(usize),
}

impl From<char> for Entry {
    fn from(c: char) -> Entry {
        match c {
            '.' => Entry::Void,
            '0'..='9' => Entry::Number(c.to_digit(RADIX).unwrap() as usize),
            _ => Entry::Symbol(c),
        }
    }
}

pub fn parse_line(line: &str) -> LineData {
    let mut part_numbers = Vec::new();
    let mut line_symbols = LineSymbols::new();
    let mut current_part_number: Option<PartNumber> = None;

    line.chars()
        .map(Entry::from)
        .enumerate()
        .for_each(|(index, entry)| match entry {
            Entry::Number(digit) => {
                if let Some(part_number) = current_part_number.as_mut() {
                    part_number.add_digit(digit);
                } else {
                    current_part_number = Some(PartNumber::new(digit, index as isize));
                }
            }
            Entry::Symbol(_) => {
                if let Some(part_number) = current_part_number.take() {
                    part_numbers.push(part_number);
                }
                line_symbols.insert(index as isize);
            }
            Entry::Void => {
                if let Some(part_number) = current_part_number.take() {
                    part_numbers.push(part_number);
                }
            }
        });

    if let Some(part_number) = current_part_number.take() {
        part_numbers.push(part_number);
    }

    LineData {
        part_numbers,
        line_symbols,
    }
}

pub fn process_lines<T>(lines: T) -> i32
where
    T: Iterator<Item = String>,
{
    let mut parts = Vec::new();
    let mut previous_line_data: Option<LineData> = None;
    lines.map(|line| parse_line(&line)).for_each(|line_data| {
        parts.extend(
            line_data
                .part_numbers
                .iter()
                .filter(|part_number| is_matching(part_number, &line_data.line_symbols))
                .cloned(),
        );

        if let Some(previous_line_data) = previous_line_data.take() {
            parts.extend(
                line_data
                    .part_numbers
                    .iter()
                    .filter(|part_number| {
                        is_matching(part_number, &previous_line_data.line_symbols)
                    })
                    .cloned(),
            );
            parts.extend(
                previous_line_data
                    .part_numbers
                    .iter()
                    .filter(|part_number| is_matching(part_number, &line_data.line_symbols))
                    .cloned(),
            );
        }
        previous_line_data = Some(line_data);
    });

    let result: usize = parts.iter().map(|part| part.value).sum();
    result as i32
}
