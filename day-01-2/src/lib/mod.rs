use std::cmp::Ordering;

#[derive(Copy, Clone)]
struct Digit {
    value: i32,
    text: &'static str,
}

impl Digit {
    const fn new(value: i32, text: &'static str) -> Self {
        Digit { value, text }
    }
}

const DIGITS: &[Digit] = &[
    Digit::new(1, "1"),
    Digit::new(2, "2"),
    Digit::new(3, "3"),
    Digit::new(4, "4"),
    Digit::new(5, "5"),
    Digit::new(6, "6"),
    Digit::new(7, "7"),
    Digit::new(8, "8"),
    Digit::new(9, "9"),
    Digit::new(1, "one"),
    Digit::new(2, "two"),
    Digit::new(3, "three"),
    Digit::new(4, "four"),
    Digit::new(5, "five"),
    Digit::new(6, "six"),
    Digit::new(7, "seven"),
    Digit::new(8, "eight"),
    Digit::new(9, "nine"),
];

#[derive(Eq)]
struct Match {
    value: i32,
    index: usize,
}

impl Ord for Match {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index.cmp(&other.index)
    }
}

impl PartialOrd for Match {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Match {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

pub fn extract_first_digit(line: &str) -> i32 {
    DIGITS
        .iter()
        .filter_map(|&digit| {
            let Digit { value, text } = digit;
            line.find(text).map(|index| Match { value, index })
        })
        .reduce(|left, right| left.min(right))
        .unwrap()
        .value
}

pub fn extract_last_digit(line: &str) -> i32 {
    DIGITS
        .iter()
        .filter_map(|&digit| {
            let Digit { value, text } = digit;
            line.rfind(text).map(|index| Match { value, index })
        })
        .reduce(|left, right| left.max(right))
        .unwrap()
        .value
}

pub fn extract_numbers(line: &str) -> (i32, i32) {
    let first_digit = extract_first_digit(line);
    let last_digit = extract_last_digit(line);
    (first_digit, last_digit)
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
