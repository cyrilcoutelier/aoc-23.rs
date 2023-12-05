use std::{cell::RefCell, collections::VecDeque};

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

struct Gear {
    left_gear: usize,
    right_gear: usize,
}

impl Gear {
    fn get_ratio(&self) -> usize {
        self.left_gear * self.right_gear
    }
}

struct Matcher {
    gears: RefCell<Vec<Gear>>,
    lines: VecDeque<Option<LineData>>,
}

impl Matcher {
    fn new() -> Self {
        let gears = RefCell::new(Vec::new());
        let lines = VecDeque::from([None, None, None]);
        Matcher { gears, lines }
    }

    fn add_line(&mut self, line: Option<LineData>) {
        self.lines.push_back(line);
        self.lines.pop_front().unwrap();
    }

    fn is_empty(&self) -> bool {
        self.lines.iter().all(|line| line.is_none())
    }

    fn match_line_if_possible(&self) {
        if let Some(line) = self.lines.get(1).unwrap() {
            line.line_stars.iter().for_each(|&pos| {
                self.try_match_pos(pos);
            });
        }
    }

    fn try_match_pos(&self, pos: isize) {
        let mut matching_numbers = Vec::new();
        self.lines
            .iter()
            .filter_map(|line| line.as_ref())
            .for_each(|line| {
                matching_numbers.extend(line.part_numbers.iter().filter_map(|part_number| {
                    if is_matching(part_number, pos) {
                        Some(part_number.value)
                    } else {
                        None
                    }
                }));
            });

        if let [left_gear, right_gear] = &matching_numbers[..] {
            self.gears.borrow_mut().push(Gear {
                left_gear: *left_gear,
                right_gear: *right_gear,
            })
        }
    }
}

pub type LineStars = Vec<isize>;

#[derive(Eq, PartialEq, Debug)]
pub struct LineData {
    pub part_numbers: Vec<PartNumber>,
    pub line_stars: LineStars,
}

fn is_matching(part_number: &PartNumber, pos: isize) -> bool {
    pos >= part_number.start - 1 && pos <= part_number.end + 1
}

enum Entry {
    Void,
    Star,
    Number(usize),
}

impl From<char> for Entry {
    fn from(c: char) -> Entry {
        match c {
            '*' => Entry::Star,
            '0'..='9' => Entry::Number(c.to_digit(RADIX).unwrap() as usize),
            _ => Entry::Void,
        }
    }
}

pub fn parse_line(line: &str) -> LineData {
    let mut part_numbers = Vec::new();
    let mut line_stars = LineStars::new();
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
            Entry::Star => {
                if let Some(part_number) = current_part_number.take() {
                    part_numbers.push(part_number);
                }
                line_stars.push(index as isize);
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
        line_stars,
    }
}

pub fn process_lines<T>(lines: T) -> i32
where
    T: Iterator<Item = String>,
{
    let mut matcher = Matcher::new();

    lines.map(|line| parse_line(&line)).for_each(|line_data| {
        matcher.add_line(Some(line_data));
        matcher.match_line_if_possible();
    });

    while !matcher.is_empty() {
        matcher.add_line(None);
        matcher.match_line_if_possible();
    }

    let result: usize = matcher
        .gears
        .borrow()
        .iter()
        .map(|gear| gear.get_ratio())
        .sum();
    result as i32
}
