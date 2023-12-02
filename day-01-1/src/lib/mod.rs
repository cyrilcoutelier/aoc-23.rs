const RADIX: u32 = 10;

pub fn extract_numbers(line: &str) -> (i32, i32) {
    let chars = line.chars();
    let mut digits = chars
        .filter_map(|c| c.to_digit(RADIX))
        .map(|digit| i32::try_from(digit).unwrap());
    let first_number = digits.next().unwrap();
    let last_number = digits.last().unwrap_or(first_number);
    (first_number, last_number)
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
